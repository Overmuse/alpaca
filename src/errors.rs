use thiserror::Error;

#[cfg(feature = "ws")]
use tokio_tungstenite::tungstenite;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing environment variable: {0}")]
    MissingEnv(#[from] std::env::VarError),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Invalid request. Received status {0}. Message: {1}")]
    ClientError(reqwest::StatusCode, String),

    #[error("Server error. Received status {0}. Message: {1}")]
    ServerError(reqwest::StatusCode, String),

    #[cfg(feature = "ws")]
    #[error("Tungstenite error: {0}")]
    Tungstenite(#[from] tungstenite::Error),

    #[cfg(feature = "ws")]
    #[error("Client has not yet been initialized.")]
    UninitializedClient,

    #[cfg(feature = "ws")]
    #[error("WebSocket stream has been closed")]
    StreamClosed,

    #[cfg(feature = "ws")]
    #[error("Failed to connect: {0}")]
    ConnectionFailure(String),
}

pub type Result<T> = std::result::Result<T, Error>;
