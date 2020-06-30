use anyhow::Result;
use reqwest::Method;
use serde::{Serialize, Deserialize};
use serde_json;
use uuid::Uuid;

use crate::utils::from_str;
use crate::{AlpacaConfig, Side, alpaca_request};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Position {
    pub asset_id: Uuid,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: String,
    #[serde(deserialize_with = "from_str")]
    pub avg_entry_price: f64,
    #[serde(deserialize_with = "from_str")]
    pub qty: u32,
    pub side: Side,
    #[serde(deserialize_with = "from_str")]
    pub market_value: f64,
    #[serde(deserialize_with = "from_str")]
    pub cost_basis: f64,
    #[serde(deserialize_with = "from_str")]
    pub unrealized_pl: f64,
    #[serde(deserialize_with = "from_str")]
    pub unrealized_plpc: f64,
    #[serde(deserialize_with = "from_str")]
    pub unrealized_intraday_pl: f64,
    #[serde(deserialize_with = "from_str")]
    pub unrealized_intraday_plpc: f64,
    #[serde(deserialize_with = "from_str")]
    pub current_price: f64,
    #[serde(deserialize_with = "from_str")]
    pub lastday_price: f64,
    #[serde(deserialize_with = "from_str")]
    pub change_today: f64,
}

pub async fn get_positions(config: &AlpacaConfig) -> Result<Vec<Position>> {
   let res = alpaca_request(Method::GET, "v2/positions", config, None::<Position>).await?;
   let positions: Vec<Position> = serde_json::from_str(&res)?;
   Ok(positions)
}

pub async fn get_position(config: &AlpacaConfig, position: &str) -> Result<Position> {
   let res = alpaca_request(Method::GET, &format!("v2/positions/{}", position), config, None::<Position>).await?;
   let position: Position = serde_json::from_str(&res)?;
   Ok(position)
}

pub async fn close_all_positions(config: &AlpacaConfig) -> Result<Vec<Position>> {
   let res = alpaca_request(Method::DELETE, "v2/positions", config, None::<Position>).await?;
   let positions: Vec<Position> = serde_json::from_str(&res)?;
   Ok(positions)
}

pub async fn close_position(config: &AlpacaConfig, position: &str) -> Result<Position> {
   let res = alpaca_request(Method::DELETE, &format!("v2/positions/{}", position), config, None::<Position>).await?;
   let position: Position = serde_json::from_str(&res)?;
   Ok(position)
}
