use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

use crate::utils::from_str;
use crate::{alpaca_request, AlpacaConfig};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
    Onboarding,
    SubmissionFailed,
    Submitted,
    AccountUpdate,
    ApprovalPending,
    Active,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    id: Uuid,
    account_number: String,
    status: AccountStatus,
    currency: String,
    pattern_day_trader: bool,
    trade_suspended_by_user: bool,
    trading_blocked: bool,
    transfers_blocked: bool,
    account_blocked: bool,
    created_at: DateTime<Utc>,
    shorting_enabled: bool,
    #[serde(deserialize_with = "from_str")]
    long_market_value: f64,
    #[serde(deserialize_with = "from_str")]
    short_market_value: f64,
    #[serde(deserialize_with = "from_str")]
    equity: f64,
    #[serde(deserialize_with = "from_str")]
    last_equity: f64,
    #[serde(deserialize_with = "from_str")]
    multiplier: f64,
    #[serde(deserialize_with = "from_str")]
    buying_power: f64,
    #[serde(deserialize_with = "from_str")]
    initial_margin: f64,
    #[serde(deserialize_with = "from_str")]
    maintenance_margin: f64,
    #[serde(deserialize_with = "from_str")]
    sma: f64,
    daytrade_count: u32,
    #[serde(deserialize_with = "from_str")]
    last_maintenance_margin: f64,
    #[serde(deserialize_with = "from_str")]
    daytrading_buying_power: f64,
    #[serde(deserialize_with = "from_str")]
    regt_buying_power: f64,
}

pub async fn get_account(config: &AlpacaConfig) -> Result<Account> {
    let res = alpaca_request(Method::GET, "/account", config, None::<Account>).await?;
    let account = serde_json::from_str(&res)?;
    Ok(account)
}
