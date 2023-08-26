use crate::fetch;
use crate::models::CreateKeyRequest;
use crate::models::CreateKeyResponse;
use crate::models::RevokeKeyRequest;
use crate::models::UpdateKeyRequest;
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
#[derive(Debug, Clone)]
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
        let mut route = routes::REVOKE_KEY.compile();
        route.uri_insert(&req.key_id);

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
        let mut route = routes::UPDATE_KEY.compile();
        route.uri_insert(&req.key_id);

        wrap_empty_response(fetch!(http, route, req).await).await
    }
}
