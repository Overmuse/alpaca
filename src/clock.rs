use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{alpaca_request, AlpacaConfig};

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
mod tests {
    use super::*;

    #[test]
    fn serde() {
        let json = r#"{
            "timestamp": "2018-04-01T12:00:00.000Z",
            "is_open": true,
            "next_open": "2018-04-01T12:00:00.000Z",
            "next_close": "2018-04-01T12:00:00.000Z"
        }"#;
        let deserialized: Clock = serde_json::from_str(json).unwrap();
        let _serialized = serde_json::to_string(&deserialized).unwrap();
    }
}
