use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Ratelimit;
use super::RatelimitState;
use crate::Undefined;

/// An outgoing verify key request.
#[derive(Debug, Clone, Serialize)]
pub struct VerifyKeyRequest {
    /// The api key to verify.
    pub key: String,
}

impl VerifyKeyRequest {
    /// Creates a new verify key request.
    ///
    /// # Arguments
    /// - `key`: The api key to verify.
    ///
    /// # Returns
    /// The verify key request.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::VerifyKeyRequest;
    /// let r = VerifyKeyRequest::new("test");
    ///
    /// assert_eq!(r.key, String::from("test"));
    /// ```
    #[must_use]
    pub fn new<T: Into<String>>(key: T) -> Self {
        Self { key: key.into() }
    }
}

/// An incoming verify key response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyKeyResponse {
    /// Whether or not the key is valid for any reason.
    ///
    /// e.g. ratelimited, no more remaining, expired, key not found.
    pub valid: bool,

    /// The owner id for this key, if any.
    pub owner_id: Option<String>,

    /// The dynamic mapping of values associated with this key, if any.
    pub meta: Option<Value>,

    /// The number of verifications before this key becomes invalidated, if
    /// any limit was set on the key.
    pub remaining: Option<usize>,

    /// The unix epoch in ms when this key expires, if it does.
    pub expires: Option<usize>,

    /// The state of the ratelimit set on this key, if any.
    pub ratelimit: Option<RatelimitState>,
}

/// An outgoing create key request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateKeyRequest {
    /// The api id to create this key for.
    pub api_id: String,

    /// The optional owner id for the key.
    pub owner_id: Option<String>,

    /// The optional byte length for the key, defaults to 16.
    pub byte_length: Option<usize>,

    /// The optional prefix for the key.
    pub prefix: Option<String>,

    /// The optional name for the key.
    pub name: Option<String>,

    /// The optional dynamic meta mapping for the key.
    pub meta: Option<Value>,

    /// The optional unix epoch in ms when the key should expire.
    pub expires: Option<usize>,

    /// The optional number of uses remaining to set for the key.
    pub remaining: Option<usize>,

    /// The optional ratelimit to set for the key.
    pub ratelimit: Option<Ratelimit>,
}

impl CreateKeyRequest {
    /// Creates a new request for key creation.
    ///
    /// # Arguments
    /// - `api_id`: The api id to create this key for.
    ///
    /// # Returns
    /// The new create key request.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::CreateKeyRequest;
    /// let r = CreateKeyRequest::new("test");
    ///
    /// assert_eq!(r.api_id, String::from("test"));
    /// assert_eq!(r.owner_id, None);
    /// assert_eq!(r.byte_length, None);
    /// assert_eq!(r.prefix, None);
    /// assert_eq!(r.name, None);
    /// assert_eq!(r.meta, None);
    /// assert_eq!(r.expires, None);
    /// assert_eq!(r.remaining, None);
    /// assert_eq!(r.ratelimit, None);
    /// ```
    #[must_use]
    pub fn new<T: Into<String>>(api_id: T) -> Self {
        Self {
            api_id: api_id.into(),
            owner_id: None,
            byte_length: None,
            prefix: None,
            name: None,
            meta: None,
            expires: None,
            remaining: None,
            ratelimit: None,
        }
    }

    /// Sets the owner id for the new key.
    ///
    /// # Arguments
    /// - `owner_id`: The owner id to set.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::CreateKeyRequest;
    /// let r = CreateKeyRequest::new("test").set_owner_id("jonxslays");
    ///
    /// assert_eq!(r.owner_id.unwrap(), String::from("jonxslays"));
    /// ```
    #[must_use]
    pub fn set_owner_id<T: Into<String>>(mut self, owner_id: T) -> Self {
        self.owner_id = Some(owner_id.into());
        self
    }

    /// Sets the byte length for the new key.
    ///
    /// # Arguments
    /// - `byte_length`: The byte length to set.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::CreateKeyRequest;
    /// let r = CreateKeyRequest::new("test").set_byte_length(32);
    ///
    /// assert_eq!(r.byte_length.unwrap(), 32);
    /// ```
    #[must_use]
    pub fn set_byte_length(mut self, byte_length: usize) -> Self {
        self.byte_length = Some(byte_length);
        self
    }

    /// Sets the prefix for the new key.
    ///
    /// # Arguments
    /// - `prefix`: The prefix to set.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::CreateKeyRequest;
    /// let r = CreateKeyRequest::new("test").set_prefix("dev");
    ///
    /// assert_eq!(r.prefix.unwrap(), String::from("dev"));
    /// ```
    #[must_use]
    pub fn set_prefix<T: Into<String>>(mut self, prefix: T) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Sets the name for the new key.
    ///
    /// # Arguments
    /// - `name`: The name to set.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::CreateKeyRequest;
    /// let r = CreateKeyRequest::new("test").set_name("example_key");
    ///
    /// assert_eq!(r.name.unwrap(), String::from("example_key"));
    /// ```
    #[must_use]
    pub fn set_name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the dynamic meta mapping for the new key.
    ///
    /// # Arguments
    /// - `meta`: The meta to set.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::CreateKeyRequest;
    /// # use serde_json::json;
    /// let r = CreateKeyRequest::new("test").set_meta(json!({"test": 1}));
    ///
    /// assert_eq!(r.meta.unwrap(), json!({"test": 1}));
    /// ```
    #[must_use]
    pub fn set_meta(mut self, meta: Value) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets when this key expires.
    ///
    /// # Arguments
    /// - `expires`: The number of milliseconds in the future this key should
    /// expire at.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::CreateKeyRequest;
    /// # use std::time::SystemTime;
    /// let now = SystemTime::now()
    ///    .duration_since(std::time::UNIX_EPOCH)
    ///    .unwrap()
    ///    .as_millis() as usize;
    ///
    /// let r = CreateKeyRequest::new("test").set_expires(1000 * 60 * 10);
    ///
    /// // 10 minutes in the future +- 1 second
    /// let expiration = now + 1000 * 60 * 10;
    /// let range = expiration..expiration+2;
    /// assert!(range.contains(&r.expires.unwrap()));
    /// ```
    #[must_use]
    pub fn set_expires(mut self, expires: usize) -> Self {
        let duration = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                eprintln!("Error fetching duration since unix epoch: {e}");
                std::process::exit(1);
            });

        let expires = duration.as_millis() as usize + expires;
        self.expires = Some(expires);
        self
    }

    /// Sets the remaining uses for the new key.
    ///
    /// # Arguments
    /// - `remaining`: The remaining uses to set.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::CreateKeyRequest;
    /// let r = CreateKeyRequest::new("test").set_remaining(100);
    ///
    /// assert_eq!(r.remaining.unwrap(), 100);
    /// ```
    #[must_use]
    pub fn set_remaining(mut self, remaining: usize) -> Self {
        self.remaining = Some(remaining);
        self
    }

    /// Sets the ratelimit for the new key.
    ///
    /// # Arguments
    /// - `ratelimit`: The ratelimit uses to set.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::CreateKeyRequest;
    /// # use unkey::models::Ratelimit;
    /// # use unkey::models::RatelimitType;
    /// let ratelimit = Ratelimit::new(
    ///     RatelimitType::Fast,
    ///     10,
    ///     10000,
    ///     100
    /// );
    ///
    /// let r = CreateKeyRequest::new("test").set_ratelimit(ratelimit.clone());
    ///
    /// assert_eq!(r.ratelimit.unwrap(), ratelimit);
    /// ```
    #[must_use]
    pub fn set_ratelimit(mut self, ratelimit: Ratelimit) -> Self {
        self.ratelimit = Some(ratelimit);
        self
    }
}

/// An incoming create key response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateKeyResponse {
    /// The unique id of this key.
    pub key_id: String,

    /// The newly created api key.
    pub key: String,
}

/// An individual api key, as the unkey api sees it.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    /// The unique id of this key.
    pub id: String,

    /// The id of the api this key belongs to.
    pub api_id: String,

    /// The id of the workspace this key belongs to.
    pub workspace_id: String,

    /// The keys prefix.
    pub start: String,

    /// The owner id of the key, if one was set.
    pub owner_id: Option<String>,

    /// The dynamic metadata associated with the key, if any.
    pub meta: Option<Value>,

    /// The keys creation time in ms since the unix epoch.
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

/// An outgoing revoke key request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeKeyRequest {
    /// The unique id of the key to revoke.
    pub key_id: String,
}

impl RevokeKeyRequest {
    /// Creates a new revoke key request.
    ///
    /// # Arguments
    /// - `key_id`: The id of the key to revoke.
    ///
    /// # Returns
    /// The revoke key request.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::RevokeKeyRequest;
    /// let r = RevokeKeyRequest::new("test_ABC123");
    ///
    /// assert_eq!(r.key_id, String::from("test_ABC123"));
    /// ```
    #[must_use]
    #[rustfmt::skip]
    pub fn new<T: Into<String>>(key_id: T) -> Self {
        Self { key_id: key_id.into() }
    }
}

#[derive(Debug, Clone)]
pub struct UpdateKeyRequest {
    /// The id of the key to update.
    pub key_id: String,

    /// The optional new owner id for the key.
    pub owner_id: Undefined<Option<String>>,

    /// The optional new name for the key.
    pub name: Option<String>,

    /// The optional new dynamic meta mapping for the key.
    pub meta: Option<Value>,

    /// The optional new unix epoch in ms when the key should expire.
    pub expires: Option<usize>,

    /// The optional new number of uses remaining to set for the key.
    pub remaining: Option<usize>,

    /// The optional new ratelimit to set for the key.
    pub ratelimit: Option<Ratelimit>,
}

impl Serialize for UpdateKeyRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}
