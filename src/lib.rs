mod client;
mod logging;

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
/// The error variant of response.
macro_rules! response_error {
    ($code:expr, $err:expr) => {
        crate::types::Response::Err(crate::types::HttpError::new($code, $err.to_string()))
    };
}

/// Unwinds the http result into a [`Response<T>`].
///
/// # Arguments
/// - `result`: The http result from the request.
///
/// # Returns
/// A result containing the response or an error.
pub(crate) async fn unwind_response<T: for<'a> Deserialize<'a>>(result: HttpResult) -> Response<T> {
    let data = match result {
        Ok(r) => r.text().await,
        Err(e) => {
            logging::error!(format!("HTTP request failed: {}", e.to_string()));
            Err(e)
        }
    };

    match data {
        Err(e) => response_error!(ErrorCode::Unknown, e),
        Ok(text) => {
            logging::debug!(format!("INCOMING: {text}"));

            match serde_json::from_str::<Response<T>>(&text) {
                Err(e) => response_error!(ErrorCode::Unknown, e),
                Ok(r) => r,
            }
        }
    }
}
