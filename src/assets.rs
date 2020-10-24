use crate::{Request, RequestBody};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AssetClass {
    UsEquity,
}
impl Default for AssetClass {
    fn default() -> Self {
        AssetClass::UsEquity
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Exchange {
    AMEX,
    ARCA,
    BATS,
    NYSE,
    NASDAQ,
    NYSEARCA,
    OTC,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Active,
    Inactive,
}
impl Default for Status {
    fn default() -> Self {
        Status::Active
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    id: Uuid,
    class: AssetClass,
    exchange: Exchange,
    symbol: String,
    status: Status,
    tradable: bool,
    marginable: bool,
    shortable: bool,
    easy_to_borrow: bool,
}

#[derive(Serialize, Default)]
pub struct GetAssets {
    status: Status,
    asset_class: AssetClass,
}
impl GetAssets {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Request for GetAssets {
    type Body = Self;
    type Response = Asset;

    fn endpoint(&self) -> Cow<str> {
        "assets".into()
    }

    fn body(&self) -> RequestBody<&Self> {
        RequestBody::Query(&self)
    }
}

#[derive(Serialize)]
pub struct GetAsset<'a>(&'a str);

impl Request for GetAsset<'_> {
    type Body = ();
    type Response = Asset;

    fn endpoint(&self) -> Cow<str> {
        format!("assets/{}", self.0).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn test_get_assets() {
        let _m = mock("GET", "/assets")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("status".into(), "active".into()),
                Matcher::UrlEncoded("asset_class".into(), "us_equity".into()),
            ]))
            .with_body(
                r#"{
                    "id": "904837e3-3b76-47ec-b432-046db621571b",
  		    "class": "us_equity",
  		    "exchange": "NASDAQ",
  		    "symbol": "AAPL",
  		    "status": "active",
  		    "tradable": true,
  		    "marginable": true,
  		    "shortable": true,
  		    "easy_to_borrow": true 
		}"#,
            )
            .create();
        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        client.send(GetAssets::new()).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_asset() {
        let _m = mock("GET", "/assets/AAPL")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(
                r#"{
                    "id": "904837e3-3b76-47ec-b432-046db621571b",
  		    "class": "us_equity",
  		    "exchange": "NASDAQ",
  		    "symbol": "AAPL",
  		    "status": "active",
  		    "tradable": true,
  		    "marginable": true,
  		    "shortable": true,
  		    "easy_to_borrow": true 
		}"#,
            )
            .create();
        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        client.send(GetAsset("AAPL")).await.unwrap();
    }
}
