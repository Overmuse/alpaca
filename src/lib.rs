use anyhow::{Result, anyhow};
//use log::{info, error};
use reqwest::{Client, header::{HeaderMap, HeaderValue, HeaderName}, Url, Method};
use serde::{Serialize, Deserialize};

mod utils;
pub mod account;
pub mod orders;
pub mod positions;
pub mod assets;

pub struct AlpacaConfig {
    client: Client,
    url: Url,
}

impl AlpacaConfig {
    pub fn new(url: String, key_id: String, secret_key: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_lowercase(b"apca-api-key-id").unwrap(), HeaderValue::from_str(&key_id)?);
        headers.insert(HeaderName::from_lowercase(b"apca-api-secret-key").unwrap(), HeaderValue::from_str(&secret_key)?);

        let client = Client::builder()
            .default_headers(headers.clone())
            .build()?;

        Ok(AlpacaConfig{ client, url: Url::parse(&url)? })
    }
}

pub async fn alpaca_request<T>(method: Method, endpoint: &str, config: &AlpacaConfig, body: Option<T>) -> Result<String>
    where T: Serialize {
    let response = config.client.request(method, config.url.join(endpoint)?)
        .json(&body)
        .send()
        .await?;

    if response.status().is_success() {
        return Ok(response.text().await?)
    } else {
        Err(anyhow!("Non-successful status: {:?}, {:?}", response.status(), response.text().await?))
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell,
    /// Only used for positions
    Long
}

impl Default for Side {
    fn default() -> Self { Side::Buy }
}
