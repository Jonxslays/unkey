use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct VerifyKeyRequest {
    pub key: String,
}

impl VerifyKeyRequest {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyKeyResponse {
    pub valid: bool,

    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,

    pub meta: Option<Value>,

    pub remaining: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RatelimitType {
    #[serde(rename = "fast")]
    Fast,

    #[serde(rename = "consistent")]
    Consistent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ratelimit {
    #[serde(rename = "type")]
    ratelimit_type: RatelimitType,

    #[serde(rename = "refillRate")]
    refill_rate: usize,

    #[serde(rename = "refillInterval")]
    refill_interval: usize,

    limit: usize,
}

impl Ratelimit {
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

#[derive(Debug, Clone, Serialize)]
pub struct CreateKeyRequest {
    #[serde(rename = "apiId")]
    pub api_id: String,

    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,

    #[serde(rename = "byteLength")]
    pub byte_length: Option<usize>,

    pub prefix: Option<String>,

    pub name: Option<String>,

    pub meta: Option<Value>,

    pub expires: Option<usize>,

    pub remaining: Option<usize>,

    pub ratelimit: Option<Ratelimit>,
}

impl CreateKeyRequest {
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

    pub fn set_owner_id(mut self, owner_id: &str) -> Self {
        self.owner_id = Some(owner_id.to_string());
        return self;
    }

    pub fn set_byte_length(mut self, byte_length: usize) -> Self {
        self.byte_length = Some(byte_length);
        return self;
    }

    pub fn set_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        return self;
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        return self;
    }

    pub fn set_meta(mut self, meta: Value) -> Self {
        self.meta = Some(meta);
        return self;
    }

    /// Sets when this key expires.
    ///
    /// # Arguments:
    ///
    /// expires: The number of milliseconds in the future this key should
    /// expire at.
    ///
    /// # Returns:
    ///     
    ///     The consumed self, for chained calls.
    pub fn set_expires(mut self, expires: usize) -> Self {
        let duration = SystemTime::now()
            .duration_since(SystemTime::from(std::time::UNIX_EPOCH))
            .unwrap();

        let expires = duration.as_millis() as usize + expires;
        self.expires = Some(expires);
        return self;
    }

    pub fn set_remaining(mut self, remaining: usize) -> Self {
        self.remaining = Some(remaining);
        return self;
    }

    pub fn set_ratelimit(mut self, ratelimit: Ratelimit) -> Self {
        self.ratelimit = Some(ratelimit);
        return self;
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateKeyResponse {
    #[serde(rename = "keyId")]
    pub key_id: String,

    pub key: String,
}
