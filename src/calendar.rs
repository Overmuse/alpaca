use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::utils::{hm_from_str, hm_to_string};
use crate::{alpaca_request, AlpacaConfig};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Calendar {
    date: NaiveDate,
    #[serde(deserialize_with = "hm_from_str", serialize_with = "hm_to_string")]
    open: NaiveTime,
    #[serde(deserialize_with = "hm_from_str", serialize_with = "hm_to_string")]
    close: NaiveTime,
}

pub async fn get_calendar(config: &AlpacaConfig) -> Result<Vec<Calendar>> {
    let res = alpaca_request(Method::GET, "calendar", config, None::<Calendar>).await?;
    let calendar: Vec<Calendar> = serde_json::from_str(&res)?;
    Ok(calendar)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() {
        let json = r#"{
            "date": "2018-01-03",
            "open": "09:30",
            "close": "16:00"
        }"#;
        let deserialized: Calendar = serde_json::from_str(json).unwrap();
        let _serialized = serde_json::to_string(&deserialized).unwrap();
    }
}
