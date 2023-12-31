use crate::fetch;
use crate::models::GetApiRequest;
use crate::models::GetApiResponse;
use crate::models::ListKeysRequest;
use crate::models::ListKeysResponse;
use crate::models::Wrapped;
use crate::routes;
use crate::services::HttpService;
use crate::wrap_response;

#[allow(unused_imports)]
use crate::models::HttpError;

/// The service that handles api related requests.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct ApiService;

impl ApiService {
    /// Retrieves a paginated list of keys for an api.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A wrapper around the response, or an [`HttpError`].
    pub async fn list_keys(
        &self,
        http: &HttpService,
        req: ListKeysRequest,
    ) -> Wrapped<ListKeysResponse> {
        let mut route = routes::LIST_KEYS.compile();
        route
            .query_insert("apiId", &req.api_id)
            .query_insert("limit", &req.limit.unwrap_or(100).to_string());

        if let Some(owner) = &req.owner_id {
            route.query_insert("ownerId", owner);
        }

        if let Some(cursor) = &req.cursor {
            route.query_insert("cursor", cursor);
        }

        wrap_response(fetch!(http, route).await).await
    }

    /// Retrieves api information.
    ///
    /// # Arguments
    /// - `http`: The http service to use for the request.
    /// - `req`: The request to send.
    ///
    /// # Returns
    /// A wrapper around the response, or an [`HttpError`].
    pub async fn get_api(&self, http: &HttpService, req: GetApiRequest) -> Wrapped<GetApiResponse> {
        let mut route = routes::GET_API.compile();
        route.query_insert("apiId", &req.api_id);

        wrap_response(fetch!(http, route).await).await
    }
}
