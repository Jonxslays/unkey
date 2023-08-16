

use crate::{
    models::{ListKeysRequest, ListKeysResponse},
    routes::{self, CompiledRoute},
    services::HttpService,
    unwind_response,
    types::Response,
};

#[allow(unused_imports)]
use crate::types::HttpError;

#[derive(Debug, Clone)]
pub struct ApiService;

impl Default for ApiService{
    fn default() -> Self {
        Self::new()
    }
}

impl ApiService{

    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub async fn list_keys(
        &self,
        http: &HttpService,
        request: ListKeysRequest,
    ) -> Response<ListKeysResponse>{
        // let route: CompiledRoute = routes::LIST_KEYS.compile();
        let mut route: CompiledRoute = routes::LIST_KEYS.compile();
        route.uri_insert(request.api_id.clone());
      
        let response = http.fetch(route, Some(request)).await;

        let a = response.as_ref().clone();
        
        // println!("{:?}",(a));

        unwind_response(response).await
    }
}
