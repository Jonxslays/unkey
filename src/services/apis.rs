use serde_json::Value;

use crate::{
    models::{ListKeysRequest, ListKeysResponse},
    routes::{self, CompiledRoute},
    services::HttpService,
    types::Response,
    unwind_response,
};

#[allow(unused_imports)]
use crate::types::HttpError;

#[derive(Debug, Clone)]
pub struct ApiService;

impl Default for ApiService {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiService {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub async fn list_keys(
        &self,
        http: &HttpService,
        request: ListKeysRequest,
    ) -> Response<ListKeysResponse> {
        let mut route: CompiledRoute = routes::LIST_KEYS.compile();
        route.uri_insert(&request.api_id);

        // The default values are from API spec docs
        route.query_insert("limit", &request.limit.unwrap_or(100).to_string());
        route.query_insert("offset", &request.offset.unwrap_or(0).to_string());

        match &request.owner_id {
            Some(o) => {
                route.query_insert("ownerId", o);
            }
            None => {}
        }

        let response = http.fetch(route, None::<Value>).await;

        unwind_response(response).await
    }
}
