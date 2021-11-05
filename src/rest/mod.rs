use rest_client::Client;

pub mod account;
pub mod account_activities;
pub mod account_configurations;
pub mod assets;
pub mod calendar;
pub mod clock;
pub mod orders;
pub mod positions;

pub fn paper_client(key: &str, secret: &str) -> Client {
    Client::new("https://paper-api.alpaca.markets").header_auth(vec![
        ("apca-api-key-id", key),
        ("apca-api-secret-key", secret),
    ])
}

pub fn live_client(key: &str, secret: &str) -> Client {
    Client::new("https://api.alpaca.markets").header_auth(vec![
        ("apca-api-key-id", key),
        ("apca-api-secret-key", secret),
    ])
}

pub fn client_with_url(url: &str, key: &str, secret: &str) -> Client {
    Client::new(url).header_auth(vec![
        ("apca-api-key-id", key),
        ("apca-api-secret-key", secret),
    ])
}
