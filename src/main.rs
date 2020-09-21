use alpaca::{assets::*, orders::*, AlpacaConfig};
use log::{error, info};

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = AlpacaConfig::from_env()?;

    let o = OrderIntent {
        symbol: "AAPL".to_string(),
        qty: 1,
        side: alpaca::orders::Side::Buy,
        order_type: OrderType::TrailingStop {
            trail_price: Some(1.0),
            trail_percent: None,
        },
        time_in_force: TimeInForce::GTC,
        extended_hours: false,
        client_order_id: Some("TEST".to_string()),
        order_class: OrderClass::Simple,
    };
    println!("{:#?}", submit_order(&config, &o).await);
    //println!("{:#?}", &serde_json::to_string(&o).unwrap());
    //println!("{:#?}", get_account_activities(&config).await);
    //println!("{:?}", get_calendar(&config).await.unwrap());
    //println!("{:#?}", cancel_all_orders(&config).await?);
    //println!("{:#?}", get_orders(&config).await?);
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
