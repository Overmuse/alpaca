use crate::errors::Result;
use crate::utils::{hm_from_str, hm_to_string};
use crate::{alpaca_request, AlpacaConfig};
use chrono::{NaiveDate, NaiveTime};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Calendar {
    pub date: NaiveDate,
    #[serde(deserialize_with = "hm_from_str", serialize_with = "hm_to_string")]
    pub open: NaiveTime,
    #[serde(deserialize_with = "hm_from_str", serialize_with = "hm_to_string")]
    pub close: NaiveTime,
}

pub async fn get_calendar(config: &AlpacaConfig) -> Result<Vec<Calendar>> {
    let res = alpaca_request(Method::GET, "calendar", config, None::<Calendar>).await?;
    let calendar: Vec<Calendar> = serde_json::from_str(&res)?;
    Ok(calendar)
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::mock;

    #[tokio::test]
    async fn test_get_calendar() {
        let _m = mock("GET", "/calendar")
            .with_body(
                r#"[
		       {
			  "date": "2018-01-03",
			  "open": "09:30",
			  "close": "16:00"
		       }
		]"#,
            )
            .create();
        let config = AlpacaConfig::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        get_calendar(&config).await.unwrap();
    }
}
