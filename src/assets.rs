use anyhow::Result;
use reqwest::Method;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{AlpacaConfig, alpaca_request};

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
    Inactive
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlpacaAsset {
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

pub async fn get_assets(config: AlpacaConfig) -> Result<Vec<AlpacaAsset>> {
   let res = alpaca_request(Method::GET, "v2/assets", config, None::<AlpacaAsset>).await?;
   let assets: Vec<AlpacaAsset> = serde_json::from_str(&res)?;
   Ok(assets)
}

pub async fn get_asset(config: AlpacaConfig, symbol: &str) -> Result<AlpacaAsset>{
   let res = alpaca_request(Method::GET, &format!("v2/assets/{}", symbol), config, None::<AlpacaAsset>).await?;
   let assets: AlpacaAsset = serde_json::from_str(&res)?;
   Ok(assets)
}
