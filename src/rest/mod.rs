use rest_client::Client;

pub mod account;
pub mod account_activities;
pub mod account_configurations;
pub mod assets;
pub mod calendar;
pub mod clock;
pub mod orders;
pub mod positions;

pub fn paper_client<'a>(key: &'static str, secret: &'static str) -> Client<'a> {
    Client::new("https://paper-api.alpaca.markets").header_auth(vec![
        ("apca-api-key-id", key),
        ("apca-api-secret-key", secret),
    ])
}

pub fn live_client<'a>(key: &'static str, secret: &'static str) -> Client<'a> {
    Client::new("https://api.alpaca.markets").header_auth(vec![
        ("apca-api-key-id", key),
        ("apca-api-secret-key", secret),
    ])
}

pub fn client_with_url<'a>(url: &'a str, key: &'static str, secret: &'static str) -> Client<'a> {
    Client::new(url).header_auth(vec![
        ("apca-api-key-id", key),
        ("apca-api-secret-key", secret),
    ])
}
