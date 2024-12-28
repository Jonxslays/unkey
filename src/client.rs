use crate::models::ApiKey;
use crate::models::CreateKeyRequest;
use crate::models::CreateKeyResponse;
use crate::models::DeleteApiRequest;
use crate::models::GetApiRequest;
use crate::models::GetApiResponse;
use crate::models::GetKeyRequest;
use crate::models::GetUsageNumbersRequest;
use crate::models::GetUsageNumbersResponse;
use crate::models::ListKeysRequest;
use crate::models::ListKeysResponse;
use crate::models::RevokeKeyRequest;
use crate::models::UpdateKeyRequest;
use crate::models::UpdateRemainingRequest;
use crate::models::UpdateRemainingResponse;
use crate::models::VerifyKeyRequest;
use crate::models::VerifyKeyResponse;
use crate::services::ApiService;
use crate::services::HttpService;
use crate::services::KeyService;

#[allow(unused_imports)]
use crate::models::HttpError;

/// The client used to make requests to the unkey api.
#[derive(Debug, Clone)]
pub struct Client {
    /// The internal http service sending and receiving requests.
    http: HttpService,

    /// The key service handling key related requests.
    keys: KeyService,

    /// The api service handling api related requests.
    apis: ApiService,
}

impl Client {
    /// Creates a new client.
    ///
    /// # Arguments
    /// - `key`: The root api key the client should send with requests.
    ///
    /// # Returns
    /// The new client.
    ///
    /// # Example
    /// ```
    /// # use unkey::Client;
    /// let c = Client::new("unkey_ghj");
    /// ```
    #[must_use]
    pub fn new(key: &str) -> Self {
        let http = HttpService::new(key);
        let keys = KeyService;
        let apis = ApiService;

        Self { http, keys, apis }
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
    /// The new client.
    ///
    /// # Example
    /// ```
    /// # use unkey::Client;
    /// let c = Client::with_url("unkey_ghj", "http://localhost:3000");
    /// ```
    #[must_use]
    pub fn with_url(key: &str, url: &str) -> Self {
        let http = HttpService::with_url(key, url);
        let keys = KeyService;
        let apis = ApiService;

        Self { http, keys, apis }
    }

    /// Updates the root api key for the client.
    ///
    /// # Arguments
    /// - `key`: The new root api key the client should send with requests.
    ///
    /// # Example
    /// ```
    /// # use unkey::Client;
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
    /// # use unkey::Client;
    /// let mut c = Client::new("unkey_ghj");
    /// c.set_url("http://localhost:6969");
    /// ```
    pub fn set_url(&mut self, url: &str) {
        self.http.set_url(url);
    }

    /// Verifies an existing api key.
    ///
    /// # Arguments
    /// - `req`: The verify key request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    ///
    /// # Example
    /// ```no_run
    /// # async fn verify() {
    /// # use unkey::Client;
    /// # use unkey::models::VerifyKeyRequest;
    /// let c = Client::new("abc123");
    /// let req = VerifyKeyRequest::new("test_KEYABC", "api_123123");
    ///
    /// match c.verify_key(req).await {
    ///     Ok(res) => println!("{:?}", res),
    ///     Err(err) => println!("{:?}", err),
    /// }
    /// # }
    /// ```
    pub async fn verify_key(&self, req: VerifyKeyRequest) -> Result<VerifyKeyResponse, HttpError> {
        self.keys.verify_key(&self.http, req).await
    }

    /// Creates a new api key.
    ///
    /// # Arguments
    /// - `req`: The create key request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    ///
    /// # Example
    /// ```no_run
    /// # async fn create() {
    /// # use unkey::Client;
    /// # use unkey::models::CreateKeyRequest;
    /// let c = Client::new("abc123");
    /// let req = CreateKeyRequest::new("api_CCC").set_remaining(100);
    ///
    /// match c.create_key(req).await {
    ///     Ok(res) => println!("{:?}", res),
    ///     Err(err) => println!("{:?}", err),
    /// }
    /// # }
    /// ```
    pub async fn create_key(&self, req: CreateKeyRequest) -> Result<CreateKeyResponse, HttpError> {
        self.keys.create_key(&self.http, req).await
    }

    /// Retrieves a paginated list of api keys.
    ///
    /// # Arguments
    /// - `req`: The list keys request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    ///
    /// # Example
    /// ```no_run
    /// # async fn list() {
    /// # use unkey::Client;
    /// # use unkey::models::ListKeysRequest;
    /// let c = Client::new("abc123");
    /// let req = ListKeysRequest::new("api_id").set_limit(25);
    ///
    /// match c.list_keys(req).await {
    ///     Ok(res) => println!("{:?}", res),
    ///     Err(err) => println!("{:?}", err),
    /// }
    /// # }
    /// ```
    pub async fn list_keys(&self, req: ListKeysRequest) -> Result<ListKeysResponse, HttpError> {
        self.apis.list_keys(&self.http, req).await
    }

    /// Revokes an existing api key.
    ///
    /// # Arguments
    /// - `req`: The revoke key request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    ///
    /// # Example
    /// ```no_run
    /// # async fn revoke() {
    /// # use unkey::Client;
    /// # use unkey::models::RevokeKeyRequest;
    /// let c = Client::new("abc123");
    /// let req = RevokeKeyRequest::new("test_123");
    ///
    /// match c.revoke_key(req).await {
    ///     Ok(_) => println!("Success!"), // Nothing on success
    ///     Err(err) => println!("{:?}", err),
    /// }
    /// # }
    /// ```
    pub async fn revoke_key(&self, req: RevokeKeyRequest) -> Result<(), HttpError> {
        self.keys.revoke_key(&self.http, req).await
    }

    /// Retrieves information for the given api id.
    ///
    /// # Arguments
    /// - `req`: The get api request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    ///
    /// # Example
    /// ```no_run
    /// # async fn get() {
    /// # use unkey::Client;
    /// # use unkey::models::GetApiRequest;
    /// let c = Client::new("abc123");
    /// let req = GetApiRequest::new("api_id");
    ///
    /// match c.get_api(req).await {
    ///     Ok(res) => println!("{:?}", res),
    ///     Err(err) => println!("{:?}", err),
    /// }
    /// # }
    /// ````
    pub async fn get_api(&self, req: GetApiRequest) -> Result<GetApiResponse, HttpError> {
        self.apis.get_api(&self.http, req).await
    }

    /// Permanently deletes an api and revokes all keys associated with it.
    ///
    /// # Arguments
    ///
    /// - `req`: The delete api request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    ///
    /// # Example
    /// ```no_run
    /// # async fn delete() {
    /// # use unkey::Client;
    /// # use unkey::models::DeleteApiRequest;
    /// let c = Client::new("abc123");
    /// let req = DeleteApiRequest::new("api_id");
    ///
    /// match c.delete_api(req).await {
    ///    Ok(res) => println!("{:?}", res),
    ///    Err(err) => println!("{:?}", err),
    /// }
    /// # }
    /// ````
    pub async fn delete_api(&self, req: DeleteApiRequest) -> Result<(), HttpError> {
        self.apis.delete_api(&self.http, req).await
    }

    /// Retrieves information for the given api id.
    ///
    /// # Arguments
    /// - `req`: The get api request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    ///
    /// # Example
    /// ```no_run
    /// # async fn get() {
    /// # use unkey::Client;
    /// # use unkey::models::UpdateKeyRequest;
    /// let c = Client::new("abc123");
    /// let req = UpdateKeyRequest::new("api_id").set_remaining(Some(100));
    ///
    /// match c.update_key(req).await {
    ///     Ok(_) => println!("Success"), // Nothing on success
    ///     Err(err) => println!("{:?}", err),
    /// }
    /// # }
    /// ````
    pub async fn update_key(&self, req: UpdateKeyRequest) -> Result<(), HttpError> {
        self.keys.update_key(&self.http, req).await
    }

    /// Retrieves information for the given api id.
    ///
    /// # Arguments
    /// - `req`: The get key request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    ///
    /// # Example
    /// ```no_run
    /// # async fn get() {
    /// # use unkey::Client;
    /// # use unkey::models::GetKeyRequest;
    /// let c = Client::new("abc123");
    /// let req = GetKeyRequest::new("key_id");
    ///
    /// match c.get_key(req).await {
    ///     Ok(res) => println!("{:?}", res),
    ///     Err(err) => println!("{:?}", err),
    /// }
    /// # }
    /// ````
    pub async fn get_key(&self, req: GetKeyRequest) -> Result<ApiKey, HttpError> {
        self.keys.get_key(&self.http, req).await
    }

    /// Update the remaining verifications for a key.
    ///
    /// # Arguments
    /// - `req`: The update remaining request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    ///
    /// # Example
    /// ```no_run
    /// # async fn get() {
    /// # use unkey::Client;
    /// # use unkey::models::UpdateRemainingRequest;
    /// # use unkey::models::UpdateOp;
    /// let c = Client::new("abc123");
    /// let req = UpdateRemainingRequest::new("key_id", Some(100), UpdateOp::Set);
    ///
    /// match c.update_remaining(req).await {
    ///     Ok(res) => println!("{:?}", res),
    ///     Err(err) => println!("{:?}", err),
    /// }
    /// # }
    /// ````
    pub async fn update_remaining(
        &self,
        req: UpdateRemainingRequest,
    ) -> Result<UpdateRemainingResponse, HttpError> {
        self.keys.update_remaining(&self.http, req).await
    }

    /// Retrieves usage numbers for a key.
    ///
    /// # Arguments
    /// - `req`: The get usage numbers request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    ///
    pub async fn get_verifications(
        &self,
        req: GetUsageNumbersRequest,
    ) -> Result<GetUsageNumbersResponse, HttpError> {
        self.keys.get_verifications(&self.http, req).await
    }
}

#[cfg(test)]
mod test {
    use crate::services::ApiService;
    use crate::services::KeyService;
    use crate::Client;

    #[test]
    fn new() {
        let c = Client::new("");

        assert_eq!(c.apis, ApiService);
        assert_eq!(c.keys, KeyService);
    }
}
