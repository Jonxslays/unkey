use crate::{
    models::{ListKeysRequest, ListKeysResponse},
    routes::{self},
    services::HttpService,
    types::Response,
    unwind_response,
};

#[allow(unused_imports)]
use crate::types::HttpError;

/// The service that handles api related requests.
#[derive(Debug, Clone)]
pub struct ApiService;

impl Default for ApiService {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiService {
    /// Creates a new api service.
    ///
    /// # Returns
    /// - [`Self`]: The new service.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Lists all api keys.
    ///
    /// # Arguments
    /// - `http`: The [`HttpService`] to use for the request.
    /// - `request`: The [`ListKeysRequest`] to send.
    ///
    /// # Returns
    /// - [`Response<ListKeysResponse`]: A result containing the [`ListKeysRepsonse`], or an [`HttpError`].
    pub async fn list_keys(
        &self,
        http: &HttpService,
        request: ListKeysRequest,
    ) -> Response<ListKeysResponse> {
        let mut route = routes::LIST_KEYS.compile();
        route
            .uri_insert(&request.api_id)
            .query_insert("limit", &request.limit.unwrap_or(100).to_string())
            .query_insert("offset", &request.offset.unwrap_or(0).to_string());
        if let Some(owner) = &request.owner_id {
            route.query_insert("ownerId", owner);
        }
        // We lie to the compiler about T here because there is no payload
        // TODO: improve DX here, maybe a macro?
        let response = http.fetch::<usize>(route, None).await;

        unwind_response(response).await
    }
}
