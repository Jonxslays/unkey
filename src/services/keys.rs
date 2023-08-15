use crate::{
    models::{CreateKeyRequest, CreateKeyResponse, VerifyKeyRequest, VerifyKeyResponse},
    routes,
    services::HttpService,
    unwind_response,
    types::Response,
};

#[allow(unused_imports)]
use crate::types::HttpError;

/// The service that handles key related requests.
#[derive(Debug, Clone)]
pub struct KeyService;

impl Default for KeyService {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyService {
    /// Creates a new key service.
    ///
    /// # Returns
    /// - [`Self`]: The new service.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Creates a new api key.
    ///
    /// # Arguments
    /// - `http`: The [`HttpService`] to use for the request.
    /// - `key`: The [`CreateKeyRequest`] to send.
    ///
    /// # Returns
    /// - [`Response<CreateKeyResponse>`]: A result containing
    ///     the [`CreateKeyResponse`], or an [`HttpError`].
    pub async fn create_key(
        &self,
        http: &HttpService,
        key: CreateKeyRequest,
    ) -> Response<CreateKeyResponse> {
        let route = routes::CREATE_KEY.compile();
        let response = http.fetch(route, Some(key)).await;

        unwind_response(response).await
    }

    /// Verifies an existing api key.
    ///
    /// # Arguments
    /// - `http`: The [`HttpService`] to use for the request.
    /// - `key`: The key to verify.
    ///
    /// # Returns
    /// - [`Response<VerifyKeyResponse>`]: A result containing
    ///     the [`VerifyKeyResponse`], or a [`HttpError`].
    pub async fn verify_key(&self, http: &HttpService, key: &str) -> Response<VerifyKeyResponse> {
        let route = routes::VERIFY_KEY.compile();
        let payload = VerifyKeyRequest::new(key);
        let response = http.fetch(route, Some(payload)).await;

        unwind_response(response).await
    }
}
