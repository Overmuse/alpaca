use crate::common::{Order, OrderClass, OrderType, Side, TimeInForce};
use crate::utils::*;
use crate::{Request, RequestBody};
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Market
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AmountSpec {
    #[serde(
        rename = "qty",
        deserialize_with = "from_str",
        serialize_with = "to_string"
    )]
    Quantity(f64),
    #[serde(
        rename = "notional",
        deserialize_with = "from_str",
        serialize_with = "to_string"
    )]
    Notional(f64),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OrderIntent {
    pub symbol: String,
    #[cfg(feature = "fractional-shares")]
    #[serde(flatten)]
    pub amount: AmountSpec,
    #[cfg(not(feature = "fractional-shares"))]
    #[serde(deserialize_with = "from_str", serialize_with = "to_string")]
    pub qty: usize,
    pub side: Side,
    #[serde(flatten, rename(serialize = "type"))]
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub extended_hours: bool,
    pub client_order_id: Option<String>,
    pub order_class: OrderClass,
}

impl OrderIntent {
    pub fn new(symbol: &str) -> Self {
        OrderIntent {
            symbol: symbol.to_string(),
            #[cfg(feature = "fractional-shares")]
            amount: AmountSpec::Quantity(1.0),
            #[cfg(not(feature = "fractional-shares"))]
            qty: 1,
            side: Side::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GoodTilCancelled,
            extended_hours: false,
            client_order_id: None,
            order_class: OrderClass::Simple,
        }
    }

    #[cfg(feature = "fractional-shares")]
    pub fn qty(mut self, qty: f64) -> Self {
        self.amount = AmountSpec::Quantity(qty);
        self
    }

    #[cfg(not(feature = "fractional-shares"))]
    pub fn qty(mut self, qty: usize) -> Self {
        self.qty = qty;
        self
    }
    #[cfg(feature = "fractional-shares")]
    pub fn notional(mut self, notional: f64) -> Self {
        self.amount = AmountSpec::Notional(notional);
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = order_type;
        self
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = time_in_force;
        self
    }

    pub fn extended_hours(mut self, extended_hours: bool) -> Self {
        self.extended_hours = extended_hours;
        self
    }

    pub fn client_order_id(mut self, client_order_id: String) -> Self {
        self.client_order_id = Some(client_order_id);
        self
    }

    pub fn order_class(mut self, order_class: OrderClass) -> Self {
        self.order_class = order_class;
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum QueryOrderStatus {
    Open,
    Closed,
    All,
}
impl Default for QueryOrderStatus {
    fn default() -> Self {
        QueryOrderStatus::Open
    }
}

#[derive(Serialize)]
pub enum Sort {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}
impl Default for Sort {
    fn default() -> Self {
        Sort::Descending
    }
}

#[derive(Serialize)]
pub struct GetOrders {
    status: QueryOrderStatus,
    limit: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until: Option<DateTime<Utc>>,
    direction: Sort,
    nested: bool,
}
impl GetOrders {
    pub fn new() -> Self {
        Default::default()
    }
}
impl Default for GetOrders {
    fn default() -> Self {
        Self {
            status: Default::default(),
            limit: 50,
            after: None,
            until: None,
            direction: Default::default(),
            nested: false,
        }
    }
}

impl Request for GetOrders {
    type Body = Self;
    type Response = Vec<Order>;

    fn endpoint(&self) -> Cow<str> {
        "orders".into()
    }

    fn body(&self) -> RequestBody<&Self> {
        RequestBody::Query(&self)
    }
}

#[derive(Serialize)]
pub struct GetOrder<'a> {
    #[serde(skip)]
    pub order_id: &'a str,
    pub nested: bool,
}
impl<'a> GetOrder<'a> {
    pub fn new(order_id: &'a str) -> Self {
        Self {
            order_id,
            nested: false,
        }
    }
}
impl Request for GetOrder<'_> {
    type Body = Self;
    type Response = Order;

    fn endpoint(&self) -> Cow<str> {
        format!("orders/{}", self.order_id).into()
    }

    fn body(&self) -> RequestBody<&Self> {
        RequestBody::Query(&self)
    }
}

pub struct SubmitOrder(pub OrderIntent);
impl Request for SubmitOrder {
    type Body = OrderIntent;
    type Response = Order;
    const METHOD: Method = Method::POST;

    fn endpoint(&self) -> Cow<str> {
        "orders".into()
    }

    fn body(&self) -> RequestBody<&OrderIntent> {
        RequestBody::Json(&self.0)
    }
}

pub struct ReplaceOrder<'a>(pub &'a str, pub OrderIntent);
impl Request for ReplaceOrder<'_> {
    type Body = OrderIntent;
    type Response = Order;
    const METHOD: Method = Method::POST;

    fn endpoint(&self) -> Cow<str> {
        format!("orders/{}", self.0).into()
    }

    fn body(&self) -> RequestBody<&OrderIntent> {
        RequestBody::Json(&self.1)
    }
}

pub struct EmptyResponse;
impl<'de> Deserialize<'de> for EmptyResponse {
    fn deserialize<D>(_deserializer: D) -> Result<EmptyResponse, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(EmptyResponse {})
    }
}

pub struct CancelOrder<'a>(pub &'a str);
impl Request for CancelOrder<'_> {
    type Body = ();
    type Response = EmptyResponse;
    const METHOD: Method = Method::DELETE;

    fn endpoint(&self) -> Cow<str> {
        format!("orders/{}", self.0).into()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancellationAttempt {
    id: Uuid,
    status: usize,
    body: Order,
}

pub struct CancelAllOrders();
impl Request for CancelAllOrders {
    type Body = ();
    type Response = Vec<Order>;
    const METHOD: Method = Method::DELETE;

    fn endpoint(&self) -> Cow<str> {
        "orders".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[test]
    fn test_defaults() {
        match OrderType::default() {
            OrderType::Market => {} // Happy case
            _ => panic!(),
        };
        match TimeInForce::default() {
            TimeInForce::Day => {} // Happy case
            _ => panic!(),
        };
        match OrderClass::default() {
            OrderClass::Simple => {} // Happy case
            _ => panic!(),
        };
        match Side::default() {
            Side::Buy => {} // Happy case
            _ => panic!(),
        };
    }

    #[test]
    fn serde() {
        let json = r#"{
            "symbol":"AAPL",
            "notional":"1.23",
            "side":"buy",
            "type":"limit",
            "limit_price":"100",
            "time_in_force":"gtc",
            "extended_hours":false,
            "client_order_id":"TEST",
            "order_class":{
                "bracket":{
                    "take_profit":{
                        "limit_price":301.0
                    },
                    "stop_loss":{
                        "stop_price":299.0,
                        "limit_price":298.5
                    }
                }
            }
        }"#;
        let deserialized: OrderIntent = serde_json::from_str(json).unwrap();
        let _serialized = serde_json::to_string(&deserialized).unwrap();
    }

    #[tokio::test]
    async fn test_get_order() {
        let _m = mock("GET", "/orders/904837e3-3b76-47ec-b432-046db621571b")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::UrlEncoded("nested".into(), "false".into()))
            .with_body(
                r#"
                    {
			  "id": "904837e3-3b76-47ec-b432-046db621571b",
			  "client_order_id": "904837e3-3b76-47ec-b432-046db621571b",
			  "created_at": "2018-10-05T05:48:59Z",
			  "updated_at": "2018-10-05T05:48:59Z",
			  "submitted_at": "2018-10-05T05:48:59Z",
			  "filled_at": "2018-10-05T05:48:59Z",
			  "expired_at": "2018-10-05T05:48:59Z",
			  "canceled_at": "2018-10-05T05:48:59Z",
			  "failed_at": "2018-10-05T05:48:59Z",
			  "replaced_at": "2018-10-05T05:48:59Z",
			  "replaced_by": "904837e3-3b76-47ec-b432-046db621571b",
			  "replaces": null,
			  "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
			  "symbol": "AAPL",
			  "asset_class": "us_equity",
			  "qty": "15",
			  "filled_qty": "0",
			  "type": "market",
			  "side": "buy",
			  "time_in_force": "day",
			  "limit_price": "107.00",
			  "stop_price": "106.00",
			  "filled_avg_price": "106.00",
			  "status": "accepted",
			  "extended_hours": false,
			  "legs": null,
                          "trail_price": "1.05",
                          "trail_percent": null,
                          "hwm": "108.05"
			}
                "#,
            )
            .create();
        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        client
            .send(GetOrder::new("904837e3-3b76-47ec-b432-046db621571b"))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_orders() {
        let _m = mock("GET", "/orders")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("status".into(), "open".into()),
                Matcher::UrlEncoded("limit".into(), "50".into()),
                Matcher::UrlEncoded("direction".into(), "desc".into()),
                Matcher::UrlEncoded("nested".into(), "false".into()),
            ]))
            .with_body(
                r#"[
                    {
			  "id": "904837e3-3b76-47ec-b432-046db621571b",
			  "client_order_id": "904837e3-3b76-47ec-b432-046db621571b",
			  "created_at": "2018-10-05T05:48:59Z",
			  "updated_at": "2018-10-05T05:48:59Z",
			  "submitted_at": "2018-10-05T05:48:59Z",
			  "filled_at": "2018-10-05T05:48:59Z",
			  "expired_at": "2018-10-05T05:48:59Z",
			  "canceled_at": "2018-10-05T05:48:59Z",
			  "failed_at": "2018-10-05T05:48:59Z",
			  "replaced_at": "2018-10-05T05:48:59Z",
			  "replaced_by": "904837e3-3b76-47ec-b432-046db621571b",
			  "replaces": null,
			  "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
			  "symbol": "AAPL",
			  "asset_class": "us_equity",
			  "qty": "15",
			  "filled_qty": "0",
			  "type": "market",
			  "side": "buy",
			  "time_in_force": "day",
			  "limit_price": "107.00",
			  "stop_price": "106.00",
			  "filled_avg_price": "106.00",
			  "status": "accepted",
			  "extended_hours": false,
			  "legs": null,
                          "trail_price": "1.05",
                          "trail_percent": null,
                          "hwm": "108.05"
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

        client.send(GetOrders::new()).await.unwrap();
    }

    #[tokio::test]
    async fn missing_order() {
        let _m = mock("GET", "/orders/904837e3-3b76-47ec-b432-046db621571b")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::UrlEncoded("nested".into(), "false".into()))
            .with_status(404)
            .create();

        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        let res = client
            .send(GetOrder::new("904837e3-3b76-47ec-b432-046db621571b"))
            .await;

        assert!(res.is_err())
    }

    #[tokio::test]
    async fn test_cancel_order() {
        let _m = mock("DELETE", "/orders/904837e3-3b76-47ec-b432-046db621571b")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_status(204)
            .with_body(
                r#"{
                    "id": "904837e3-3b76-47ec-b432-046db621571b",
		    "client_order_id": "904837e3-3b76-47ec-b432-046db621571b",
	            "created_at": "2018-10-05T05:48:59Z",
		    "updated_at": "2018-10-05T05:48:59Z",
		    "submitted_at": "2018-10-05T05:48:59Z",
		    "filled_at": "2018-10-05T05:48:59Z",
		    "expired_at": "2018-10-05T05:48:59Z",
		    "canceled_at": "2018-10-05T05:48:59Z",
		    "failed_at": "2018-10-05T05:48:59Z",
		    "replaced_at": "2018-10-05T05:48:59Z",
		    "replaced_by": "904837e3-3b76-47ec-b432-046db621571b",
		    "replaces": null,
		    "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
		    "symbol": "AAPL",
		    "asset_class": "us_equity",
		    "qty": "15",
		    "filled_qty": "0",
		    "type": "market",
		    "side": "buy",
		    "time_in_force": "day",
		    "limit_price": "107.00",
		    "stop_price": "106.00",
		    "filled_avg_price": "106.00",
		    "status": "accepted",
		    "extended_hours": false,
		    "legs": null,
                    "trail_price": "1.05",
                    "trail_percent": null,
                    "hwm": "108.05"
		}"#,
            )
            .create();

        let client = Client::new(
            mockito::server_url(),
            "APCA_API_KEY_ID".to_string(),
            "APCA_API_SECRET_KEY".to_string(),
        )
        .unwrap();

        client
            .send(CancelOrder("904837e3-3b76-47ec-b432-046db621571b"))
            .await
            .unwrap();
    }
}
