use crate::errors::{AlpacaError, Result};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Method, Url,
};
use serde::Serialize;
use std::env;

pub mod account;
pub mod account_activities;
pub mod assets;
pub mod calendar;
pub mod clock;
pub mod errors;
pub mod orders;
pub mod positions;
pub mod stream;
mod utils;

/// The main client used for making request to Alpaca.
///
/// `AlpacaConfig` stores an async Reqwest client as well as the associate 
/// base url for the Alpaca server.
pub struct AlpacaConfig {
    /// The underlying Reqwest client used for requests.
    client: Client,
    /// The url to which the request are sent.
    url: Url,
}

impl AlpacaConfig {
    /// Create a new `AlpacaConfig`.
    pub fn new(url: String, key_id: String, secret_key: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_lowercase(b"apca-api-key-id").unwrap(),
            HeaderValue::from_str(&key_id).unwrap(),
        );
        headers.insert(
            HeaderName::from_lowercase(b"apca-api-secret-key").unwrap(),
            HeaderValue::from_str(&secret_key).unwrap(),
        );

        let client = Client::builder().default_headers(headers.clone()).build()?;

        Ok(AlpacaConfig {
            client,
            url: Url::parse(&url)?,
        })
    }

    /// Creates an `AlpacaConfig` from environment variables.
    ///
    /// The three environment variables used to instantiate the struct are:
    /// - `APCA_API_BASE_URL`
    /// - `APCA_API_KEY_ID`
    /// - `APCA_API_SECRET_KEY`
    pub fn from_env() -> Result<Self> {
        let url = env::var("APCA_API_BASE_URL")?;
        let key_id = env::var("APCA_API_KEY_ID")?;
        let secret_key = env::var("APCA_API_SECRET_KEY")?;
        Self::new(url, key_id, secret_key)
    }
}

/// Make a request to Alpaca.
///
/// This is a low-level function ment to be used by the higher level
/// conveniance functions and not by end-users directly.
async fn alpaca_request<T>(
    method: Method,
    endpoint: &str,
    config: &AlpacaConfig,
    body: Option<T>,
) -> Result<String>
where
    T: Serialize,
{
    let mut url = config.url.as_str().to_string().clone();
    url.push_str(endpoint);
    let response = config
        .client
        .request(method, &url)
        .json(&body)
        .send()
        .await?;

    if response.status().is_success() {
        return Ok(response.text().await?);
    } else if response.status().is_client_error() {
        Err(AlpacaError::ClientError(
            response.status(),
            response.text().await?,
        ))
    } else {
        Err(AlpacaError::ServerError(
            response.status(),
            response.text().await?,
        ))
    }
}
