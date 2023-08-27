use serde::Deserialize;

/// A low level http result representation.
pub(crate) type HttpResult = Result<reqwest::Response, reqwest::Error>;

/// An error code returned by the unkey api.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    /// Resource not found.
    NotFound,

    /// Request forbidden.
    Forbidden,

    /// Bad request payload.
    BadRequest,

    /// You are ratelimited.
    Ratelimited,

    /// Not authorized for resource.
    Unauthorized,

    /// You have exceeded your usage.
    UsageExceeded,

    /// An internal server error occurred with the api.
    InternalServerError,

    /// Reserved for unknown interactions.
    #[serde(other)]
    Unknown,
}

/// An http error representation.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct HttpError {
    /// The error code for the error.
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
    /// The new http error.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::HttpError;
    /// # use unkey::models::ErrorCode;
    /// let e = HttpError {
    ///     code: ErrorCode::Unknown,
    ///     message: String::from("err")
    /// };
    ///
    /// assert_eq!(e.code, ErrorCode::Unknown);
    /// assert_eq!(e.message, String::from("err"));
    /// ```
    #[must_use]
    pub(crate) fn new(code: ErrorCode, message: String) -> Self {
        Self { code, message }
    }
}

/// A wrapper around the response type or an error.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum Wrapped<T> {
    /// The error value.
    #[serde(rename = "error")]
    Err(HttpError),

    /// The ok value.
    #[serde(untagged)]
    Ok(T),
}
