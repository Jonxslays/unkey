use crate::fetch;
use crate::models::ApiKey;
use crate::models::CreateKeyRequest;
use crate::models::CreateKeyResponse;
use crate::models::GetKeyRequest;
use crate::models::RevokeKeyRequest;
use crate::models::UpdateKeyRequest;
use crate::models::UpdateRemainingRequest;
use crate::models::UpdateRemainingResponse;
use crate::models::VerifyKeyRequest;
use crate::models::VerifyKeyResponse;
use crate::models::Wrapped;
use crate::routes;
use crate::services::HttpService;
use crate::wrap_empty_response;
use crate::wrap_response;

#[allow(unused_imports)]
use crate::models::HttpError;

/// The service that handles key related requests.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct KeyService;

impl KeyService {
    /// Creates a new api key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A wrapper around the response, or an [`HttpError`].
    pub async fn create_key(
        &self,
        http: &HttpService,
        req: CreateKeyRequest,
    ) -> Wrapped<CreateKeyResponse> {
        let route = routes::CREATE_KEY.compile();

        wrap_response(fetch!(http, route, req).await).await
    }

    /// Verifies an existing api key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A wrapper around the response, or an [`HttpError`].
    pub async fn verify_key(
        &self,
        http: &HttpService,
        req: VerifyKeyRequest,
    ) -> Wrapped<VerifyKeyResponse> {
        let route = routes::VERIFY_KEY.compile();

        wrap_response(fetch!(http, route, req).await).await
    }

    /// Revokes an existing api key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A wrapper around an empty response, or an [`HttpError`].
    pub async fn revoke_key(&self, http: &HttpService, req: RevokeKeyRequest) -> Wrapped<()> {
        let route = routes::REVOKE_KEY.compile();

        wrap_empty_response(fetch!(http, route, req).await).await
    }

    /// Updates an existing api key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A wrapper around an empty response, or an [`HttpError`].
    pub async fn update_key(&self, http: &HttpService, req: UpdateKeyRequest) -> Wrapped<()> {
        let route = routes::UPDATE_KEY.compile();

        wrap_empty_response(fetch!(http, route, req).await).await
    }

    /// Gets details about an api key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A wrapper around the response, or an [`HttpError`].
    pub async fn get_key(&self, http: &HttpService, req: GetKeyRequest) -> Wrapped<ApiKey> {
        let mut route = routes::GET_KEY.compile();
        route.query_insert("keyId", &req.key_id);

        wrap_response(fetch!(http, route).await).await
    }

    /// Updates the remaining verifications for a key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A wrapper around the response, or an [`HttpError`].
    pub async fn update_remaining(
        &self,
        http: &HttpService,
        req: UpdateRemainingRequest,
    ) -> Wrapped<UpdateRemainingResponse> {
        let route = routes::UPDATE_REMAINING.compile();

        wrap_response(fetch!(http, route, req).await).await
    }
}
