use http;
use serde::{Serialize, Deserialize, Serializer};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
enum Side {
    Buy,
    Sell
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
enum OrderType {
    Market,
    Limit {
        limit_price: f64
    },
    Stop {
        stop_price: f64
    },
    StopLimit {
        limit_price: f64,
        stop_price: f64
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum TimeInForce {
    DAY,
    GTC,
    OPG,
    CLS,
    IOC,
    FOK
}

#[derive(Serialize, Deserialize, Debug)]
struct AlpacaOrder {
    symbol: String,
    qty: i32,
    side: Side,
    #[serde(flatten)]
    order_type: OrderType,
    time_in_force: TimeInForce,
    extended_hours: bool,
    client_order_id: String,
}

fn main() {
    let a = AlpacaOrder{
        symbol: "AAPL".to_string(),
        qty: 1,
        side: Side::Buy,
        order_type: OrderType::Market,
        time_in_force: TimeInForce::GTC,
        extended_hours: false,
        client_order_id: "A".to_string()        
    };
    println!("{:?}", serde_json::to_string(&a))
}
