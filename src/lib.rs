use crate::errors::Result;
use futures::future::TryFutureExt;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client as ReqwestClient, Method, RequestBuilder,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::env;

pub mod account;
pub mod account_activities;
pub mod account_configurations;
pub mod assets;
pub mod calendar;
pub mod clock;
pub mod errors;
pub mod orders;
pub mod positions;
pub mod stream;
mod utils;

pub enum RequestBody<T> {
    None,
    Query(T),
    Json(T),
}

impl<T> Default for RequestBody<T> {
    fn default() -> Self {
        RequestBody::None
    }
}

pub trait Request {
    type Body: Serialize;
    type Response: for<'de> Deserialize<'de>;
    const METHOD: Method = Method::GET;

    fn endpoint(&self) -> Cow<str>;

    fn headers(&self) -> HeaderMap {
        Default::default()
    }

    fn body(&self) -> RequestBody<&Self::Body> {
        Default::default()
    }
}

trait RequestBuilderExt: Sized {
    fn apca_body<T: Serialize>(self, body: RequestBody<T>) -> Self;
}

impl RequestBuilderExt for RequestBuilder {
    fn apca_body<T: Serialize>(self, body: RequestBody<T>) -> Self {
        match body {
            RequestBody::None => self,
            RequestBody::Json(value) => self.json(&value),
            RequestBody::Query(value) => self.query(&value),
        }
    }
}

/// The main client used for making request to Alpaca.
///
/// `AlpacaConfig` stores an async Reqwest client as well as the associate
/// base url for the Alpaca server.
pub struct Client {
    /// The underlying Reqwest client used for requests.
    inner: ReqwestClient,
    /// The url to which the request are sent.
    url: String,
}

impl Client {
    /// Create a new `Client`.
    pub fn new(url: String, key_id: String, secret_key: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_lowercase(b"apca-api-key-id").unwrap(),
            HeaderValue::from_str(&key_id).unwrap(),
        );
        headers.insert(
            HeaderName::from_lowercase(b"apca-api-secret-key").unwrap(),
            HeaderValue::from_str(&secret_key).unwrap(),
        );

        let inner = ReqwestClient::builder().default_headers(headers).build()?;

        Ok(Self { inner, url })
    }

    /// Creates a `Client` from environment variables.
    ///
    /// The three environment variables used to instantiate the struct are:
    /// - `APCA_API_BASE_URL`
    /// - `APCA_API_KEY_ID`
    /// - `APCA_API_SECRET_KEY`
    pub fn from_env() -> Result<Self> {
        let url = env::var("APCA_API_BASE_URL")?;
        let key_id = env::var("APCA_API_KEY_ID")?;
        let secret_key = env::var("APCA_API_SECRET_KEY")?;
        Self::new(url, key_id, secret_key)
    }

    pub async fn send<R: Request>(&self, request: R) -> Result<R::Response> {
        let endpoint = request.endpoint();
        let endpoint = endpoint.trim_matches('/');
        let url = format!("{}/{}", self.url, endpoint);

        self.inner
            .request(R::METHOD, &url)
            .headers(request.headers())
            .apca_body(request.body())
            .send()
            .and_then(|res| res.json())
            .map_err(From::from)
            .await
    }
}
