use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

use crate::utils::*;
use crate::{alpaca_request, AlpacaConfig, Side};

pub async fn get_account_activities(config: &AlpacaConfig) -> Result<Vec<Activity>> {
    let res = alpaca_request(
        Method::GET,
        "v2/account/activities",
        config,
        None::<Activity>,
    )
    .await?;
    let result: Vec<Activity> = serde_json::from_str(&res)?;
    Ok(result)
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FillType {
    Fill,
    PartialFill,
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
