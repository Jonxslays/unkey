use crate::models::Ratelimit;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use super::RatelimitType;

#[derive(Debug, Clone, Serialize)]
pub struct ListKeysRequest {
    #[serde(rename = "apiId")]
    pub api_id: String,

    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,

    pub limit: Option<usize>,

    pub offset: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListKeysResponse {
    pub keys: Vec<ApiKey>,

    pub total: usize,
}

// Individual key object returned in the response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,

    #[serde(rename = "apiId")]
    pub api_id: String,

    #[serde(rename = "workspaceId")]
    pub workspace_id: String,

    pub start: String,

    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,

    pub meta: Option<Value>,

    #[serde(rename = "createdAt")]
    pub created_at: usize,

    pub expires: Option<usize>,

    pub remaining: Option<usize>,

    pub ratelimit: Option<mRatelimit>,
}

/// Ratelimit copied for debugging

/// A ratelimit imposed on an api key.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct mRatelimit {
    /// The type for this ratelimit.
    /// 
    /// Change this to RatelimitType and the code will error
    #[serde(rename = "type")]
    pub ratelimit_type: String,

    /// The rate at which the ratelimit refills, per interval.
    #[serde(rename = "refillRate")]
    pub refill_rate: usize,

    // /// The interval at which to refill, in milliseconds.
    #[serde(rename = "refillInterval")]
    pub refill_interval: usize,

    /// Total number of burstable requests.
    pub limit: usize,
}