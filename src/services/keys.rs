use crate::{models, routes};

use super::{unwind_response, HttpService, ServiceResult};

/// The service that handles key related requests.
#[derive(Debug, Clone)]
pub struct KeyService;

impl KeyService {
    /// Creates a new key service.
    ///
    /// # Returns
    /// - [`Self`]: The new service.
    pub fn new() -> Self {
        Self
    }

    /// Creates a new api key.
    ///
    /// # Arguments
    /// - `http`: The [`HttpService`] to use for the request.
    /// - `key`: The [`models::CreateKeyRequest`] to send.
    ///
    /// # Returns
    /// - [`ServiceResult<CreateKeyResponse>`]: A result containing
    ///     the [`models::CreateKeyResponse`], or a [`models::HttpError`].
    pub async fn create_key(
        &self,
        http: &HttpService,
        key: models::CreateKeyRequest,
    ) -> ServiceResult<models::CreateKeyResponse> {
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
    /// - [`ServiceResult<VerifyKeyResponse>`]: A result containing
    ///     the [`models::VerifyKeyResponse`], or a [`models::HttpError`].
    pub async fn verify_key(
        &self,
        http: &HttpService,
        key: &str,
    ) -> ServiceResult<models::VerifyKeyResponse> {
        let route = routes::VERIFY_KEY.compile();
        let payload = models::VerifyKeyRequest::new(key);
        let response = http.fetch(route, Some(payload)).await;

        unwind_response(response).await
    }
}
