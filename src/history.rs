use crate::Request;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub enum TimeFrame {
    #[serde(rename(serialize = "1Min", deserialize = "1Min"))]
    OneMin,
    #[serde(rename(serialize = "5Min", deserialize = "5Min"))]
    FiveMin,
    #[serde(rename(serialize = "15Min", deserialize = "15Min"))]
    FifeteenMin,
    #[serde(rename(serialize = "1H", deserialize = "1H"))]
    OneHour,
    #[serde(rename(serialize = "1D", deserialize = "1D"))]
    OneDay,
}

#[derive(Deserialize)]
pub struct History {
    pub timestamp: Vec<u64>,
    pub equity: Vec<f64>,
    pub profit_loss: Vec<f64>,
    pub profit_loss_pct: Vec<f64>,
    pub base_value: f64,
    pub timeframe: TimeFrame,
}

pub struct GetHistory;
impl Request for GetHistory {
    type Body = ();
    type Response = History;

    fn endpoint(&self) -> Cow<str> {
        "account/portfolio/history".into()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::Client;
    use mockito::mock;

    #[tokio::test]
    async fn test_get_history() {
        let _m = mock("GET", "/account/portfolio/history")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(
                r#"{
		  "timestamp": [1580826600000, 1580827500000, 1580828400000],
  		  "equity": [27423.73, 27408.19, 27515.97],
  		  "profit_loss": [11.8, -3.74, 104.04],
  		  "profit_loss_pct": [0.000430469507254688, -0.0001364369455197062, 0.0037954277571845543],
  		  "base_value": 27411.93,
  		  "timeframe": "15Min"
                }"#,
            )
            .create();
        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        client.send(GetHistory).await.unwrap();
    }
}
