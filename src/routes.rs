use reqwest::Method;

////////////////////////////////////////////////////////////////////////////////
// ROUTES
////////////////////////////////////////////////////////////////////////////////

/// The create key endpoint `POST /keys`
pub static CREATE_KEY: Route = Route::new(Method::POST, "/keys");

/// The verify key endpoint `POST /keys/verify`
pub static VERIFY_KEY: Route = Route::new(Method::POST, "/keys/verify");

/// The delete key endpoint `DELETE /keys/{id}`
pub static DELETE_KEY: Route = Route::new(Method::DELETE, "/keys/{}");

/// The update key endpoint `PUT /keys/{id}`
pub static UPDATE_KEY: Route = Route::new(Method::PUT, "/keys/{}");

////////////////////////////////////////////////////////////////////////////////

/// The get api endpoint `GET /apis/{id}`
pub static GET_API: Route = Route::new(Method::GET, "/apis/{}");

/// The list keys endpoint `GET /apis/{id}/keys`
pub static LIST_KEYS: Route = Route::new(Method::GET, "/apis/{}/keys");

////////////////////////////////////////////////////////////////////////////////
// END ROUTES
////////////////////////////////////////////////////////////////////////////////

/// A route mapping to an unkey api endpoint.
#[derive(Debug, Clone)]
pub struct Route {
    /// The http method for the route.
    pub method: Method,

    /// The routes uri.
    pub uri: &'static str,
}

impl Route {
    /// Creates a new route.
    ///
    /// # Note
    /// These should really only be created internally by the library.
    ///
    /// # Arguments
    /// - `method`: The http [`Method`] for the route.
    /// - `uri`: The routes uri.
    ///
    /// # Returns
    /// - [`Self`]: The new route.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::routes::Route;
    /// # use reqwest::Method;
    /// let r = Route::new(Method::GET, "/keys/owo");
    ///
    /// assert_eq!(r.method, Method::GET);
    /// assert_eq!(r.uri, "/keys/owo");
    /// ```
    pub const fn new(method: Method, uri: &'static str) -> Self {
        Self { method, uri }
    }

    /// Compiles the properties of this static route into a new object.
    ///
    /// # Returns
    /// - [`CompiledRoute`]: The compiled route.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::routes::Route;
    /// # use reqwest::Method;
    /// let r = Route::new(Method::GET, "/apis/woot").compile();
    ///
    /// assert_eq!(r.params, vec![]);
    /// assert_eq!(r.method, Method::GET);
    /// assert_eq!(r.uri, String::from("/apis/woot"));
    /// ```
    pub fn compile(&self) -> CompiledRoute {
        CompiledRoute::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct CompiledRoute {
    /// The routes uri.
    pub uri: String,

    /// The http method for the route.
    pub method: Method,

    /// The query params for the route.
    pub params: Vec<(String, String)>,
}

impl CompiledRoute {
    /// Creates a new compiled route.
    ///
    /// # Arguments
    /// - `route`: The static [`Route`] this one will extend.
    ///
    /// # Returns
    /// - [`Self`]: The new route.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::routes::CompiledRoute;
    /// # use unkey_sdk::routes::Route;
    /// # use reqwest::Method;
    /// let r = Route::new(Method::GET, "/apis/hi");
    /// let c = CompiledRoute::new(&r);
    ///
    /// assert_eq!(c.params, vec![]);
    /// assert_eq!(c.method, Method::GET);
    /// assert_eq!(c.uri, String::from("/apis/hi"));
    /// ```
    #[rustfmt::skip]
    pub fn new(route: &Route) -> Self {
        let params = Vec::new();
        let uri = route.uri.to_string();
        let method = route.method.clone();

        Self { method, params, uri }
    }

    /// Inserts the given param into the route uri.
    ///
    /// # Arguments
    /// - `param`: The param to insert.
    ///
    /// # Returns
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::routes::CompiledRoute;
    /// # use unkey_sdk::routes::Route;
    /// # use reqwest::Method;
    /// let r = Route::new(Method::GET, "/apis/{}/keys/{}");
    /// let mut c = CompiledRoute::new(&r);
    /// c.uri_insert("5").uri_insert("1");
    ///
    /// assert_eq!(c.params, vec![]);
    /// assert_eq!(c.method, Method::GET);
    /// assert_eq!(c.uri, String::from("/apis/5/keys/1"));
    /// ```
    pub fn uri_insert<T: Into<String>>(&mut self, param: T) -> &mut Self {
        self.uri = self.uri.replacen("{}", &param.into(), 1);
        self
    }

    /// Inserts a query param with the given name and value.
    ///
    /// # Arguments
    /// - `name`: The param name to insert.
    /// - `param`: The param value to insert.
    ///
    /// # Returns
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::routes::CompiledRoute;
    /// # use unkey_sdk::routes::Route;
    /// # use reqwest::Method;
    /// let r = Route::new(Method::GET, "/apis/milk");
    /// let mut c = CompiledRoute::new(&r);
    /// c.query_insert("test", "value");
    ///
    /// assert_eq!(c.params, vec![(String::from("test"), String::from("value"))]);
    /// assert_eq!(c.method, Method::GET);
    /// assert_eq!(c.uri, String::from("/apis/milk"));
    /// ```
    pub fn query_insert<T: Into<String>>(&mut self, name: T, value: T) -> &mut Self {
        self.params.push((name.into(), value.into()));
        self
    }

    /// Builds the query string for this route, i.e. `?a=b&c=d`.
    ///
    /// # Returns
    /// - [`String`]: The formatted query string.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::routes::CompiledRoute;
    /// # use unkey_sdk::routes::Route;
    /// # use reqwest::Method;
    /// let r = Route::new(Method::GET, "/apis/milk");
    /// let mut c = CompiledRoute::new(&r);
    /// c.query_insert("test", "value").query_insert("js", "bad");
    ///
    /// assert_eq!(c.build_query(), String::from("?test=value&js=bad"));
    /// ```
    pub fn build_query(&self) -> String {
        let mut query = self
            .params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");

        if !query.is_empty() {
            query.insert(0, '?');
        }

        query
    }
}
