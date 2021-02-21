use crate::errors::{Error, Result};
use futures::{ready, SinkExt, Stream, StreamExt};
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
                    Message::Text(txt) => {
                        let parsed: Result<AlpacaMessage> =
                            serde_json::from_str(&txt).map_err(Error::from);
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

        self.send_message(&serde_json::to_string(&subscription_message)?)
            .await?;
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
    pub fn new(key_id: String, secret_key: String, events: Vec<String>) -> Self {
        Self {
            url: "wss://alpaca.socket.polygon.io/stocks".to_string(),
            key_id,
            secret_key,
            events,
        }
    }

    pub async fn connect(self) -> Result<WebSocket> {
        let auth_message = AlpacaAction::Authenticate {
            key_id: self.key_id.clone(),
            secret_key: self.secret_key.clone(),
        };
        let (client, _) = connect_async(&self.url).await?;
        let mut ws = WebSocket { inner: client };
        ws.send_message(&serde_json::to_string(&auth_message)?)
            .await?;
        let parsed = ws.read_message().await?;
        if let AlpacaMessage::Authorization { status, action } = parsed {
            if let AuthorizationStatus::Authorized = status {
            } else {
                return Err(Error::ConnectionFailure(action.clone()));
            }
        }
        ws.subscribe(self.events).await?;
        Ok(ws)
    }
}

#[cfg(test)]
mod test {}
