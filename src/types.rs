use serde::{Deserialize, Serialize};

pub type HttpResult = Result<reqwest::Response, reqwest::Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "NOT_FOUND")]
    NotFound,

    #[serde(rename = "FORBIDDEN")]
    Forbidden,

    #[serde(rename = "BAD_REQUEST")]
    BadRequest,

    #[serde(rename = "RATELIMITED")]
    Ratelimited,

    #[serde(rename = "UNAUTHORIZED")]
    Unauthorized,

    #[serde(rename = "USAGE_EXCEEDED")]
    UsageExceeded,

    #[serde(rename = "INTERNAL_SERVER_ERROR")]
    InternalServerError,

    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpError {
    pub code: ErrorCode,
    pub message: String,
}

impl HttpError {
    pub fn new(code: ErrorCode, message: String) -> Self {
        Self { code, message }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum Response<T> {
    #[serde(rename = "error")]
    Err(HttpError),
    #[serde(untagged)]
    Ok(T),
}

#[macro_export]
macro_rules! response_error {
    ($code:expr, $err:expr) => {
        crate::types::Response::Err(crate::types::HttpError::new($code, $err.to_string()))
    };
}

pub async fn unwind_response<T: for<'a> Deserialize<'a>>(response: HttpResult) -> Response<T> {
    if response.is_err() {
        return response_error!(ErrorCode::Unknown, response.unwrap_err());
    }

    let data = response.unwrap().json::<Response<T>>().await;

    match data {
        Ok(data) => data,
        Err(e) => response_error!(ErrorCode::Unknown, e),
    }
}
