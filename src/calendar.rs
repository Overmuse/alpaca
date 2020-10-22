use crate::utils::{hm_from_str, hm_to_string};
use crate::{Request, RequestBody};
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

#[derive(Serialize)]
pub struct GetCalendar {
    start: NaiveDate,
    end: NaiveDate,
}
impl GetCalendar {
    pub fn new() -> Self {
        Default::default()
    }
}
impl Default for GetCalendar {
    fn default() -> Self {
        Self {
            start: NaiveDate::from_ymd(1970, 1, 1),
            end: NaiveDate::from_ymd(2029, 12, 31),
        }
    }
}

impl Request for GetCalendar {
    type Body = Self;
    type Response = Vec<Calendar>;

    fn endpoint(&self) -> Cow<str> {
        "calendar".into()
    }

    fn body(&self) -> RequestBody<&Self> {
        RequestBody::Query(&self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn test_get_calendar() {
        let _m = mock("GET", "/calendar")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("start".into(), "1970-01-01".into()),
                Matcher::UrlEncoded("end".into(), "2029-12-31".into()),
            ]))
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

        client.send(GetCalendar::new()).await.unwrap();
    }
}
