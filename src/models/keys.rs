use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    /// - [`Self`]: The verification request.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::VerifyKeyRequest;
    /// let r = VerifyKeyRequest::new("test");
    ///
    /// assert_eq!(r.key, String::from("test"));
    /// ```
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
        }
    }
}

/// An incoming key verification response.
#[derive(Debug, Clone, Deserialize)]
pub struct VerifyKeyResponse {
    /// Whether or not the key is valid for any reason.
    ///
    /// e.g. ratelimited, no more remaining, expired, key not found.
    pub valid: bool,

    /// The optional owner of this key.
    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,

    /// The optional dynamic mapping of values to associate with
    /// this key.
    pub meta: Option<Value>,

    /// The number of verifications to allow this key before it
    /// becomes invalidated.
    pub remaining: Option<usize>,
}

/// Different rate limit types implemented by unkey.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum RatelimitType {
    /// Quick because each edge location maintains its own ratelimit.
    #[serde(rename = "fast")]
    Fast,

    /// All ratelimit operations go through a single service,
    /// meaning consistent ratelimits.
    #[serde(rename = "consistent")]
    Consistent,
}

/// A ratelimit imposed on an api key.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Ratelimit {
    /// The type for this ratelimit.
    #[serde(rename = "type")]
    pub ratelimit_type: RatelimitType,

    /// The rate at which the ratelimit refills, per interval.
    #[serde(rename = "refillRate")]
    pub refill_rate: usize,

    /// The interval at which to refill, in milliseconds.
    #[serde(rename = "refillInterval")]
    pub refill_interval: usize,

    /// Total number of burstable requests.
    pub limit: usize,
}

impl Ratelimit {
    /// Creates a new ratelimit.
    ///
    /// # Arguments
    /// - `ratelimit_type`: The type for this ratelimit.
    /// - `refill_rate`: The rate at which the ratelimit refills, per interval.
    /// - `refill_interval`: The interval at which to refill, in milliseconds.
    /// - `limit`: Total number of burstable requests.
    ///
    /// # Returns
    /// - [`Self`]: The requested ratelimit.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::Ratelimit;
    /// # use unkey_sdk::models::RatelimitType;
    /// let r = Ratelimit::new(
    ///     RatelimitType::Fast,
    ///     10,
    ///     10000,
    ///     100,
    /// );
    ///
    /// assert_eq!(r.ratelimit_type, RatelimitType::Fast);
    /// assert_eq!(r.refill_rate, 10);
    /// assert_eq!(r.refill_interval, 10000);
    /// assert_eq!(r.limit, 100);
    /// ```
    pub fn new(
        ratelimit_type: RatelimitType,
        refill_rate: usize,
        refill_interval: usize,
        limit: usize,
    ) -> Self {
        Self {
            ratelimit_type,
            refill_rate,
            refill_interval,
            limit,
        }
    }
}

/// An outgoing create key request.
#[derive(Debug, Clone, Serialize)]
pub struct CreateKeyRequest {
    /// The api id to create this key for.
    #[serde(rename = "apiId")]
    pub api_id: String,

    /// The optional owner id for the key.
    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,

    /// The optional byte length for the key, defaults to 16.
    #[serde(rename = "byteLength")]
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
    /// - `owner_id`: The optional owner for the key.
    /// - `byte_length`: The optional byte length for the key, defaults to 16.
    /// - `prefix`: The optional prefix for the key.
    /// - `name`: The optional name for the key.
    /// - `meta`: The optional dynamic meta mapping for the key. Use the
    ///     `json!` macro.
    /// - `expires`: The optional unix epoch in ms when the key should expire.
    /// - `remaining`: The optional number of uses remaining to set for the key.
    /// - `ratelimit`: The optional ratelimit to set for the key.
    ///
    /// # Returns
    /// - [`Self`]: The new create key request.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::CreateKeyRequest;
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
    pub fn new(api_id: &str) -> Self {
        Self {
            api_id: api_id.to_string(),
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
    /// # Arguments:
    /// - `owner_id`: The owner id to set.
    ///
    /// # Returns
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::CreateKeyRequest;
    /// let r = CreateKeyRequest::new("test").set_owner_id("jonxslays");
    ///
    /// assert_eq!(r.owner_id.unwrap(), String::from("jonxslays"));
    /// ```
    pub fn set_owner_id(mut self, owner_id: &str) -> Self {
        self.owner_id = Some(owner_id.to_string());
        self
    }

    /// Sets the byte length for the new key.
    ///
    /// # Arguments:
    /// - `byte_length`: The byte length to set.
    ///
    /// # Returns
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::CreateKeyRequest;
    /// let r = CreateKeyRequest::new("test").set_byte_length(32);
    ///
    /// assert_eq!(r.byte_length.unwrap(), 32);
    /// ```
    pub fn set_byte_length(mut self, byte_length: usize) -> Self {
        self.byte_length = Some(byte_length);
        self
    }

    /// Sets the prefix for the new key.
    ///
    /// # Arguments:
    /// - `prefix`: The prefix to set.
    ///
    /// # Returns
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::CreateKeyRequest;
    /// let r = CreateKeyRequest::new("test").set_prefix("dev");
    ///
    /// assert_eq!(r.prefix.unwrap(), String::from("dev"));
    /// ```
    pub fn set_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }

    /// Sets the name for the new key.
    ///
    /// # Arguments:
    /// - `name`: The name to set.
    ///
    /// # Returns
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::CreateKeyRequest;
    /// let r = CreateKeyRequest::new("test").set_name("example_key");
    ///
    /// assert_eq!(r.name.unwrap(), String::from("example_key"));
    /// ```
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets the dynamic meta mapping for the new key.
    ///
    /// # Arguments:
    /// - `meta`: The meta to set.
    ///
    /// # Returns
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::CreateKeyRequest;
    /// # use serde_json::json;
    /// let r = CreateKeyRequest::new("test").set_meta(json!({"test": 1}));
    ///
    /// assert_eq!(r.meta.unwrap(), json!({"test": 1}));
    /// ```
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
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::CreateKeyRequest;
    /// # use std::time::SystemTime;
    /// let now = SystemTime::now()
    ///    .duration_since(SystemTime::from(std::time::UNIX_EPOCH))
    ///    .unwrap()
    ///    .as_millis() as usize;
    ///
    /// let r = CreateKeyRequest::new("test").set_expires(1000 * 60 * 10);
    /// 
    /// // 10 minutes in the future
    /// assert_eq!(now + 1000 * 60 * 10, r.expires.unwrap());
    /// ```
    pub fn set_expires(mut self, expires: usize) -> Self {
        let duration = SystemTime::now()
            .duration_since(SystemTime::from(std::time::UNIX_EPOCH))
            .unwrap();

        let expires = duration.as_millis() as usize + expires;
        self.expires = Some(expires);
        self
    }

    /// Sets the remaining uses for the new key.
    ///
    /// # Arguments:
    /// - `remaining`: The remaining uses to set.
    ///
    /// # Returns
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::CreateKeyRequest;
    /// let r = CreateKeyRequest::new("test").set_remaining(100);
    ///
    /// assert_eq!(r.remaining.unwrap(), 100);
    /// ```
    pub fn set_remaining(mut self, remaining: usize) -> Self {
        self.remaining = Some(remaining);
        self
    }

    /// Sets the ratelimit for the new key.
    ///
    /// # Arguments:
    /// - `ratelimit`: The ratelimit uses to set.
    ///
    /// # Returns
    /// - [`Self`]: for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::models::CreateKeyRequest;
    /// # use unkey_sdk::models::Ratelimit;
    /// # use unkey_sdk::models::RatelimitType;
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
    pub fn set_ratelimit(mut self, ratelimit: Ratelimit) -> Self {
        self.ratelimit = Some(ratelimit);
        self
    }
}

/// An incoming create key response.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateKeyResponse {
    /// The unique id for this key.
    #[serde(rename = "keyId")]
    pub key_id: String,

    /// The newly created api secret key.
    pub key: String,
}
