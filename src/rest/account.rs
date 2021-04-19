use crate::Request;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
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
    long_market_value: Decimal,
    short_market_value: Decimal,
    equity: Decimal,
    last_equity: Decimal,
    multiplier: Decimal,
    buying_power: Decimal,
    initial_margin: Decimal,
    maintenance_margin: Decimal,
    sma: Decimal,
    daytrade_count: u32,
    last_maintenance_margin: Decimal,
    daytrading_buying_power: Decimal,
    regt_buying_power: Decimal,
}

pub struct GetAccount;

impl Request for GetAccount {
    type Body = ();
    type Response = Account;

    fn endpoint(&self) -> Cow<str> {
        "account".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Client;
    use mockito::mock;

    #[tokio::test]
    async fn test_get_account() {
        let _m = mock("GET", "/account")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
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
        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        client.send(GetAccount).await.unwrap();
    }
}
