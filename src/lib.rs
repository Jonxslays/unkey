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

/// Wraps the http result.
///
/// # Arguments
/// - `result`: The http result from the request.
///
/// # Returns
/// The wrapped response or an error.
pub(crate) async fn wrap_response<T>(result: HttpResult) -> Wrapped<T>
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

/// Wraps the http result for an empty return value.
///
/// # Arguments
/// - `result`: The http result from the request.
///
/// # Returns
/// The wrapped response or an error.
pub(crate) async fn wrap_empty_response(result: HttpResult) -> Wrapped<()> {
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

            match serde_json::from_str::<Wrapped<()>>(&text) {
                Ok(r) => r,
                Err(e) => {
                    if text.contains("error") {
                        // If the text contains error and we failed to deserialize
                        // it means the error struct is misaligned with the api
                        response_error!(ErrorCode::Unknown, e)
                    } else {
                        // Otherwise it was successful even though we are in Err
                        // due to serde failing to deserialize a unit type
                        Wrapped::Ok(())
                    }
                }
            }
        }
    }
}

macro_rules! fetch {
    ($http:expr, $route:ident) => {
        $http.fetch($route, None::<u8>)
    };
    ($http:expr, $route:ident, $payload:expr) => {
        $http.fetch($route, Some($payload))
    };
}

pub(crate) use fetch;
