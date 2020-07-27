use anyhow::Result;
use reqwest::Method;
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};

use crate::{alpaca_request, AlpacaConfig};

pub async fn get_account_activities(config: &AlpacaConfig) -> Result<Value> {
    let res = alpaca_request(Method::GET, "v2/account/activities", config, Some("")).await?;
    let result = serde_json::from_str(&res)?;
    Ok(result)
}
