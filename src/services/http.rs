use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;

use crate::logging;
use crate::routes::CompiledRoute;
use crate::types::HttpResult;

// TODO: implement versioning at some point
/// The unkey api production base url.
static BASE_API_URL: &str = "https://api.unkey.dev/v1";

/// The http service used for handling requests.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct HttpService {
    /// The base url to use for requests.
    url: String,

    /// The request client to use for requests.
    client: reqwest::Client,

    /// The request headers to send with each request.
    headers: HeaderMap,
}

impl HttpService {
    /// Creates a new http service.
    ///
    /// # Arguments
    /// - `key`: The root api key to use.
    ///
    /// # Returns
    /// The new http service.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::services::HttpService;
    /// let s = HttpService::new("unkey_abds");
    /// ```
    #[must_use]
    #[rustfmt::skip]
    pub fn new(key: &str) -> Self {
        let headers = Self::generate_headers(key);
        let client = reqwest::Client::new();
        let url = BASE_API_URL.to_string();

        Self { url, client, headers }
    }

    /// Creates a new http service that does not use the production
    /// unkey api url.
    ///
    /// # Arguments
    /// - `key`: The root api key to use.
    /// - `url`: The base url to use.
    ///
    /// # Returns
    /// The new http service.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::services::HttpService;
    /// let s = HttpService::with_url("unkey_abds", "http://localhost:3000");
    /// ```
    #[must_use]
    #[rustfmt::skip]
    pub fn with_url(key: &str, url: &str) -> Self {
        let headers = Self::generate_headers(key);
        let client = reqwest::Client::new();
        let url = url.to_string();

        Self { url, client, headers }
    }

    /// Generates the headers to send with requests.
    ///
    /// # Arguments
    /// - `key`: The root api key to use.
    ///
    /// # Returns
    /// The header map to use.
    fn generate_headers(key: &str) -> HeaderMap {
        let mut headers = HeaderMap::with_capacity(3);
        let key = format!("Bearer {key}");
        let version = env!("CARGO_PKG_VERSION");
        let user_agent = format!("Unkey Rust SDK v{version}");

        let buffer: [(&'static str, Result<HeaderValue, _>); 3] = [
            ("Accept", HeaderValue::from_str("application/json")),
            ("x-user-agent", HeaderValue::from_str(&user_agent)),
            ("Authorization", HeaderValue::from_str(&key)),
        ];

        for (k, v) in &buffer {
            match v {
                Ok(h) => headers.insert(*k, h.clone()),
                Err(e) => {
                    eprintln!("Invalid header value: {e:?}");
                    std::process::exit(1);
                }
            };
        }

        headers
    }

    /// Updates the root api key to send with requests.
    ///
    /// # Arguments
    /// - `key`: The new root api key to use.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::services::HttpService;
    /// let mut s = HttpService::new("unkey_ghj");
    /// s.set_key("unkey_abc");
    /// ```
    pub fn set_key(&mut self, key: &str) {
        let header = HeaderValue::from_str(key);

        match header {
            Err(e) => eprintln!("Error setting header value: {e:?}"),
            Ok(h) => {
                self.headers.insert("Authorization", h);
            }
        }
    }

    /// Sets the base url to use for the api.
    ///
    /// # Arguments
    /// - `url`: The new api base url to use.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::services::HttpService;
    /// let mut s = HttpService::new("unkey_ghj");
    /// s.set_url("http://localhost:4000");
    /// ```
    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    /// Sends the http request.
    ///
    /// # Arguments
    /// - `route`: The compiled route to fetch.
    /// - `payload`: The optional json payload.
    ///
    /// # Returns
    /// The result of the http request.
    ///
    /// # Errors
    /// The reqwest error encountered during the request.
    ///
    /// # Example
    /// ```no_run
    /// # use unkey_sdk::services::HttpService;
    /// # use unkey_sdk::routes::Route;
    /// # use unkey_sdk::routes::CompiledRoute;
    /// # use unkey_sdk::models::CreateKeyRequest;
    /// # use reqwest::Method;
    /// # async fn f() {
    /// let r = Route::new(Method::GET, "/death/destroyer/worlds");
    /// let c = CompiledRoute::new(&r);
    /// let s = HttpService::new("unkey_ghj");
    ///
    /// let res = s.fetch::<CreateKeyRequest>(c, None).await;
    /// # }
    /// ```
    pub async fn fetch<T>(&self, route: CompiledRoute, payload: Option<T>) -> HttpResult
    where
        T: std::fmt::Debug + Serialize,
    {
        let query = route.build_query();
        let endpoint = route.uri.clone() + &query;
        logging::info!(format!("OUTGOING: {} {endpoint}", &route.method));

        let url = self.url.clone() + &endpoint;
        let mut req = self
            .client
            .request(route.method, url)
            .headers(self.headers.clone());

        if let Some(p) = payload {
            logging::debug!(format!("PAYLOAD : {p:?}"));
            req = req.json(&p);
        }

        req.send().await
    }
}
