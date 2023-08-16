use crate::models::Ratelimit;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An outgoing list keys request.
#[derive(Debug, Clone, Serialize)]
pub struct ListKeysRequest {
    /// The api id whose keys to list.
    #[serde(rename = "apiId")]
    pub api_id: String,

    /// If provided, this will only return keys where the `ownerId` matches.
    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,

    /// Limit the number of returned keys, the maximum is 100.
    pub limit: Option<usize>,

    /// An offset for pagination.
    pub offset: Option<usize>,
}

impl ListKeysRequest {
    /// Creates a new request for listing keys.
    ///
    /// # Arguments
    /// - `api_id`: The api id to whose keys to list.
    ///
    /// # Returns
    /// - [`KeysListResponse`]: List of `KeysListResponse`.
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

    /// Sets limit for the keys list request.
    /// # Arguments
    /// - `limit`: The limit to set. Default 100.
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

    /// Sets offset for the keys list request.
    /// # Arguments
    /// - `offset`: The limit to set for pagination. Default 0.
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

    /// Sets owner id for the keys list request. Returns keys where the owner id matches.
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

/// An incoming list keys response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListKeysResponse {
    /// The list of API keys.
    pub keys: Vec<ApiKey>,

    /// The total number of API keys present.
    pub total: usize,
}

// Individual key object returned in the response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// The id of the API key.
    pub id: String,

    /// The id of the API this key belongs to.
    #[serde(rename = "apiId")]
    pub api_id: String,

    /// The id of the workspace this key belongs to.
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,

    /// The starting characters of the key
    pub start: String,

    /// The optional owner id of the key.
    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,

    /// Any optional metadata associated with the key.
    pub meta: Option<Value>,

    /// The key creation time in ms.
    #[serde(rename = "createdAt")]
    pub created_at: usize,

    /// The key expiry time in ms.
    pub expires: Option<usize>,

    /// The number of key invocations remaining.
    pub remaining: Option<usize>,

    /// The optional rate limit set to the key.
    pub ratelimit: Option<Ratelimit>,
}
