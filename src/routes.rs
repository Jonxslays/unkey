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

    pub fn compile(&self) -> CompiledRoute {
        CompiledRoute::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct CompiledRoute<'a> {
    pub uri: String,
    pub method: Method,
    pub params: Vec<(&'a str, String)>,
}

impl<'a> CompiledRoute<'a> {
    #[rustfmt::skip]
    pub fn new(route: &Route) -> Self {
        let params = Vec::new();
        let uri = route.uri.to_string();
        let method = route.method.clone();

        Self { method, params, uri }
    }

    /// Inserts the given param into the route uri.
    pub fn uri_insert(&mut self, param: impl ToString) -> &mut Self {
        self.uri = self.uri.replacen("{}", &param.to_string(), 1);
        self
    }

    /// Inserts a query param with the given name and value.
    pub fn query_insert(&mut self, name: &'a str, value: impl ToString) -> &mut Self {
        self.params.push((name, value.to_string()));
        self
    }

    /// Builds the query string for this route. i.e. `?a=b&c=d`.
    pub fn build_query(&self) -> String {
        let mut query = String::new();

        for (name, value) in &self.params {
            query.push(if query.is_empty() { '?' } else { '&' });
            query.push_str(&format!("{}={}", name, value));
        }

        query
    }
}
