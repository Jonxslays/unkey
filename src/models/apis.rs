use crate::models::Ratelimit;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An outgoing paginated list keys request.
#[derive(Debug, Clone, Serialize)]
pub struct ListKeysRequest {
    /// The id of the api to list keys for.
    #[serde(rename = "apiId")]
    pub api_id: String,

    /// The optional owner id used to filter keys by owner.
    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,

    /// The optional number of keys to return, up to 100.
    pub limit: Option<usize>,

    /// The pagination offset.
    pub offset: Option<usize>,
}

impl ListKeysRequest {
    /// Creates a new list keys request.
    ///
    /// # Arguments
    /// - - `api_id`: The id of the api to list keys for.
    ///
    /// # Returns
    /// - - [`ListKeysResponse`]: The paginated list of api
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::ListKeysRequest;
    /// let r = ListKeysRequest::new("test");
    ///
    /// assert_eq!(r.api_id, String::from("test"));
    /// assert_eq!(r.limit, None);
    /// assert_eq!(r.offset, None);
    /// assert_eq!(r.owner_id, None);
    /// ```
    #[must_use]
    pub fn new<T: Into<String>>(api_id: T) -> Self {
        Self {
            api_id: api_id.into(),
            owner_id: None,
            limit: None,
            offset: None,
        }
    }

    /// Sets the limit for the request.
    ///
    /// # Arguments
    /// - `limit`: - `limit`: The limit to set, defaults to 100.
    ///
    /// # Returns
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::ListKeysRequest;
    /// let r = ListKeysRequest::new("test").set_limit(50);
    ///
    /// assert_eq!(r.limit.unwrap(), 50);
    /// ```
    #[must_use]
    pub fn set_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the pagination offset for the request.
    ///
    /// # Arguments
    /// - `offset`: - `offset`: The pagination offset to set, defaults to 0.
    ///
    /// # Returns
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::ListKeysRequest;
    /// let r = ListKeysRequest::new("test").set_offset(4);
    ///
    /// assert_eq!(r.offset.unwrap(), 4);
    /// ```
    #[must_use]
    pub fn set_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Sets the owner id for filtering the listed keys by owner.
    /// # Arguments
    /// - `owner_id`: The owner id to set.
    ///
    /// # Returns
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::ListKeysRequest;
    /// let r = ListKeysRequest::new("test").set_owner_id("WilfredAlmeida");
    ///
    /// assert_eq!(r.owner_id.unwrap(), String::from("WilfredAlmeida"));
    /// ```
    #[must_use]
    pub fn set_owner_id<T: Into<String>>(mut self, owner_id: T) -> Self {
        self.owner_id = Some(owner_id.into());
        self
    }
}

/// An incoming paginated list keys response.
#[derive(Debug, Clone, Deserialize)]
pub struct ListKeysResponse {
    /// The total number of api keys.
    pub keys: Vec<ApiKey>,

    /// The total number of API keys present.
    pub total: usize,
}

/// An individual api key, as the unkey api sees it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// The unique id of this key.
    pub id: String,

    /// The id of the api this key belongs to.
    #[serde(rename = "apiId")]
    pub api_id: String,

    /// The keys prefix.
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,

    /// The starting characters of the key
    pub start: String,

    /// The owner id of the key, if one was set.
    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,

    /// The dynamic metadata associated with the key, if any.
    pub meta: Option<Value>,

    /// The keys creation time in ms since the unix epoch.
    #[serde(rename = "createdAt")]
    pub created_at: usize,

    /// The unix epoch in ms when this key expires, if it does.
    pub expires: Option<usize>,

    /// The number of uses remaining for this key, if any.
    ///
    /// *Note*: If `None`, the key has unlimited uses remaining.
    pub remaining: Option<usize>,

    /// The ratelimit imposed on this key, if any.
    pub ratelimit: Option<Ratelimit>,
}
