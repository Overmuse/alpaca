use alpaca::{account::*, account_activities::*, assets::*, orders::*, positions::*, AlpacaConfig};
use std::env;

#[tokio::main]
async fn main() {
    let config = AlpacaConfig::new(
        "https://paper-api.alpaca.markets".to_string(),
        env::var("ALPACA_KEY_ID").unwrap(),
        env::var("ALPACA_SECRET_KEY").unwrap(),
    )
    .unwrap();

    let o = OrderIntent {
        symbol: "AAPL".to_string(),
        qty: 1,
        side: alpaca::Side::Buy,
        order_type: OrderType::Limit { limit_price: 100.0 },
        time_in_force: TimeInForce::GTC,
        extended_hours: false,
        client_order_id: Some("TEST".to_string()),
        order_class: OrderClass::Bracket {
            take_profit: TakeProfitSpec { limit_price: 301.0 },
            stop_loss: StopLossSpec {
                stop_price: 299.0,
                limit_price: 298.5,
            },
        },
    };
    //println!("{:#?}", &o);
    //println!("{:#?}", &serde_json::to_string(&o).unwrap());
    println!("{:#?}", get_account_activities(&config).await.unwrap());
    //println!("{:#?}", get_positions(&config).await.unwrap());
}
