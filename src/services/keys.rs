use crate::{models, routes};

use super::{HttpService, ServiceResult};

#[derive(Debug, Clone)]
pub struct KeyService;

impl KeyService {
    pub fn new() -> Self {
        Self
    }

    pub async fn verify_key<T: ToString>(
        http: &HttpService,
        key: T,
    ) -> ServiceResult<models::VerifyKeyResponse> {
        let route = routes::VERIFY_KEY.compile();
        let payload = models::VerifyKeyRequest::new(key.to_string());
        let response = http.fetch(route, Some(payload)).await;

        if response.is_err() {
            return Err(models::HttpError::new(
                models::ErrorCode::Unknown,
                response.unwrap_err().to_string(),
            ));
        }

        todo!() // This is not idiomatic, rethink the models
    }
}
