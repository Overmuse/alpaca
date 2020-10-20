use crate::utils::from_str;
use crate::Request;
use reqwest::Method;
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
    #[serde(deserialize_with = "from_str")]
    pub avg_entry_price: f64,
    #[serde(deserialize_with = "from_str")]
    pub qty: i32,
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
