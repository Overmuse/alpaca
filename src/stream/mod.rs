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
        self.inner.send(Message::Text(msg.to_string())).await?;
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
