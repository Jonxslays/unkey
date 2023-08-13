use serde::{Deserialize, Serialize};

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

// impl TryFrom<String> for ErrorCode {
//     type Error = String;

//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         Self::try_from(value.as_str())
//     }
// }

// impl TryFrom<&str> for ErrorCode {
//     type Error = String;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value {
//             "NOT_FOUND" => Ok(ErrorCode::NotFound),
//             "FORBIDDEN" => Ok(ErrorCode::Forbidden),
//             "BAD_REQUEST" => Ok(ErrorCode::BadRequest),
//             "RATELIMITED" => Ok(ErrorCode::Ratelimited),
//             "UNAUTHORIZED" => Ok(ErrorCode::Unauthorized),
//             "USAGE_EXCEEDED" => Ok(ErrorCode::UsageExceeded),
//             "INTERNAL_SERVER_ERROR" => Ok(ErrorCode::InternalServerError),
//             _ => Err(format!("Unknown Error Code: {:?}", value)),
//         }
//     }
// }

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
