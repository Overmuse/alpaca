use crate::Result;
use crate::{alpaca_request, AlpacaConfig};
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Clock {
    pub timestamp: DateTime<Utc>,
    pub is_open: bool,
    pub next_open: DateTime<Utc>,
    pub next_close: DateTime<Utc>,
}

pub async fn get_clock(config: &AlpacaConfig) -> Result<Clock> {
    let res = alpaca_request(Method::GET, "clock", config, None::<Clock>).await?;
    let clock: Clock = serde_json::from_str(&res)?;
    Ok(clock)
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::mock;

    #[tokio::test]
    async fn test_get_clock() {
        let _m = mock("GET", "/clock")
            .with_body(
                r#"{
                    "timestamp": "2018-04-01T12:00:00.000Z",
                    "is_open": true,
                    "next_open": "2018-04-01T12:00:00.000Z",
                    "next_close": "2018-04-01T12:00:00.000Z"
                }"#,
            )
            .create();
        let config = AlpacaConfig::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        get_clock(&config).await.unwrap();
    }
}
