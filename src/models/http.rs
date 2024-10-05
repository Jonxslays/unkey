use serde::Deserialize;

/// A low level http result representation.
pub(crate) type HttpResult = Result<reqwest::Response, reqwest::Error>;

/// An error code returned by the unkey api.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    /// This is a valid resource - no error.
    Valid,

    /// Resource not found.
    NotFound,

    /// Request forbidden.
    Forbidden,

    /// Bad request payload.
    BadRequest,

    /// You are ratelimited.
    RateLimited,

    /// Not authorized for resource.
    Unauthorized,

    /// The resource has exceeded its usage.
    UsageExceeded,

    /// An internal server error occurred with the api.
    InternalServerError,

    /// An invalid key type was used (shouldn't happen usually).
    InvalidKeyType,

    /// The identifier is in use by another resource.
    NotUnique,

    /// Another resource already uses this identifier.
    Conflict,

    /// The resource is delete protected.
    DeleteProtected,

    /// The resource is expired.
    Expired,

    /// The resource is disabled.
    Disabled,

    /// You have made too many requests.
    TooManyRequests,

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
#[must_use = "this `Wrapped` result may be an `Err` variant, which should be handled"]
pub(crate) enum Wrapped<T> {
    /// The error value.
    #[serde(rename = "error")]
    Err(HttpError),

    /// The ok value.
    #[serde(untagged)]
    Ok(T),
}

impl<T> From<Wrapped<T>> for Result<T, HttpError> {
    fn from(wrapped: Wrapped<T>) -> Self {
        match wrapped {
            Wrapped::Err(err) => Err(err),
            Wrapped::Ok(res) => Ok(res),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ErrorCode;
    use super::HttpError;
    use super::Wrapped;

    #[test]
    fn test_from_wrapped_ok() {
        let wrapped = Wrapped::Ok(120);
        let result: Result<_, HttpError> = wrapped.into();

        assert_eq!(result.unwrap(), 120);
    }

    #[test]
    fn test_from_wrapped_err() {
        let err = HttpError::new(ErrorCode::Conflict, "test".to_string());
        let wrapped = Wrapped::Err(err.clone());
        let result: Result<u8, HttpError> = wrapped.into();

        assert_eq!(result.unwrap_err(), err);
    }
}
