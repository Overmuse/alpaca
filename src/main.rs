
use trader::{AlpacaOrder, Side, OrderType, TimeInForce};

fn main() {
    let a = AlpacaOrder{
        symbol: "AAPL".to_string(),
        qty: "1".to_string(),
        side: Side::Buy,
        order_type: OrderType::Market,
        time_in_force: TimeInForce::GTC,
        extended_hours: false,
        client_order_id: "A".to_string()        
    };
    println!("{:?}", serde_json::to_string(&a));
}
