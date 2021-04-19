use crate::Request;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Side {
    Long,
    Short,
}

impl Default for Side {
    fn default() -> Side {
        Side::Long
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Position {
    pub asset_id: Uuid,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: String,
    pub avg_entry_price: Decimal,
    #[serde(
        deserialize_with = "crate::utils::from_str",
        serialize_with = "crate::utils::to_string"
    )]
    pub qty: i32,
    pub side: Side,
    pub market_value: Decimal,
    pub cost_basis: Decimal,
    pub unrealized_pl: Decimal,
    pub unrealized_plpc: Decimal,
    pub unrealized_intraday_pl: Decimal,
    pub unrealized_intraday_plpc: Decimal,
    pub current_price: Decimal,
    pub lastday_price: Decimal,
    pub change_today: Decimal,
}

pub struct GetPositions;
impl Request for GetPositions {
    type Body = ();
    type Response = Vec<Position>;

    fn endpoint(&self) -> Cow<str> {
        "positions".into()
    }
}

pub struct GetPosition<'a>(pub &'a str);
impl Request for GetPosition<'_> {
    type Body = ();
    type Response = Position;

    fn endpoint(&self) -> Cow<str> {
        format!("positions/{}", self.0).into()
    }
}

pub struct CloseAllPositions;
impl Request for CloseAllPositions {
    type Body = ();
    type Response = Vec<Position>;
    const METHOD: Method = Method::DELETE;

    fn endpoint(&self) -> Cow<str> {
        "positions".into()
    }
}

pub struct ClosePosition<'a>(pub &'a str);
impl Request for ClosePosition<'_> {
    type Body = ();
    type Response = Position;

    fn endpoint(&self) -> Cow<str> {
        format!("positions/{}", self.0).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Client;
    use mockito::mock;

    #[tokio::test]
    async fn test_get_positions() {
        let _m = mock("GET", "/positions")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(POSITIONS)
            .create();
        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        client.send(GetPositions).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_position() {
        let _m = mock("GET", "/positions/AAPL")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(POSITION)
            .create();
        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        client.send(GetPosition("AAPL")).await.unwrap();
    }

    const POSITION: &'static str = r#"{
	  "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
	  "symbol": "AAPL",
	  "exchange": "NASDAQ",
	  "asset_class": "us_equity",
	  "avg_entry_price": "100.0",
	  "qty": "5",
	  "side": "long",
	  "market_value": "600.0",
	  "cost_basis": "500.0",
	  "unrealized_pl": "100.0",
	  "unrealized_plpc": "0.20",
	  "unrealized_intraday_pl": "10.0",
	  "unrealized_intraday_plpc": "0.0084",
	  "current_price": "120.0",
	  "lastday_price": "119.0",
	  "change_today": "0.0084"
	}"#;

    const POSITIONS: &'static str = r#"[{
	  "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
	  "symbol": "AAPL",
	  "exchange": "NASDAQ",
	  "asset_class": "us_equity",
	  "avg_entry_price": "100.0",
	  "qty": "5",
	  "side": "long",
	  "market_value": "600.0",
	  "cost_basis": "500.0",
	  "unrealized_pl": "100.0",
	  "unrealized_plpc": "0.20",
	  "unrealized_intraday_pl": "10.0",
	  "unrealized_intraday_plpc": "0.0084",
	  "current_price": "120.0",
	  "lastday_price": "119.0",
	  "change_today": "0.0084"
	}]"#;
}
