use crate::Request;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AssetClass {
    UsEquity,
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

pub struct GetAssets;
impl Request for GetAssets {
    type Body = ();
    type Response = Vec<Asset>;

    fn endpoint(&self) -> Cow<str> {
        "assets".into()
    }
}

pub struct GetAsset<'a>(pub &'a str);
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
    use mockito::mock;

    fn get_client() -> Client {
        Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn test_get_assets() {
        let _m = mock("GET", "/assets")
            .with_body(
                r#"[
                   	{
			    "id": "904837e3-3b76-47ec-b432-046db621571b",
			    "class": "us_equity",
			    "exchange": "NASDAQ",
			    "symbol": "AAPL",
			    "status": "active",
			    "tradable": true,
			    "marginable": true,
			    "shortable": true,
			    "easy_to_borrow": true
			  } 
                    ]"#,
            )
            .create();

        get_client().send(GetAssets {}).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_asset() {
        let _m = mock("GET", "/assets/AAPL")
            .with_body(
                r#"
                   	{
			    "id": "904837e3-3b76-47ec-b432-046db621571b",
			    "class": "us_equity",
			    "exchange": "NASDAQ",
			    "symbol": "AAPL",
			    "status": "active",
			    "tradable": true,
			    "marginable": true,
			    "shortable": true,
			    "easy_to_borrow": true
			  } 
		"#,
            )
            .create();

        get_client().send(GetAsset("AAPL")).await.unwrap();
    }
}
