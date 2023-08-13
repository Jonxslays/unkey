use super::{models, services};

#[derive(Debug, Clone)]
pub struct Client {
    http: services::HttpService,
    keys: services::KeyService,
}

impl Client {
    pub fn new(key: &str) -> Self {
        let http = services::HttpService::new(key);
        let keys = services::KeyService::new();

        Self { http, keys }
    }

    pub fn with_url(key: &str, url: &str) -> Self {
        let http = services::HttpService::with_url(key, url);
        let keys = services::KeyService::new();

        Self { http, keys }
    }

    pub fn set_key(&mut self, key: &str) {
        self.http.set_key(key)
    }

    pub fn set_url(&mut self, url: &str) {
        self.http.set_url(url)
    }

    pub async fn verify_key(
        &self,
        key: &str,
    ) -> services::ServiceResult<models::VerifyKeyResponse> {
        self.keys.verify_key(&self.http, key).await
    }

    pub async fn create_key(
        &self,
        key: models::CreateKeyRequest,
    ) -> services::ServiceResult<models::CreateKeyResponse> {
        self.keys.create_key(&self.http, key).await
    }
}
