use serde::{Deserialize, Serialize};

use super::ApiKey;

/// An outgoing paginated list keys request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListKeysRequest {
    /// The id of the api to list keys for.
    pub api_id: String,

    /// The optional owner id used to filter keys by owner.
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
    /// - `api_id`: The id of the api to list keys for.
    ///
    /// # Returns
    /// The new list keys request.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::ListKeysRequest;
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
    /// - `limit`: The limit to set, defaults to 100.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::ListKeysRequest;
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
    /// - `offset`: The pagination offset to set, defaults to 0.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::ListKeysRequest;
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
    ///
    /// # Arguments
    /// - `owner_id`: The owner id to set.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::ListKeysRequest;
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
    /// The api keys included in this page.
    pub keys: Vec<ApiKey>,

    /// The total number of api keys.
    pub total: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApiRequest {
    /// The id of the api to get information for.
    pub api_id: String,
}

/// An outgoing get api information request.
impl GetApiRequest {
    /// Creates a new get api request.
    ///
    /// # Arguments
    /// - `api_id`: The id of the api to get api information for.
    ///
    /// # Returns
    /// The new get api information request.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::GetApiRequest;
    /// let r = GetApiRequest::new("test");
    ///
    /// assert_eq!(r.api_id, String::from("test"));
    /// ```
    #[must_use]
    pub fn new<T: Into<String>>(api_id: T) -> Self {
        Self {
            api_id: api_id.into(),
        }
    }
}

/// An incoming api information response.
#[derive(Debug, Clone, Deserialize)]
pub struct GetApiResponse {
    /// The id of the api.
    #[serde(rename = "id")]
    pub api_id: String,

    /// The name of the api.
    pub name: String,

    /// The workspace id of the api.
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
}
