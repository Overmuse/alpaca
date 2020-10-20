use crate::Request;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Clock {
    pub timestamp: DateTime<Utc>,
    pub is_open: bool,
    pub next_open: DateTime<Utc>,
    pub next_close: DateTime<Utc>,
}

pub struct GetClock;
impl Request for GetClock {
    type Body = ();
    type Response = Clock;

    fn endpoint(&self) -> Cow<str> {
        "clock".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Client;
    use mockito::mock;

    #[tokio::test]
    async fn test_get_clock() {
        let _m = mock("GET", "/clock")
            .with_body(
                r#"{
                    "timestamp": "2018-04-01T12:00:00.000Z",
                    "is_open": true,
                    "next_open": "2018-04-01T12:00:00.000Z",
                    "next_close": "2018-04-01T12:00:00.000Z"
                }"#,
            )
            .create();
        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        client.send(GetClock {}).await.unwrap();
    }
}
