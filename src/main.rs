use alpaca::{assets::*, orders::*, AlpacaConfig};
use env_logger;
use log::{error, info};

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = AlpacaConfig::from_env()?;

    let _o = OrderIntent {
        symbol: "AAPL".to_string(),
        qty: 1,
        side: alpaca::orders::Side::Buy,
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
    //println!("{:#?}", get_account_activities(&config).await);
    //println!("{:?}", get_calendar(&config).await.unwrap());
    println!("{:#?}", get_asset(&config, "AAPL").await?);
    Ok(())
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    env_logger::init();

    match rt.block_on(run()) {
        Ok(()) => info!("All done!"),
        Err(e) => error!("Received error: {:?}", e),
    }
}
