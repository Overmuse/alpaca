
use http;
use serde::{Serialize, Deserialize};
use serde_json;

mod utils;
use utils::from_str;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OrderType {
    Market,
    Limit {
        #[serde(deserialize_with = "from_str")]
        limit_price: f64
    },
    Stop {
        #[serde(deserialize_with = "from_str")]
        stop_price: f64
    },
    StopLimit {
        #[serde(deserialize_with = "from_str")]
        limit_price: f64,
        #[serde(deserialize_with = "from_str")]
        stop_price: f64
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    DAY,
    GTC,
    OPG,
    CLS,
    IOC,
    FOK
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlpacaOrder {
    pub symbol: String,
    #[serde(deserialize_with = "from_str")]
    pub qty: String,
    pub side: Side,
    #[serde(flatten)]
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub extended_hours: bool,
    pub client_order_id: String,
}
