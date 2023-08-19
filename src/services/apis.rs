use crate::routes;
use crate::unwind_response;

use crate::models::ListKeysRequest;
use crate::models::ListKeysResponse;
use crate::services::HttpService;
use crate::types::Response;

#[allow(unused_imports)]
use crate::types::HttpError;

/// The service that handles api related requests.
#[derive(Debug, Clone)]
pub struct ApiService;

impl ApiService {
    /// Retrieves a paginated list of keys for an api.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A result containing the response, or an [`HttpError`].
    pub async fn list_keys(
        &self,
        http: &HttpService,
        req: ListKeysRequest,
    ) -> Response<ListKeysResponse> {
        let mut route = routes::LIST_KEYS.compile();
        route
            .uri_insert(&req.api_id)
            .query_insert("limit", &req.limit.unwrap_or(100).to_string())
            .query_insert("offset", &req.offset.unwrap_or(0).to_string());

        if let Some(owner) = &req.owner_id {
            route.query_insert("ownerId", owner);
        }

        // We lie to the compiler about T here because there is no payload
        // TODO: improve DX here, maybe a macro?
        let response = http.fetch::<usize>(route, None).await;

        unwind_response(response).await
    }
}
