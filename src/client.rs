use crate::{
    models::{CreateKeyRequest, CreateKeyResponse, VerifyKeyResponse, ListKeysRequest, ListKeysResponse},
    services::{HttpService, KeyService, ApiService},
    types::Response,
};

#[allow(unused_imports)]
use crate::types::HttpError;

/// The client used to make requests to the unkey api.
#[derive(Debug, Clone)]
pub struct Client {
    /// The internal http service handling requests.
    http: HttpService,

    /// The key service handling key related serialization and deserialization.
    keys: KeyService,

    api: ApiService,
}

impl Client {
    /// Creates a new client.
    ///
    /// # Arguments
    /// - `key`: The root api key the client should send with requests.
    ///
    /// # Returns
    /// - [`Self`]: The new client.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::Client;
    /// let c = Client::new("unkey_ghj");
    /// ```
    #[must_use]
    pub fn new(key: &str) -> Self {
        let http = HttpService::new(key);
        let keys = KeyService::new();
        let api = ApiService::new();

        Self { http, keys, api }
    }

    /// Creates a new client with a different base url than the production
    /// unkey api url.
    ///
    /// # Arguments
    /// - `key`: The root api key the client should send with requests.
    /// - `url`: The base url to use, excluding trailing slash.
    ///     i.e. `http://localhost:3000`.
    ///
    /// # Returns
    /// - [`Self`]: The new client.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::Client;
    /// let c = Client::with_url("unkey_ghj", "http://localhost:3000");
    /// ```
    #[must_use]
    pub fn with_url(key: &str, url: &str) -> Self {
        let http = HttpService::with_url(key, url);
        let keys = KeyService::new();
        let api = ApiService::new();

        Self { http, keys, api }
    }

    /// Updates the root api key for the client.
    ///
    /// # Arguments
    /// - `key`: The new root api key the client should send with requests.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::Client;
    /// let mut c = Client::new("unkey_ghj");
    /// c.set_key("unkey_abc");
    /// ```
    pub fn set_key(&mut self, key: &str) {
        self.http.set_key(key);
    }

    /// Sets the url the client will send requests to.
    ///
    /// # Arguments
    /// - `url`: The new base url to use.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::Client;
    /// let mut c = Client::new("unkey_ghj");
    /// c.set_url("http://localhost:6969");
    /// ```
    pub fn set_url(&mut self, url: &str) {
        self.http.set_url(url);
    }

    /// Verifies an existing api key.
    ///
    /// # Arguments
    /// - `key`: The key to verify.
    ///
    /// # Returns
    /// - [`Response<VerifyKeyResponse>`]: A result containing
    ///     the [`VerifyKeyResponse`], or an [`HttpError`].
    ///
    /// # Example
    /// ```no_run
    /// # async fn verify() {
    /// # use unkey_sdk::Client;
    /// # use unkey_sdk::models::VerifyKeyRequest;
    /// # use unkey_sdk::types::Response;
    /// let c = Client::new("abc123");
    /// let key = "test_abc123";
    ///
    /// match c.verify_key(key).await {
    ///     Response::Ok(v) => println!("{:?}", v),
    ///     Response::Err(e) => println!("{:?}", e),
    /// }
    /// # }
    /// ```
    pub async fn verify_key(&self, key: &str) -> Response<VerifyKeyResponse> {
        self.keys.verify_key(&self.http, key).await
    }

    /// Creates a new api key.
    ///
    /// # Arguments
    /// - `key`: The [`CreateKeyRequest`] to send.
    ///
    /// # Returns
    /// - [`Response<CreateKeyResponse>`]: A result containing
    ///     the [`CreateKeyResponse`], or an [`HttpError`].
    ///
    /// # Example
    /// ```no_run
    /// # async fn create() {
    /// # use unkey_sdk::Client;
    /// # use unkey_sdk::models::CreateKeyRequest;
    /// # use unkey_sdk::types::Response;
    /// let c = Client::new("abc123");
    /// let req = CreateKeyRequest::new("api_CCC").set_remaining(100);
    ///
    /// match c.create_key(req).await {
    ///     Response::Ok(key) => println!("{:?}", key),
    ///     Response::Err(err) => println!("{:?}", err),
    /// }
    /// # }
    /// ```
    pub async fn create_key(&self, key: CreateKeyRequest) -> Response<CreateKeyResponse> {
        self.keys.create_key(&self.http, key).await
    }


    pub async fn list_keys(&self, request: ListKeysRequest) -> Response<ListKeysResponse>{
        self.api.list_keys(&self.http, request).await
    }

}
