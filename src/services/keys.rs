use crate::routes;
use crate::unwind_response;

use crate::models::CreateKeyRequest;
use crate::models::CreateKeyResponse;
use crate::models::VerifyKeyRequest;
use crate::models::VerifyKeyResponse;
use crate::services::HttpService;
use crate::types::Response;

#[allow(unused_imports)]
use crate::types::HttpError;

/// The service that handles key related requests.
#[derive(Debug, Clone)]
pub struct KeyService;

impl KeyService {
    /// Creates a new api key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A result containing the response, or an [`HttpError`].
    pub async fn create_key(
        &self,
        http: &HttpService,
        req: CreateKeyRequest,
    ) -> Response<CreateKeyResponse> {
        let route = routes::CREATE_KEY.compile();
        let response = http.fetch(route, Some(req)).await;

        unwind_response(response).await
    }

    /// Verifies an existing api key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A result containing the response, or an [`HttpError`].
    pub async fn verify_key(
        &self,
        http: &HttpService,
        req: VerifyKeyRequest,
    ) -> Response<VerifyKeyResponse> {
        let route = routes::VERIFY_KEY.compile();
        let response = http.fetch(route, Some(req)).await;

        unwind_response(response).await
    }
}
