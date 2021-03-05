use crate::errors::{Error, Result};
use futures::{ready, SinkExt, Stream, StreamExt};
use log::{debug, info};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub mod types;
pub use types::*;

pub struct WebSocket {
    inner: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Stream for WebSocket {
    type Item = Result<AlpacaMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match ready!(Pin::new(&mut self.inner).poll_next(cx)) {
            Some(Ok(item)) => {
                match item {
                    Message::Binary(bits) => {
                        let parsed: Result<AlpacaMessage> =
                            serde_json::from_slice(&bits).map_err(Error::from);
                        Poll::Ready(Some(parsed))
                    }
                    _ => {
                        // Non Text message received, immediately schedule re-poll
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                }
            }
            Some(Err(e)) => Poll::Ready(Some(Err(Error::Tungstenite(e)))),
            None => Poll::Ready(None),
        }
    }
}

impl WebSocket {
    async fn send_message(&mut self, msg: &str) -> Result<()> {
        self.inner.send(Message::text(msg)).await?;
        Ok(())
    }

    async fn read_message(&mut self) -> Result<AlpacaMessage> {
        let resp = self.inner.next().await.ok_or(Error::StreamClosed)??;
        let parsed: AlpacaMessage = serde_json::from_str(resp.to_text()?)?;
        Ok(parsed)
    }

    pub async fn subscribe(&mut self, events: Vec<String>) -> Result<()> {
        let subscription_message = AlpacaAction::Listen { streams: events };

        debug!("subscription message: {:?}", &subscription_message);
        self.send_message(&serde_json::to_string(&subscription_message)?)
            .await?;
        let parsed = self.read_message().await?;
        debug!("Subscription reply: {:?}", &parsed);
        Ok(())
    }
}

pub struct Connection {
    url: String,
    key_id: String,
    secret_key: String,
    events: Vec<String>,
}

impl Connection {
    pub fn new(url: String, key_id: String, secret_key: String, events: Vec<String>) -> Self {
        Self {
            url,
            key_id,
            secret_key,
            events,
        }
    }

    pub async fn connect(self) -> Result<WebSocket> {
        let (client, _) = connect_async(&self.url).await?;
        let mut ws = WebSocket { inner: client };
        let auth_message = AlpacaAction::Authenticate {
            key_id: self.key_id.clone(),
            secret_key: self.secret_key.clone(),
        };
        ws.send_message(&serde_json::to_string(&auth_message)?)
            .await?;
        let parsed = ws.read_message().await?;
        debug!("{:?}", &parsed);
        if let AlpacaMessage::Authorization { status, action } = parsed {
            if let AuthorizationStatus::Authorized = status {
                info!("Authorization successful");
            } else {
                return Err(Error::ConnectionFailure(action));
            }
        }
        ws.subscribe(self.events).await?;
        Ok(ws)
    }
}

#[cfg(test)]
mod test {
    use super::Connection;
    use futures::{SinkExt, StreamExt};
    use tokio::{
        io::{AsyncRead, AsyncWrite},
        net::TcpListener,
    };
    use tokio_tungstenite::tungstenite::Message;
    use tokio_tungstenite::{accept_async, WebSocketStream};

    async fn run_connection<S>(connection: WebSocketStream<S>)
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let mut connection = connection;
        let auth_request = connection.next().await.unwrap().unwrap();
        assert_eq!(
            auth_request,
            Message::text(
                r#"{"action":"authenticate","data":{"key_id":"key","secret_key":"secret"}}"#
            )
        );
        let auth_response = Message::text(
            r#"{"stream":"authorization","data":{"status":"authorized","action":"authenticate"}}"#,
        );
        connection
            .send(auth_response)
            .await
            .expect("Failed to send auth_response");
        let subscription_request = connection.next().await.unwrap().unwrap();
        assert_eq!(
            subscription_request,
            Message::text(
                r#"{"action":"listen","data":{"streams":["account_updates","trade_updates"]}}"#
            )
        );
        let subscription_response =
            r#"{"stream":"listening","data":{"streams":["account_updates","trade_updates"]}}"#;
        connection
            .send(Message::text(subscription_response))
            .await
            .expect("Failed to send subscription response");
        // TODO: Send account and trade update messages
        //let account_update_message =
        //    r#"{"stream":"listening","data":{"streams":["account_updates","trade_updates"]}}"#;
        //connection
        //    .send(Message::text(subscription_response))
        //    .await
        //    .expect("Failed to send subscription response");
        //let trade_update_message =
        //    r#"{"stream":"listening","data":{"streams":["account_updates","trade_updates"]}}"#;
        //connection
        //    .send(Message::text(subscription_response))
        //    .await
        //    .expect("Failed to send subscription response");
    }

    #[tokio::test]
    async fn test_connection() {
        let (con_tx, con_rx) = futures_channel::oneshot::channel();
        tokio::spawn(async move {
            let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
            // Send message when server is ready to start the test
            con_tx.send(()).unwrap();
            let (connection, _) = listener.accept().await.expect("No connections to accept");
            let stream = accept_async(connection).await;
            let stream = stream.expect("Failed to handshake with connection");
            run_connection(stream).await;
        });

        con_rx.await.expect("Server not ready");
        let connection = Connection::new(
            "ws://localhost:12345".into(),
            "key".into(),
            "secret".into(),
            vec!["account_updates".into(), "trade_updates".into()],
        );

        let _ws = connection.connect().await.unwrap();
        //let account_update_message = ws.next().await.unwrap();
        //let trade_update_message = ws.next().await.unwrap();
    }
}
