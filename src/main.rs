use std::env;
use alpaca::{AlpacaConfig, orders::*, positions::*, assets::*};

#[tokio::main]
async fn main() {
    let config = AlpacaConfig::new(
        "https://paper-api.alpaca.markets".to_string(),
        env::var("ALPACA_KEY_ID").unwrap(),
        env::var("ALPACA_SECRET_KEY").unwrap()
    ).unwrap();

    let _o = AlpacaOrder {
        symbol: "AAPL".to_string(),
        qty: 1,
        side: alpaca::Side::Buy,
        order_type: OrderType::Limit{ limit_price: 100.0 },
        time_in_force: TimeInForce::GTC,
        extended_hours: false,
        client_order_id: Some("BOOGALOO2".to_string())
    };
    println!("{:?}", get_asset(&config, "AAPL").await); 
}

