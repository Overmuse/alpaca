use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::Neg;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OrderType {
    Market,
    Limit {
        limit_price: Decimal,
    },
    Stop {
        stop_price: Decimal,
    },
    StopLimit {
        limit_price: Decimal,
        stop_price: Decimal,
    },
    TrailingStop {
        trail_price: Option<Decimal>,
        trail_percent: Option<Decimal>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TimeInForce {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "gtc")]
    GoodTilCancelled,
    #[serde(rename = "opg")]
    Open,
    #[serde(rename = "cls")]
    Close,
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    #[serde(rename = "fok")]
    FillOrKill,
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::Day
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TakeProfitSpec {
    pub limit_price: Decimal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StopLossSpec {
    pub stop_price: Decimal,
    pub limit_price: Decimal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OrderClass {
    Simple,
    Bracket {
        take_profit: TakeProfitSpec,
        stop_loss: StopLossSpec,
    },
    OneCancelsOther {
        take_ptofit: TakeProfitSpec,
        stop_loss: StopLossSpec,
    },
    OneTriggersOther {
        stop_loss: StopLossSpec,
    },
}

impl Default for OrderClass {
    fn default() -> Self {
        OrderClass::Simple
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

impl Default for Side {
    fn default() -> Side {
        Side::Buy
    }
}

impl Neg for Side {
    type Output = Side;

    fn neg(self) -> Self::Output {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Order {
    pub id: Uuid,
    pub client_order_id: String,
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
    pub notional: Option<Decimal>,
    #[serde(
        deserialize_with = "crate::utils::from_str",
        serialize_with = "crate::utils::to_string"
    )]
    pub qty: usize,
    #[serde(
        deserialize_with = "crate::utils::from_str",
        serialize_with = "crate::utils::to_string"
    )]
    pub filled_qty: usize,
    pub filled_avg_price: Option<Decimal>,
    #[serde(flatten, rename(serialize = "type"))]
    pub order_type: OrderType,
    pub side: Side,
    pub time_in_force: TimeInForce,
    pub status: OrderStatus,
    pub extended_hours: bool,
    pub legs: Option<Vec<Order>>,
    pub hwm: Option<Decimal>,
}
