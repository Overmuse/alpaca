use alpaca::{assets::*, orders::*, Client};
use log::{error, info};

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;

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
    //println!("{:#?}", submit_order(&client, &o).await);
    //println!("{:#?}", &serde_json::to_string(&o).unwrap());
    //println!("{:#?}", get_account_activities(&client).await);
    //println!("{:?}", get_calendar(&client).await.unwrap());
    //println!("{:#?}", cancel_all_orders(&client).await?);
    //println!("{:#?}", get_orders(&client).await?);
    Ok(())
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(run()) {
        Ok(()) => info!("All done!"),
        Err(e) => error!("Received error: {:?}", e),
    }
}
