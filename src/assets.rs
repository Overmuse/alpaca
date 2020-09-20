use crate::errors::Result;
use crate::{alpaca_request, AlpacaConfig};
use reqwest::Method;
use serde::{Deserialize, Serialize};
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

pub async fn get_assets(config: &AlpacaConfig) -> Result<Vec<Asset>> {
    let res = alpaca_request(Method::GET, "assets", config, None::<Asset>).await?;
    let assets: Vec<Asset> = serde_json::from_str(&res)?;
    Ok(assets)
}

pub async fn get_asset(config: &AlpacaConfig, symbol: &str) -> Result<Asset> {
    let res = alpaca_request(
        Method::GET,
        &format!("assets/{}", symbol),
        config,
        None::<Asset>,
    )
    .await?;
    let asset: Asset = serde_json::from_str(&res)?;
    Ok(asset)
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::mock;

    fn get_config() -> AlpacaConfig {
        AlpacaConfig::new(
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

        get_assets(&get_config()).await.unwrap();
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

        get_asset(&get_config(), "AAPL").await.unwrap();
    }
}
