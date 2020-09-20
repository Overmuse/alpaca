use crate::errors::Result;
use crate::utils::from_str;
use crate::{alpaca_request, AlpacaConfig};
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

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
    let res = alpaca_request(Method::GET, "account", config, None::<Account>).await?;
    let account = serde_json::from_str(&res)?;
    Ok(account)
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::mock;

    #[tokio::test]
    async fn test_get_account() {
        let _m = mock("GET", "/account")
            .with_body(
                r#"{
		  "account_blocked": false,
		  "account_number": "010203ABCD",
		  "buying_power": "262113.632",
		  "cash": "-23140.2",
		  "created_at": "2019-06-12T22:47:07.99658Z",
		  "currency": "USD",
		  "daytrade_count": 0,
		  "daytrading_buying_power": "262113.632",
		  "equity": "103820.56",
		  "id": "e6fe16f3-64a4-4921-8928-cadf02f92f98",
		  "initial_margin": "63480.38",
		  "last_equity": "103529.24",
		  "last_maintenance_margin": "38000.832",
		  "long_market_value": "126960.76",
		  "maintenance_margin": "38088.228",
		  "multiplier": "4",
		  "pattern_day_trader": false,
		  "portfolio_value": "103820.56",
		  "regt_buying_power": "80680.36",
		  "short_market_value": "0",
		  "shorting_enabled": true,
		  "sma": "0",
		  "status": "ACTIVE",
		  "trade_suspended_by_user": false,
		  "trading_blocked": false,
		  "transfers_blocked": false
		}"#,
            )
            .create();
        let config = AlpacaConfig::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        get_account(&config).await.unwrap();
    }
}
