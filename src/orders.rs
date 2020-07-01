use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

use crate::utils::*;
use crate::{alpaca_request, AlpacaConfig, Side};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OrderType {
    Market,
    Limit {
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        limit_price: f64,
    },
    Stop {
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        stop_price: f64,
    },
    StopLimit {
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        limit_price: f64,
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        stop_price: f64,
    },
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Market
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    DAY,
    GTC,
    OPG,
    CLS,
    IOC,
    FOK,
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::DAY
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TakeProfitSpec {
    pub limit_price: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StopLossSpec {
    pub stop_price: f32,
    pub limit_price: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderClass {
    Simple,
    Bracket {
        take_profit: TakeProfitSpec,
        stop_loss: StopLossSpec,
    },
    OCO {
        take_ptofit: TakeProfitSpec,
        stop_loss: StopLossSpec,
    },
    OTO {
        stop_loss: StopLossSpec,
    },
}

impl Default for OrderClass {
    fn default() -> Self {
        OrderClass::Simple
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Accepted,
    AcceptedForBidding,
    Calculated,
    Canceled,
    DoneForDay,
    Expired,
    Filled,
    New,
    PartiallyFilled,
    PendingCancel,
    PendingNew,
    PendingReplace,
    Rejected,
    Replaced,
    Stopped,
    Suspended,
}

impl Default for OrderStatus {
    fn default() -> OrderStatus {
        OrderStatus::New
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct OrderIntent {
    pub symbol: String,
    #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
    pub qty: u32,
    pub side: Side,
    #[serde(flatten, rename(serialize = "type"))]
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub extended_hours: bool,
    pub client_order_id: Option<String>,
    pub order_class: OrderClass,
}

impl OrderIntent {
    pub fn new(symbol: &str) -> Self {
        OrderIntent {
            symbol: symbol.to_string(),
            ..Default::default()
        }
    }

    pub fn qty(mut self, qty: u32) -> Self {
        self.qty = qty;
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = order_type;
        self
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = time_in_force;
        self
    }

    pub fn extended_hours(mut self, extended_hours: bool) -> Self {
        self.extended_hours = extended_hours;
        self
    }

    pub fn client_order_id(mut self, client_order_id: String) -> Self {
        self.client_order_id = Some(client_order_id);
        self
    }

    pub fn order_class(mut self, order_class: OrderClass) -> Self {
        self.order_class = order_class;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Order {
    pub id: Uuid,
    pub client_order_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub filled_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub replaced_at: Option<DateTime<Utc>>,
    pub replaced_by: Option<Uuid>,
    pub replaces: Option<Uuid>,
    pub asset_id: Uuid,
    pub symbol: String,
    pub asset_class: String,
    #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
    pub qty: u32,
    #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
    pub filled_qty: u32,
    #[serde(flatten, rename(serialize = "type"))]
    pub order_type: OrderType,
    pub side: Side,
    pub time_in_force: TimeInForce,
    #[serde(
        deserialize_with = "from_str_optional",
        serialize_with = "to_string_optional"
    )]
    pub filled_avg_price: Option<f64>,
    pub status: OrderStatus,
    pub extended_hours: bool,
    pub legs: Option<Vec<Order>>,
}

pub async fn get_orders(config: &AlpacaConfig) -> Result<Vec<Order>> {
    let res = alpaca_request(Method::GET, "v2/orders", config, None::<Order>).await?;
    let orders: Vec<Order> = serde_json::from_str(&res)?;
    Ok(orders)
}

pub async fn get_order(config: &AlpacaConfig, order_id: &str) -> Result<Order> {
    let res = alpaca_request(
        Method::GET,
        &format!("v2/orders/{}", order_id),
        config,
        None::<Order>,
    )
    .await?;
    let order: Order = serde_json::from_str(&res)?;
    Ok(order)
}

pub async fn submit_order(config: &AlpacaConfig, order: &OrderIntent) -> Result<Order> {
    let res = alpaca_request(Method::POST, "v2/orders", config, Some(order)).await?;
    let order = serde_json::from_str(&res)?;
    Ok(order)
}
pub async fn replace_order(
    config: &AlpacaConfig,
    order_id: &str,
    order: &OrderIntent,
) -> Result<Order> {
    let res = alpaca_request(
        Method::PATCH,
        &format!("v2/orders/{}", order_id),
        config,
        Some(order),
    )
    .await?;
    let order = serde_json::from_str(&res)?;
    Ok(order)
}

pub async fn cancel_order(config: &AlpacaConfig, order_id: &str) -> Result<Order> {
    let res = alpaca_request(
        Method::DELETE,
        &format!("v2/orders/{}", order_id),
        config,
        None::<Order>,
    )
    .await?;
    let order: Order = serde_json::from_str(&res)?;
    Ok(order)
}

pub async fn cancel_all_orders(config: &AlpacaConfig) -> Result<Vec<Order>> {
    let res = alpaca_request(Method::DELETE, "v2/orders", config, None::<Order>).await?;
    let order: Vec<Order> = serde_json::from_str(&res)?;
    Ok(order)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn oi_str() -> String {
        "{\
            \"symbol\":\"AAPL\",\
            \"qty\":\"1\",\
            \"side\":\"buy\",\
            \"type\":\"limit\",\
            \"limit_price\":\"100\",\
            \"time_in_force\":\"gtc\",\
            \"extended_hours\":false,\
            \"client_order_id\":\"TEST\",\
            \"order_class\":{\
                \"bracket\":{\
                    \"take_profit\":{\
                        \"limit_price\":301.0\
                    },\
                    \"stop_loss\":{\
                        \"stop_price\":299.0,\
                        \"limit_price\":298.5\
                    }\
                }\
            }\
        }"
        .into()
    }

    fn get_order_intent() -> OrderIntent {
        OrderIntent {
            symbol: "AAPL".to_string(),
            qty: 1,
            side: Side::Buy,
            order_type: OrderType::Limit { limit_price: 100.0 },
            time_in_force: TimeInForce::GTC,
            extended_hours: false,
            client_order_id: Some("TEST".to_string()),
            order_class: OrderClass::Bracket {
                take_profit: TakeProfitSpec { limit_price: 301.0 },
                stop_loss: StopLossSpec {
                    stop_price: 299.0,
                    limit_price: 298.5,
                },
            },
        }
    }

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&get_order_intent()).unwrap(),
            oi_str(),
        )
    }

    #[test]
    fn deserialize() {
        let oi: OrderIntent = serde_json::from_str(&oi_str()).unwrap();
        assert_eq!(oi, get_order_intent(),)
    }
}
