use super::services;

#[derive(Debug, Clone)]
pub struct Client {
    http: services::HttpService,
    pub keys: services::KeyService,
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
}
