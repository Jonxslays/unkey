mod client;

pub mod models;
pub mod routes;
pub mod services;
pub mod types;

pub use client::Client;
use serde::Deserialize;
use types::{ErrorCode, HttpResult, Response};

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
    if let Err(e) = response {
        return response_error!(ErrorCode::Unknown, e);
    }

    let data = match response {
        Ok(r) => r.json::<Response<T>>().await,
        Err(e) => return response_error!(ErrorCode::Unknown, e),
    };

    match data {
        Ok(data) => data,
        Err(e) => response_error!(ErrorCode::Unknown, e),
    }
}
