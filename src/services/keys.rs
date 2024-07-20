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
use crate::parse_empty_response;
use crate::parse_response;
use crate::routes;
use crate::services::HttpService;

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
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    pub async fn create_key(
        &self,
        http: &HttpService,
        req: CreateKeyRequest,
    ) -> Result<CreateKeyResponse, HttpError> {
        let route = routes::CREATE_KEY.compile();

        parse_response(fetch!(http, route, req).await).await
    }

    /// Verifies an existing api key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    pub async fn verify_key(
        &self,
        http: &HttpService,
        req: VerifyKeyRequest,
    ) -> Result<VerifyKeyResponse, HttpError> {
        let route = routes::VERIFY_KEY.compile();

        parse_response(fetch!(http, route, req).await).await
    }

    /// Revokes an existing api key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    pub async fn revoke_key(
        &self,
        http: &HttpService,
        req: RevokeKeyRequest,
    ) -> Result<(), HttpError> {
        let route = routes::REVOKE_KEY.compile();

        parse_empty_response(fetch!(http, route, req).await).await
    }

    /// Updates an existing api key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    pub async fn update_key(
        &self,
        http: &HttpService,
        req: UpdateKeyRequest,
    ) -> Result<(), HttpError> {
        let route = routes::UPDATE_KEY.compile();

        parse_empty_response(fetch!(http, route, req).await).await
    }

    /// Gets details about an api key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    pub async fn get_key(
        &self,
        http: &HttpService,
        req: GetKeyRequest,
    ) -> Result<ApiKey, HttpError> {
        let mut route = routes::GET_KEY.compile();
        route.query_insert("keyId", &req.key_id);

        parse_response(fetch!(http, route).await).await
    }

    /// Updates the remaining verifications for a key.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A [`Result`] containing the response, or an error.
    ///
    /// # Errors
    /// The [`HttpError`], if one occurred.
    pub async fn update_remaining(
        &self,
        http: &HttpService,
        req: UpdateRemainingRequest,
    ) -> Result<UpdateRemainingResponse, HttpError> {
        let route = routes::UPDATE_REMAINING.compile();

        parse_response(fetch!(http, route, req).await).await
    }
}
