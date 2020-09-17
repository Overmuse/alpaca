use anyhow::{anyhow, Result};
//use log::{info, error};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Method, Url,
};
use serde::{Deserialize, Serialize};

pub mod account;
pub mod account_activities;
pub mod assets;
pub mod calendar;
pub mod clock;
pub mod orders;
pub mod positions;
pub mod stream;
mod utils;

pub struct AlpacaConfig {
    client: Client,
    url: Url,
}

impl AlpacaConfig {
    pub fn new(url: String, key_id: String, secret_key: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_lowercase(b"apca-api-key-id").unwrap(),
            HeaderValue::from_str(&key_id)?,
        );
        headers.insert(
            HeaderName::from_lowercase(b"apca-api-secret-key").unwrap(),
            HeaderValue::from_str(&secret_key)?,
        );

        let client = Client::builder().default_headers(headers.clone()).build()?;

        Ok(AlpacaConfig {
            client,
            url: Url::parse(&url)?,
        })
    }
}

pub async fn alpaca_request<T>(
    method: Method,
    endpoint: &str,
    config: &AlpacaConfig,
    body: Option<T>,
) -> Result<String>
where
    T: Serialize,
{
    let response = config
        .client
        .request(method, config.url.join(endpoint)?)
        .json(&body)
        .send()
        .await?;

    if response.status().is_success() {
        return Ok(response.text().await?);
    } else {
        Err(anyhow!(
            "Non-successful status: {:?}, {:?}",
            response.status(),
            response.text().await?
        ))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
    SellShort,
    /// Only used for positions
    Long,
    Short,
}

impl Default for Side {
    fn default() -> Self {
        Side::Buy
    }
}
