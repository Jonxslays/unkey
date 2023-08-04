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
