use crate::{models, routes};

use super::{HttpService, ServiceResult, unwind_response};

#[derive(Debug, Clone)]
pub struct KeyService;

impl KeyService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_key() {}

    pub async fn verify_key<T: ToString>(
        &self,
        http: &HttpService,
        key: T,
    ) -> ServiceResult<models::VerifyKeyResponse> {
        let route = routes::VERIFY_KEY.compile();
        let payload = models::VerifyKeyRequest::new(key.to_string());
        let response = http.fetch(route, Some(payload)).await;

        unwind_response(response).await
    }
}
