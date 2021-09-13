use crate::{Request, RequestBody};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AssetClass {
    UsEquity,
}
impl Default for AssetClass {
    fn default() -> Self {
        AssetClass::UsEquity
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Exchange {
    Amex,
    Arca,
    Bats,
    Nyse,
    Nasdaq,
    NyseArca,
    Otc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Asset {
    pub id: Uuid,
    pub class: AssetClass,
    pub exchange: Exchange,
    pub symbol: String,
    pub status: Status,
    pub tradable: bool,
    pub marginable: bool,
    pub shortable: bool,
    pub easy_to_borrow: bool,
}

#[derive(Serialize, Default, Clone, Debug)]
pub struct GetAssets {
    pub status: Status,
    pub asset_class: AssetClass,
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
        RequestBody::Query(self)
    }
}

#[derive(Serialize, Clone, Debug)]
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

    #[test]
    fn test_default_assetclass() {
        match AssetClass::default() {
            AssetClass::UsEquity => {} // Happy case
        }
    }

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
