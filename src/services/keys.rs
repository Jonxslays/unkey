use crate::fetch;
use crate::routes;
use crate::wrap_response;

use crate::models::CreateKeyRequest;
use crate::models::CreateKeyResponse;
use crate::models::VerifyKeyRequest;
use crate::models::VerifyKeyResponse;
use crate::models::Wrapped;
use crate::services::HttpService;

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
}
