use crate::{
    models::{CreateKeyRequest, CreateKeyResponse, VerifyKeyResponse},
    services::{HttpService, KeyService},
    types::Response,
};

#[allow(unused_imports)]
use crate::types::HttpError;

#[derive(Debug, Clone)]
pub struct Client {
    http: HttpService,
    keys: KeyService,
}

impl Client {
    pub fn new(key: &str) -> Self {
        let http = HttpService::new(key);
        let keys = KeyService::new();

        Self { http, keys }
    }

    pub fn with_url(key: &str, url: &str) -> Self {
        let http = HttpService::with_url(key, url);
        let keys = KeyService::new();

        Self { http, keys }
    }

    pub fn set_key(&mut self, key: &str) {
        self.http.set_key(key)
    }

    pub fn set_url(&mut self, url: &str) {
        self.http.set_url(url)
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
    ///     Response::Ok(key) => println!("{:?}", key),
    ///     Response::Err(err) => println!("{:?}", err),
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
}
