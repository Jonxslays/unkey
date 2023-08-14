use serde::{Deserialize, Serialize};

/// A low level http result representation.
pub type HttpResult = Result<reqwest::Response, reqwest::Error>;

/// An error code returned by the unkey api.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ErrorCode {
    /// Resource not found.
    #[serde(rename = "NOT_FOUND")]
    NotFound,

    /// Request forbidden.
    #[serde(rename = "FORBIDDEN")]
    Forbidden,

    /// Bad request payload.
    #[serde(rename = "BAD_REQUEST")]
    BadRequest,

    /// You are ratelimited.
    #[serde(rename = "RATELIMITED")]
    Ratelimited,

    /// Not authorized for resource.
    #[serde(rename = "UNAUTHORIZED")]
    Unauthorized,

    /// You have exceeded your usage.
    #[serde(rename = "USAGE_EXCEEDED")]
    UsageExceeded,

    /// An internal server error occurred with the api.
    #[serde(rename = "INTERNAL_SERVER_ERROR")]
    InternalServerError,

    /// Reserved for unknown interactions.
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

/// An http error representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpError {
    /// The [`ErrorCode`] for the error.
    pub code: ErrorCode,

    /// The error message.
    pub message: String,
}

impl HttpError {
    /// Creates a new http error.
    ///
    /// # Arguments
    /// - `code`: The [`ErrorCode`] for the error.
    /// - `message`: The error message.
    ///
    /// # Returns
    /// - [`Self`]: The new http error.
    ///
    /// # Example
    /// ```
    /// # use unkey_sdk::types::HttpError;
    /// # use unkey_sdk::types::ErrorCode;
    /// let e = HttpError::new(ErrorCode::Unknown, String::from("err"));
    ///
    /// assert_eq!(e.code, ErrorCode::Unknown);
    /// assert_eq!(e.message, String::from("err"));
    /// ```
    pub fn new(code: ErrorCode, message: String) -> Self {
        Self { code, message }
    }
}

/// A generic response type the client returns.
#[derive(Deserialize, Debug, Clone)]
pub enum Response<T> {
    /// The error value.
    #[serde(rename = "error")]
    Err(HttpError),

    /// The ok value.
    #[serde(untagged)]
    Ok(T),
}

/// Creates a new Err variant of [`Response`].
///
/// # Arguments
/// - `$code`: The [`ErrorCode`] for the error.
/// - `$err`: The error (must have `to_string()` impl).
///
/// # Returns
/// - [`Response::Err`]: The error.
macro_rules! response_error {
    ($code:expr, $err:expr) => {
        crate::types::Response::Err(crate::types::HttpError::new($code, $err.to_string()))
    };
}

/// Unwinds the http result into a [`Response<T>`].
///
/// # Arguments
/// - `response`: The [`HttpResult`] from the request.
///
/// # Returns
/// - [`Response<T>`]: The response or an error.
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
