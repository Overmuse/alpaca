use crate::common::Order;
use crate::utils::{from_str, to_string};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum AlpacaData {
    #[serde(rename = "streams")]
    Streams(Vec<String>),
    #[serde(rename = "auth")]
    Auth { key_id: String, secret_key: String },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "action", content = "data")]
pub enum AlpacaAction {
    #[serde(rename = "listen")]
    Listen { streams: Vec<String> },
    #[serde(rename = "authenticate")]
    Authenticate { key_id: String, secret_key: String },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        price: f64,
        timestamp: DateTime<Utc>,
        #[cfg(feature = "fractional-shares")]
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        position_qty: f64,
        #[cfg(not(feature = "fractional-shares"))]
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        positition_qty: isize,
    },
    New,
    OrderCancelRejected,
    OrderReplaceRejected,
    PartialFill {
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        price: f64,
        timestamp: DateTime<Utc>,
        #[cfg(feature = "fractional-shares")]
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        position_qty: f64,
        #[cfg(not(feature = "fractional-shares"))]
        #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
        position_qty: isize,
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct OrderEvent {
    #[serde(flatten)]
    pub event: Event,
    pub order: Order,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationStatus {
    Authorized,
    Unauthorized,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

    #[test]
    fn serde_order_event() {
        let json = r#"{"stream":"trade_updates","data":{"event":"fill","price":"179.08","timestamp":"2018-02-28T20:38:22Z","position_qty":"100","order":{"id":"61e69015-8549-4bfd-b9c3-01e75843f47d","client_order_id":"eb9e2aaa-f71a-4f51-b5b4-52a6c565dad4","created_at":"2021-03-16T18:38:01.942282Z","updated_at":"2021-03-16T18:38:01.942282Z","submitted_at":"2021-03-16T18:38:01.937734Z","filled_at":null,"expired_at":null,"canceled_at":null,"failed_at":null,"replaced_at":null,"replaced_by":null,"replaces":null,"asset_id":"b0b6dd9d-8b9b-48a9-ba46-b9d54906e415","symbol":"AAPL","asset_class":"us_equity","notional":"500","qty":null,"filled_qty":"0","filled_avg_price":null,"order_class":"","order_type":"market","type":"market","side":"buy","time_in_force":"day","limit_price":null,"stop_price":null,"status":"accepted","extended_hours":false,"legs":null,"trail_percent":null,"trail_price":null,"hwm":null}}}"#;
        let deserialized: AlpacaMessage = serde_json::from_str(json).unwrap();
        let _serialized = serde_json::to_string(&deserialized).unwrap();
    }
}
