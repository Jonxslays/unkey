use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;

use crate::routes::CompiledRoute;

// TODO: implement versioning at some point
static BASE_API_URL: &'static str = "https://api.unkey.dev/v1";

pub type HttpResult = Result<reqwest::Response, reqwest::Error>;

#[derive(Debug, Clone)]
pub struct HttpService {
    url: String,
    client: reqwest::Client,
    headers: HeaderMap,
}

impl HttpService {
    #[rustfmt::skip]
    pub fn new(key: &str) -> Self {
        let headers = Self::generate_headers(key);
        let client = reqwest::Client::new();
        let url = BASE_API_URL.to_string();

        Self { url, client, headers }
    }

    #[rustfmt::skip]
    pub fn with_url(key: &str, url: &str) -> Self {
        let headers = Self::generate_headers(key);
        let client = reqwest::Client::new();
        let url = url.to_string();

        Self { url, client, headers }
    }

    fn generate_headers(key: &str) -> HeaderMap {
        let mut headers = HeaderMap::with_capacity(3);
        let version = env!("CARGO_PKG_VERSION");
        let user_agent = format!("Unkey Rust SDK v{}", version);
        let key = String::from("Bearer ") + key;

        headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());
        headers.insert("x-user-agent", HeaderValue::from_str(&user_agent).unwrap());
        headers.insert("Authorization", HeaderValue::from_str(&key).unwrap());

        headers
    }

    pub fn set_key(&mut self, key: &str) {
        let value = HeaderValue::from_str(key).unwrap();
        self.headers.insert("Authorization", value);
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    pub async fn fetch<T: Serialize>(
        &self,
        route: CompiledRoute,
        payload: Option<T>,
    ) -> HttpResult {
        let url = self.url.clone() + &route.uri;
        let mut req = self
            .client
            .request(route.method, url)
            .headers(self.headers.clone());

        if let Some(p) = payload {
            req = req.json(&p);
        }

        req.send().await
    }
}
