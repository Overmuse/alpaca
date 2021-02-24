use crate::orders::Order;
use crate::utils::from_str;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum AlpacaData {
    #[serde(rename = "streams")]
    Streams(Vec<String>),
    #[serde(rename = "auth")]
    Auth { key_id: String, secret_key: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", content = "data")]
pub enum AlpacaAction {
    #[serde(rename = "listen")]
    Listen { streams: Vec<String> },
    #[serde(rename = "authenticate")]
    Authenticate { key_id: String, secret_key: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum Event {
    Calculated,
    Canceled {
        timestamp: DateTime<Utc>,
    },
    DoneForDay,
    Expired {
        timestamp: DateTime<Utc>,
    },
    Fill {
        timestamp: DateTime<Utc>,
        #[serde(deserialize_with = "from_str")]
        price: f64,
        #[serde(deserialize_with = "from_str")]
        qty: u32,
    },
    New,
    OrderCancelRejected,
    OrderReplaceRejected,
    PartialFill {
        timestamp: DateTime<Utc>,
        #[serde(deserialize_with = "from_str")]
        price: f64,
        #[serde(deserialize_with = "from_str")]
        qty: u32,
    },
    PendingCancel,
    PendingNew,
    PendingReplace,
    Rejected {
        timestamp: DateTime<Utc>,
    },
    Replaced {
        timestamp: DateTime<Utc>,
    },
    Stopped,
    Suspended,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OrderEvent {
    #[serde(flatten)]
    pub event: Event,
    pub order: Order,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationStatus {
    Authorized,
    Unauthorized,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "stream", content = "data", rename_all = "snake_case")]
pub enum AlpacaMessage {
    Authorization {
        status: AuthorizationStatus,
        action: String,
    },
    Listening {
        streams: Vec<String>,
    },
    TradeUpdates(Box<OrderEvent>),
    AccountUpdates {
        id: String,
        created_at: String,
        updated_at: String,
        deleted_at: Option<String>,
        status: String,
        currency: String,
        #[serde(deserialize_with = "from_str")]
        cash: f64,
        #[serde(deserialize_with = "from_str")]
        cash_withdrawable: f64,
    },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serde_listen() {
        let serialized = serde_json::to_string(&AlpacaAction::Listen {
            streams: vec!["trade_updates".into(), "account_updates".into()],
        })
        .unwrap();
        assert_eq!(serialized, "{\"action\":\"listen\",\"data\":{\"streams\":[\"trade_updates\",\"account_updates\"]}}");
    }
}
