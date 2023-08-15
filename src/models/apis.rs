
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{
    models::{Ratelimit}
};

#[derive(Debug, Clone, Serialize)]
pub struct KeyListRequest{
  #[serde(rename = "apiId")]
  pub api_id: String,
  pub limit: Option<usize>,
  pub offset: Option<usize>,

  #[serde(rename = "ownerId")]
  pub owner_id: Option<String>
}


#[derive(Debug, Clone, Serialize)]
pub struct KeyListResponse{
  pub keys:  KeyListResponseObject,
  pub total: usize
}


// Individual key object returned in the response
#[derive(Debug, Clone, Serialize)]
pub struct KeyListResponseObject{
  pub id: String,

  #[serde(rename="apiId")]
  pub api_id: String,

  #[serde(rename="workspaceId")]
  pub workspace_id:String,

  pub start:String,

  #[serde(rename="ownerId")]
  pub owner_id: String,

  pub meta: Option<Value>,

  #[serde(rename="createdAt")]
  pub created_at: usize,
  pub expires: usize,
  pub remaining: usize,
  
  pub ratelimit: Ratelimit
  
}