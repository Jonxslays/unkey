use std::collections::HashMap;

use reqwest::Method;

// Keys
pub static CREATE_KEY: Route = Route::new(Method::POST, "/keys");
pub static VERIFY_KEY: Route = Route::new(Method::POST, "/keys/verify");
pub static DELETE_KEY: Route = Route::new(Method::DELETE, "/keys/{}");
pub static UPDATE_KEY: Route = Route::new(Method::PUT, "/keys/{}");

// Apis
pub static GET_API: Route = Route::new(Method::GET, "/apis/{}");
pub static LIST_KEYS: Route = Route::new(Method::GET, "/apis/{}/keys");

#[derive(Debug, Clone)]
pub struct Route {
    pub method: Method,
    pub uri: &'static str,
}

impl Route {
    pub const fn new(method: Method, uri: &'static str) -> Self {
        Self { method, uri }
    }

    pub fn compile_empty(&self) -> CompiledRoute {
        self.compile(&[])
    }

    pub fn compile(&self, params: &[&str]) -> CompiledRoute {
        let mut compiled = CompiledRoute::new(self.clone());

        for param in params {
            compiled.uri = compiled.uri.replacen("{}", param, 1);
        }

        compiled
    }
}

#[derive(Debug, Clone)]
pub struct CompiledRoute {
    pub route: Route,
    pub uri: String,
    pub params: HashMap<String, String>,
}

impl CompiledRoute {
    pub fn new(route: Route) -> Self {
        let params = HashMap::new();
        let uri = route.uri.to_string();
        Self { route, uri, params }
    }
}
