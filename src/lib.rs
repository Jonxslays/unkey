mod client;

pub mod models;
pub mod routes;
pub mod services;
pub mod types;

pub use client::Client;
use serde::Deserialize;
use types::{ErrorCode, HttpResult, Response};

lazy_static::lazy_static! {
    /// An environment variable that can be set to include debug output.
    pub static ref UNKEY_DEBUG: bool = match option_env!("UNKEY_DEBUG") {
        Some(_) => true,
        None => false,
    };
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
    let data = match response {
        Err(e) => Err(e),
        Ok(r) => r.text().await,
    };

    match data {
        Err(e) => response_error!(ErrorCode::Unknown, e),
        Ok(text) => {
            if *UNKEY_DEBUG {
                println!("[DEBUG]: {text}");
            }

            match serde_json::from_str::<Response<T>>(&text) {
                Err(e) => response_error!(ErrorCode::Unknown, e),
                Ok(r) => r,
            }
        }
    }
}
