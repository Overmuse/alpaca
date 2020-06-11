use std::env;
use tokio::runtime::Runtime;
use alpaca::{AlpacaConfig, account::get_account, orders::get_orders, positions::close_positions};

fn main() {
    let config = AlpacaConfig::new(
        "https://paper-api.alpaca.markets".to_string(),
        env::var("ALPACA_KEY_ID").unwrap(),
        env::var("ALPACA_SECRET_KEY").unwrap()
    ).unwrap();

    let mut runtime = Runtime::new().unwrap();

    println!("{:?}", runtime.block_on(close_positions(config))); 
}

