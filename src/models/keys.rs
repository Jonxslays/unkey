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

#[derive(Debug, Clone, Serialize)]
pub struct CreateKeyRequest {
    #[serde(rename = "apiId")]
    api_id: String,

    #[serde(rename = "ownerId")]
    owner_id: String,

    #[serde(rename = "byteLength")]
    byte_length: Option<usize>,

    prefix: String,

    name: Option<String>,

    meta: Option<Value>,

    expires: Option<usize>,

    remaining: Option<usize>,

    ratelimit: Option<Ratelimit>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateKeyResponse {
    #[serde(rename = "keyId")]
    pub key_id: String,

    pub key: String,
}
