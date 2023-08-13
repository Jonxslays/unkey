use crate::{models, routes};

use super::{unwind_response, HttpService, ServiceResult};

#[derive(Debug, Clone)]
pub struct KeyService;

impl KeyService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_key(
        &self,
        http: &HttpService,
        key: models::CreateKeyRequest,
    ) -> ServiceResult<models::CreateKeyResponse> {
        let route = routes::CREATE_KEY.compile();
        let response = http.fetch(route, Some(key)).await;

        unwind_response(response).await
    }

    pub async fn verify_key(
        &self,
        http: &HttpService,
        key: &str,
    ) -> ServiceResult<models::VerifyKeyResponse> {
        let route = routes::VERIFY_KEY.compile();
        let payload = models::VerifyKeyRequest::new(key.to_string());
        let response = http.fetch(route, Some(payload)).await;

        unwind_response(response).await
    }
}
