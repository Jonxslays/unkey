use crate::models::Ratelimit;
use serde::{Serialize, Deserialize};
use serde_json::Value;

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

    pub ratelimit: Option<Ratelimit>,
}