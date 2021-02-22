use crate::utils::*;
use crate::Request;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FillType {
    Fill,
    PartialFill,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "snake_case")]
pub enum Side {
    Buy,
    Sell,
    SellShort,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Activity {
    TradeActivity {
        activity_type: String,
        id: String,
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        qty: i32,
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        cum_qty: i32,
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        leaves_qty: i32,
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        price: f64,
        side: Side,
        symbol: String,
        transaction_time: DateTime<Utc>,
        order_id: Uuid,
        #[serde(rename(serialize = "type", deserialize = "type"))]
        fill_type: FillType,
    },
    NonTradeActivity {
        activity_type: String,
        id: String,
        date: DateTime<Utc>,
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        net_amount: f64,
        symbol: Option<String>,
        #[serde(
            deserialize_with = "from_str_optional",
            serialize_with = "to_string_optional"
        )]
        qty: Option<i32>,
        #[serde(
            deserialize_with = "from_str_optional",
            serialize_with = "to_string_optional"
        )]
        per_share_amount: Option<f64>,
    },
}

pub struct GetAccountActivities;
impl Request for GetAccountActivities {
    type Body = ();
    type Response = Vec<Activity>;

    fn endpoint(&self) -> Cow<str> {
        "account/activities".into()
    }
}
