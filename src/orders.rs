use anyhow::Result;
use reqwest::Method;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::utils::from_str;
use crate::{AlpacaConfig, Side, alpaca_request};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OrderType {
    Market,
    Limit {
        #[serde(deserialize_with = "from_str")]
        limit_price: f64
    },
    Stop {
        #[serde(deserialize_with = "from_str")]
        stop_price: f64
    },
    StopLimit {
        #[serde(deserialize_with = "from_str")]
        limit_price: f64,
        #[serde(deserialize_with = "from_str")]
        stop_price: f64
    }
}

impl Default for OrderType {
    fn default() -> Self { OrderType::Market }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    DAY,
    GTC,
    OPG,
    CLS,
    IOC,
    FOK
}

impl Default for TimeInForce {
    fn default() -> Self { TimeInForce::DAY }
}

//#[derive(Serialize, Deserialize, Debug)]
//pub struct TakeProfitSpec {
//    pub limit_price: f32,
//}
//
//#[derive(Serialize, Deserialize, Debug)]
//pub struct StopLossSpec {
//    pub stop_price: f32,
//    pub limit_price: f32,
//}
//
//#[derive(Serialize, Deserialize, Debug)]
//#[serde(rename_all = "lowercase")]
//pub enum OrderClass {
//    Simple,
//    Bracket {
//        take_profit: TakeProfitSpec,
//        stop_loss: StopLossSpec
//    }
//}
//
//impl Default for OrderClass {
//    fn default() -> Self { OrderClass::Simple }
//}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AlpacaOrder {
    pub symbol: String,
    #[serde(deserialize_with = "from_str")]
    pub qty: u32,
    pub side: Side,
    #[serde(flatten, rename(serialize = "type"))]
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub extended_hours: bool,
    pub client_order_id: Option<String>,
    //pub order_class: OrderClass,
}

impl AlpacaOrder {
    pub fn new(symbol: &str) -> Self {
        AlpacaOrder {
            symbol: symbol.to_string(),
            ..Default::default()
        }
    }

    //pub fn submit(&self) -> Result<reqwest::Response> {
    //    let json_order = serde_json::to_string(self)?; 
    //    println!("{:?}", &json_order);
    //    let response = client.post("http://paper-api.alpaca.markets/v2/orders")
    //        .header("APCA-API-KEY-ID", env::var("ALPACA_KEY_ID")?)
    //        .header("APCA-API-SECRET-KEY", env::var("ALPACA_SECRET_KEY")?)
    //        .body(json_order)
    //        .send()?;
    //    Ok(response)
    //}
}

pub async fn get_orders(config: &AlpacaConfig) -> Result<Vec<AlpacaOrder>> {
   let res = alpaca_request(Method::GET, "v2/orders", config, None::<AlpacaOrder>).await?;
   let orders: Vec<AlpacaOrder> = serde_json::from_str(&res)?;
   Ok(orders)
}

pub async fn get_order(config: &AlpacaConfig, order_id: &str) -> Result<AlpacaOrder> {
   let res = alpaca_request(Method::GET, &format!("v2/orders/{}", order_id), config, None::<AlpacaOrder>).await?;
   let order: AlpacaOrder = serde_json::from_str(&res)?;
   Ok(order)

}

pub async fn submit_order(config: &AlpacaConfig, order: &AlpacaOrder) -> Result<AlpacaOrder> {
    let res = alpaca_request(Method::POST, "v2/orders", config, Some(order)).await?;
    let order = serde_json::from_str(&res)?;
    Ok(order)
}
pub async fn replace_order(config: &AlpacaConfig, order_id: &str, order: &AlpacaOrder) -> Result<AlpacaOrder> {
    let res = alpaca_request(Method::PATCH, &format!("v2/orders/{}", order_id), config, Some(order)).await?;
    let order = serde_json::from_str(&res)?;
    Ok(order)
}

pub async fn cancel_order(config: &AlpacaConfig, order_id: &str) -> Result<AlpacaOrder> {
   let res = alpaca_request(Method::DELETE, &format!("v2/orders/{}", order_id), config, None::<AlpacaOrder>).await?;
   let order: AlpacaOrder = serde_json::from_str(&res)?;
   Ok(order)
}

pub async fn cancel_all_orders(config: &AlpacaConfig) -> Result<Vec<AlpacaOrder>> {
   let res = alpaca_request(Method::DELETE, "v2/orders", config, None::<AlpacaOrder>).await?;
   let order: Vec<AlpacaOrder> = serde_json::from_str(&res)?;
   Ok(order)
}
