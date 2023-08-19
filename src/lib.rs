mod client;
mod logging;
pub mod models;
mod routes;
mod services;

use serde::Deserialize;

pub use client::Client;
use models::ErrorCode;
use models::HttpResult;
use models::Wrapped;

/// Creates a new Err variant of [`Response`].
///
/// # Arguments
/// - `$code`: The [`ErrorCode`] for the error.
/// - `$err`: The error (must have `to_string()` impl).
///
/// # Returns
/// The wrapped error.
macro_rules! response_error {
    ($code:expr, $err:expr) => {
        crate::models::Wrapped::Err(crate::models::HttpError::new($code, $err.to_string()))
    };
}

/// Unwinds the http result into a [`Response<T>`].
///
/// # Arguments
/// - `result`: The http result from the request.
///
/// # Returns
/// The wrapped response or an error.
pub(crate) async fn unwind_response<T>(result: HttpResult) -> Wrapped<T>
where
    T: for<'a> Deserialize<'a>,
{
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

            match serde_json::from_str::<Wrapped<T>>(&text) {
                Err(e) => response_error!(ErrorCode::Unknown, e),
                Ok(r) => r,
            }
        }
    }
}
