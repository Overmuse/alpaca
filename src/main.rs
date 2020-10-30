use alpaca::{
    orders::{OrderIntent, OrderType, Side, SubmitOrder, TimeInForce},
    Client,
};

#[tokio::main]
async fn main() {
    let client = Client::from_env().unwrap();
    let oi = OrderIntent::new("V")
        .qty(54)
        .side(Side::Buy)
        .order_type(OrderType::Limit { limit_price: 100.0 })
        .extended_hours(true)
        .time_in_force(TimeInForce::DAY);
    let res = client.send(SubmitOrder(oi)).await.unwrap();
    println!("{:#?}", res)
}
