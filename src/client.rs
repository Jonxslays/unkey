use super::services;

#[derive(Debug, Clone)]
pub struct Client {
    http: services::HttpService,
}

impl Client {
    pub fn new(key: &str) -> Self {
        let http = services::HttpService::new(key);

        Self { http }
    }

    pub fn with_url(key: &str, url: &str) -> Self {
        let http = services::HttpService::with_url(key, url);

        Self { http }
    }

    pub fn set_key(&mut self, key: &str) {
        self.http.set_key(key)
    }

    pub fn set_url(&mut self, url: &str) {
        self.http.set_url(url)
    }
}
