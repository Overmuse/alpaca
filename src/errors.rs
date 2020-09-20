use thiserror::Error;
use url;

#[derive(Error, Debug)]
pub enum AlpacaError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(#[from] std::env::VarError),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Invalid url: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Invalid request. Received status {0}. Message: {1}")]
    ClientError(reqwest::StatusCode, String),

    #[error("Server error. Received status {0}. Message: {1}")]
    ServerError(reqwest::StatusCode, String),
    //#[error("Generic error")]
    //Unknown,
}

pub type Result<T> = std::result::Result<T, AlpacaError>;
