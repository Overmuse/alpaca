use crate::utils::{hm_from_str, hm_to_string};
use crate::Request;
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Calendar {
    pub date: NaiveDate,
    #[serde(deserialize_with = "hm_from_str", serialize_with = "hm_to_string")]
    pub open: NaiveTime,
    #[serde(deserialize_with = "hm_from_str", serialize_with = "hm_to_string")]
    pub close: NaiveTime,
}

pub struct GetCalendar;
impl Request for GetCalendar {
    type Body = ();
    type Response = Vec<Calendar>;

    fn endpoint(&self) -> Cow<str> {
        "calendar".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Client;
    use mockito::mock;

    #[tokio::test]
    async fn test_get_calendar() {
        let _m = mock("GET", "/calendar")
            .with_body(
                r#"[
		       {
			  "date": "2018-01-03",
			  "open": "09:30",
			  "close": "16:00"
		       }
		]"#,
            )
            .create();
        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        client.send(GetCalendar {}).await.unwrap();
    }
}
