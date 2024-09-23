use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Ratelimit;
use super::RatelimitState;
use super::Refill;
use super::UndefinedOr;

/// An update operation that can be performed.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UpdateOp {
    /// Increment operation.
    Increment,

    /// Decrement operation.
    Decrement,

    /// Set operation.
    Set,
}

/// An outgoing verify key request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyKeyRequest {
    /// The api key to verify.
    pub key: String,

    /// The id of the api this key belongs to.
    pub api_id: String,
}

impl VerifyKeyRequest {
    /// Creates a new verify key request.
    ///
    /// # Arguments
    /// - `key`: The api key to verify.
    /// - `api_id`: The id of the api this key belongs to.
    ///
    /// # Returns
    /// The verify key request.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::VerifyKeyRequest;
    /// let r = VerifyKeyRequest::new("test", "api_123");
    ///
    /// assert_eq!(r.key, String::from("test"));
    /// assert_eq!(r.api_id, String::from("api_123"));
    /// ```
    #[must_use]
    pub fn new<T: Into<String>>(key: T, api_id: T) -> Self {
        Self {
            key: key.into(),
            api_id: api_id.into(),
        }
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

    /// The keys unique id, if any.
    pub key_id: Option<String>,

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

    /// The refill state of this key, if any.
    pub refill: Option<Refill>,
}

/// An outgoing create key request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateKeyRequest {
    /// The api id to create this key for.
    pub api_id: String,

    /// The optional owner id for the key.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub owner_id: UndefinedOr<String>,

    /// The optional byte length for the key, defaults to 16.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub byte_length: UndefinedOr<usize>,

    /// The optional prefix for the key.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub prefix: UndefinedOr<String>,

    /// The optional name for the key.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub name: UndefinedOr<String>,

    /// The optional dynamic meta mapping for the key.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub meta: UndefinedOr<Value>,

    /// The optional unix epoch in ms when the key should expire.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub expires: UndefinedOr<usize>,

    /// The optional number of uses remaining to set for the key.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub remaining: UndefinedOr<usize>,

    /// The optional ratelimit to set for the key.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub ratelimit: UndefinedOr<Ratelimit>,

    /// The keys refill state, if any.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub refill: UndefinedOr<Refill>,
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
    /// # use unkey::models::UndefinedOr;
    /// let r = CreateKeyRequest::new("test");
    ///
    /// assert_eq!(r.api_id, String::from("test"));
    /// assert_eq!(r.owner_id, UndefinedOr::Undefined);
    /// assert_eq!(r.byte_length, UndefinedOr::Undefined);
    /// assert_eq!(r.prefix, UndefinedOr::Undefined);
    /// assert_eq!(r.name, UndefinedOr::Undefined);
    /// assert_eq!(r.meta, UndefinedOr::Undefined);
    /// assert_eq!(r.expires, UndefinedOr::Undefined);
    /// assert_eq!(r.remaining, UndefinedOr::Undefined);
    /// assert_eq!(r.ratelimit, UndefinedOr::Undefined);
    /// assert_eq!(r.refill, UndefinedOr::Undefined);
    /// ```
    #[must_use]
    pub fn new<T: Into<String>>(api_id: T) -> Self {
        Self {
            api_id: api_id.into(),
            owner_id: UndefinedOr::Undefined,
            byte_length: UndefinedOr::Undefined,
            prefix: UndefinedOr::Undefined,
            name: UndefinedOr::Undefined,
            meta: UndefinedOr::Undefined,
            expires: UndefinedOr::Undefined,
            remaining: UndefinedOr::Undefined,
            ratelimit: UndefinedOr::Undefined,
            refill: UndefinedOr::Undefined,
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
    /// assert_eq!(r.owner_id.inner().unwrap(), &String::from("jonxslays"));
    /// ```
    #[must_use]
    pub fn set_owner_id<T: Into<String>>(mut self, owner_id: T) -> Self {
        self.owner_id = UndefinedOr::Value(owner_id.into());
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
    /// assert_eq!(r.byte_length.inner().unwrap(), &32);
    /// ```
    #[must_use]
    pub fn set_byte_length(mut self, byte_length: usize) -> Self {
        self.byte_length = UndefinedOr::Value(byte_length);
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
    /// assert_eq!(r.prefix.inner().unwrap(), &String::from("dev"));
    /// ```
    #[must_use]
    pub fn set_prefix<T: Into<String>>(mut self, prefix: T) -> Self {
        self.prefix = UndefinedOr::Value(prefix.into());
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
    /// assert_eq!(r.name.inner().unwrap(), &String::from("example_key"));
    /// ```
    #[must_use]
    pub fn set_name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = UndefinedOr::Value(name.into());
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
    /// assert_eq!(r.meta.inner().unwrap(), &json!({"test": 1}));
    /// ```
    #[must_use]
    pub fn set_meta(mut self, meta: Value) -> Self {
        self.meta = UndefinedOr::Value(meta);
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
    /// assert!(range.contains(r.expires.inner().unwrap()));
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
        self.expires = UndefinedOr::Value(expires);
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
    /// assert_eq!(r.remaining.inner().unwrap(), &100);
    /// ```
    #[must_use]
    pub fn set_remaining(mut self, remaining: usize) -> Self {
        self.remaining = UndefinedOr::Value(remaining);
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
    /// assert_eq!(r.ratelimit.inner().unwrap(), &ratelimit);
    /// ```
    #[must_use]
    pub fn set_ratelimit(mut self, ratelimit: Ratelimit) -> Self {
        self.ratelimit = UndefinedOr::Value(ratelimit);
        self
    }

    /// Sets the refill for the new key.
    ///
    /// # Arguments
    /// - `refill`: The refill to set.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::CreateKeyRequest;
    /// # use unkey::models::Refill;
    /// # use unkey::models::RefillInterval;
    /// let refill = Refill::new(100, RefillInterval::Daily);
    ///
    /// let r = CreateKeyRequest::new("test").set_refill(refill.clone());
    ///
    /// assert_eq!(r.refill.inner().unwrap(), &refill);
    /// ```
    #[must_use]
    pub fn set_refill(mut self, refill: Refill) -> Self {
        self.refill = UndefinedOr::Value(refill);
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

    /// The optional name for the key.
    pub name: Option<String>,

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

    /// The refill state of this key, if any.
    pub refill: Option<Refill>,
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

/// An outgoing update key request.
///
/// ## Note
/// All optional values are initialized to the [`UndefinedOr::Undefined`] state.
/// Upon calling the `set_x` method, you may set the value to `Some(_)` or
/// `None`. Setting the value to `None` indicates you would like to remove any
/// value that is currently set for that field on the key.
///
/// e.g. The key you are updating currently has a ratelimit and you call
/// `set_ratelimit(None)` on the update key request. The key will no longer
/// have a ratelimit.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateKeyRequest {
    /// The id of the key to update.
    pub key_id: String,

    /// The optional new owner id for the key.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub owner_id: UndefinedOr<String>,

    /// The optional new name for the key.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub name: UndefinedOr<String>,

    /// The optional new dynamic meta mapping for the key.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub meta: UndefinedOr<Value>,

    /// The optional new unix epoch in ms when the key should expire.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub expires: UndefinedOr<usize>,

    /// The optional new number of uses remaining to set for the key.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub remaining: UndefinedOr<usize>,

    /// The optional new ratelimit to set for the key.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub ratelimit: UndefinedOr<Ratelimit>,

    /// The optional new refill to set for the key.
    #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
    pub refill: UndefinedOr<Refill>,
}

impl UpdateKeyRequest {
    /// Creates a new update key request.
    ///
    /// # Arguments
    /// - `key_id`: The id of the key to update.
    ///
    /// # Returns
    /// The new update key request.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UpdateKeyRequest;
    /// # use unkey::models::UndefinedOr;
    /// let r = UpdateKeyRequest::new("test_123");
    ///
    /// assert_eq!(r.key_id, String::from("test_123"));
    /// assert_eq!(r.owner_id, UndefinedOr::Undefined);
    /// assert_eq!(r.name, UndefinedOr::Undefined);
    /// assert_eq!(r.meta, UndefinedOr::Undefined);
    /// assert_eq!(r.expires, UndefinedOr::Undefined);
    /// assert_eq!(r.remaining, UndefinedOr::Undefined);
    /// assert_eq!(r.ratelimit, UndefinedOr::Undefined);
    /// assert_eq!(r.refill, UndefinedOr::Undefined);
    /// ```
    #[must_use]
    pub fn new<T: Into<String>>(key_id: T) -> Self {
        Self {
            key_id: key_id.into(),
            ..Default::default()
        }
    }

    /// Sets or unsets the owner id for the key.
    ///
    /// # Arguments
    /// - `owner_id`: The owner id to set or unset.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UpdateKeyRequest;
    /// # use unkey::models::UndefinedOr;
    /// let r = UpdateKeyRequest::new("test");
    ///
    /// assert_eq!(r.owner_id, UndefinedOr::Undefined);
    /// assert_eq!(r.owner_id.inner(), None);
    ///
    /// let r = r.set_owner_id(Some("jonxslays"));
    ///
    /// assert_eq!(r.owner_id, UndefinedOr::Value(String::from("jonxslays")));
    /// assert_eq!(r.owner_id.inner(), Some(&String::from("jonxslays")));
    ///
    /// let r = r.set_owner_id(None);
    ///
    /// assert_eq!(r.owner_id, UndefinedOr::Null);
    /// assert_eq!(r.owner_id.inner(), None);
    /// ```
    #[must_use]
    pub fn set_owner_id(mut self, owner_id: Option<&str>) -> Self {
        self.owner_id = match owner_id {
            Some(id) => Some(id.into()).into(),
            None => None.into(),
        };

        self
    }

    /// Sets or unsets the name for the key.
    ///
    /// # Arguments
    /// - `name`: The name to set or unset.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UpdateKeyRequest;
    /// # use unkey::models::UndefinedOr;
    /// let r = UpdateKeyRequest::new("test");
    ///
    /// assert_eq!(r.name, UndefinedOr::Undefined);
    /// assert_eq!(r.name.inner(), None);
    ///
    /// let r = r.set_name(Some("test_key"));
    ///
    /// assert_eq!(r.name, UndefinedOr::Value(String::from("test_key")));
    /// assert_eq!(r.name.inner(), Some(&String::from("test_key")));
    ///
    /// let r = r.set_name(None);
    ///
    /// assert_eq!(r.name, UndefinedOr::Null);
    /// assert_eq!(r.name.inner(), None);
    /// ```
    #[must_use]
    pub fn set_name(mut self, name: Option<&str>) -> Self {
        self.name = match name {
            Some(n) => Some(n.into()).into(),
            None => None.into(),
        };

        self
    }

    /// Sets or unsets the dynamic meta mapping for the key.
    ///
    /// # Arguments
    /// - `meta`: The meta to set or unset.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UpdateKeyRequest;
    /// # use unkey::models::UndefinedOr;
    /// # use serde_json::json;
    /// let r = UpdateKeyRequest::new("test");
    ///
    /// assert_eq!(r.meta, UndefinedOr::Undefined);
    /// assert_eq!(r.meta.inner(), None);
    ///
    /// let r = r.set_meta(Some(json!({"test": 69})));
    ///
    /// assert_eq!(r.meta, UndefinedOr::Value(json!({"test": 69})));
    /// assert_eq!(r.meta.inner(), Some(&json!({"test": 69})));
    ///
    /// let r = r.set_meta(None);
    ///
    /// assert_eq!(r.meta, UndefinedOr::Null);
    /// assert_eq!(r.meta.inner(), None);
    /// ```
    #[must_use]
    pub fn set_meta(mut self, meta: Option<Value>) -> Self {
        self.meta = match meta {
            Some(m) => Some(m).into(),
            None => None.into(),
        };

        self
    }

    /// Sets or unsets the unix epoch in ms indicating when this key expires.
    ///
    /// # Arguments
    /// - `expires`: The expiration epoch to set or unset.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UpdateKeyRequest;
    /// # use unkey::models::UndefinedOr;
    /// let r = UpdateKeyRequest::new("test");
    ///
    /// assert_eq!(r.expires, UndefinedOr::Undefined);
    /// assert_eq!(r.expires.inner(), None);
    ///
    /// let r = r.set_expires(Some(42));
    ///
    /// assert_eq!(r.expires, UndefinedOr::Value(42));
    /// assert_eq!(r.expires.inner(), Some(&42));
    ///
    /// let r = r.set_expires(None);
    ///
    /// assert_eq!(r.expires, UndefinedOr::Null);
    /// assert_eq!(r.expires.inner(), None);
    /// ```
    #[must_use]
    pub fn set_expires(mut self, expires: Option<usize>) -> Self {
        self.expires = expires.into();
        self
    }

    /// Sets or unsets the remaining uses for the key.
    ///
    /// # Arguments
    /// - `remaining`: The number of remaining uses to set or unset.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UpdateKeyRequest;
    /// # use unkey::models::UndefinedOr;
    /// let r = UpdateKeyRequest::new("test");
    ///
    /// assert_eq!(r.remaining, UndefinedOr::Undefined);
    /// assert_eq!(r.remaining.inner(), None);
    ///
    /// let r = r.set_remaining(Some(420));
    ///
    /// assert_eq!(r.remaining, UndefinedOr::Value(420));
    /// assert_eq!(r.remaining.inner(), Some(&420));
    ///
    /// let r = r.set_remaining(None);
    ///
    /// assert_eq!(r.remaining, UndefinedOr::Null);
    /// assert_eq!(r.remaining.inner(), None);
    /// ```
    #[must_use]
    pub fn set_remaining(mut self, remaining: Option<usize>) -> Self {
        self.remaining = remaining.into();
        self
    }

    /// Sets or unsets the ratelimit for the key.
    ///
    /// # Arguments
    /// - `ratelimit`: The ratelimit to set or unset.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UpdateKeyRequest;
    /// # use unkey::models::Ratelimit;
    /// # use unkey::models::RatelimitType;
    /// # use unkey::models::UndefinedOr;
    /// let r = UpdateKeyRequest::new("test");
    ///
    /// assert_eq!(r.ratelimit, UndefinedOr::Undefined);
    /// assert_eq!(r.ratelimit.inner(), None);
    ///
    /// let ratelimit = Ratelimit::new(
    ///     RatelimitType::Fast,
    ///     10,
    ///     10000,
    ///     100
    /// );
    ///
    /// let r = r.set_ratelimit(Some(ratelimit.clone()));
    ///
    /// assert_eq!(r.ratelimit, UndefinedOr::Value(ratelimit.clone()));
    /// assert_eq!(r.ratelimit.inner(), Some(&ratelimit));
    ///
    /// let r = r.set_ratelimit(None);
    ///
    /// assert_eq!(r.ratelimit, UndefinedOr::Null);
    /// assert_eq!(r.ratelimit.inner(), None);
    /// ```
    #[must_use]
    pub fn set_ratelimit(mut self, ratelimit: Option<Ratelimit>) -> Self {
        self.ratelimit = ratelimit.into();
        self
    }

    /// Sets or unsets the refill for the key.
    ///
    /// # Arguments
    /// - `refill`: The refill to set or unset.
    ///
    /// # Returns
    /// Self for chained calls.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UpdateKeyRequest;
    /// # use unkey::models::Refill;
    /// # use unkey::models::RefillInterval;
    /// # use unkey::models::UndefinedOr;
    /// let r = UpdateKeyRequest::new("test");
    ///
    /// assert_eq!(r.ratelimit, UndefinedOr::Undefined);
    /// assert_eq!(r.ratelimit.inner(), None);
    ///
    /// let refill = Refill::new(100, RefillInterval::Daily);
    ///
    /// let r = r.set_refill(Some(refill.clone()));
    ///
    /// assert_eq!(r.refill, UndefinedOr::Value(refill.clone()));
    /// assert_eq!(r.refill.inner(), Some(&refill));
    ///
    /// let r = r.set_refill(None);
    ///
    /// assert_eq!(r.refill, UndefinedOr::Null);
    /// assert_eq!(r.refill.inner(), None);
    /// ```
    #[must_use]
    pub fn set_refill(mut self, refill: Option<Refill>) -> Self {
        self.refill = refill.into();
        self
    }
}

/// An outgoing get key request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetKeyRequest {
    /// The unique id of the key to get.
    pub key_id: String,
}

impl GetKeyRequest {
    /// Creates a new get key request.
    ///
    /// # Arguments
    /// - `key_id`: The id of the key to get.
    ///
    /// # Returns
    /// The get key request.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::GetKeyRequest;
    /// let r = GetKeyRequest::new("test_ABC123");
    ///
    /// assert_eq!(r.key_id, String::from("test_ABC123"));
    /// ```
    #[must_use]
    #[rustfmt::skip]
    pub fn new<T: Into<String>>(key_id: T) -> Self {
        Self { key_id: key_id.into() }
    }
}

/// An outgoing update remaining request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRemainingRequest {
    /// The unique id of the key to updating remaining for.
    pub key_id: String,

    /// The value to perform the operation on.
    pub value: Option<usize>,

    /// The update operation to perform.
    pub op: UpdateOp,
}

impl UpdateRemainingRequest {
    /// Creates a new update remaining request.
    ///
    /// # Arguments
    /// - `key_id`: The id of the key to update remaining for.
    /// - `value`: The value to perform the operation on.
    /// - `op`: The update operation to perform.
    ///
    /// # Returns
    /// The update remaining request.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UpdateRemainingRequest;
    /// # use unkey::models::UpdateOp;
    /// let r = UpdateRemainingRequest::new("test_ABC123", Some(100), UpdateOp::Set);
    ///
    /// assert_eq!(r.key_id, String::from("test_ABC123"));
    /// assert_eq!(r.value.unwrap(), 100);
    /// assert_eq!(r.op, UpdateOp::Set);
    /// ```
    #[must_use]
    #[rustfmt::skip]
    pub fn new<T: Into<String>>(key_id: T, value: Option<usize>, op: UpdateOp) -> Self {
        Self { key_id: key_id.into(), value, op }
    }
}

/// An incoming update remaining request.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRemainingResponse {
    /// The number of remaining verifications for the key.
    pub remaining: usize,
}
