#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

mod client;
mod logging;
pub mod models;
mod routes;
mod services;

use models::HttpError;
use serde::Deserialize;

pub use client::Client;
use models::ErrorCode;
use models::HttpResult;
use models::Wrapped;

/// Creates a new Err variant of [`Wrapped`].
///
/// # Arguments
/// - `$code`: The [`ErrorCode`] for the error.
/// - `$err`: The error (must have `to_string()` impl).
///
/// # Returns
/// The wrapped error.
macro_rules! response_error {
    ($code:expr, $err:expr) => {
        ::std::result::Result::Err($crate::models::HttpError::new($code, $err.to_string()))
    };
}

/// Parses the http result.
///
/// # Arguments
/// - `result`: The http result from the request.
///
/// # Returns
/// A [`Result`] containing the response, or an error.
///
/// # Errors
/// The [`HttpError`], if one occurred.
pub(crate) async fn parse_response<T>(result: HttpResult) -> Result<T, HttpError>
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
                Ok(r) => r.into(),
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
/// A [`Result`] containing the response, or an error.
///
/// # Errors
/// The [`HttpError`], if one occurred.
pub(crate) async fn parse_empty_response(result: HttpResult) -> Result<(), HttpError> {
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
                Ok(r) => r.into(),
                Err(e) => {
                    if text.contains("error") {
                        // If the text contains error and we failed to deserialize
                        // it means the error struct is misaligned with the api
                        response_error!(ErrorCode::Unknown, e)
                    } else {
                        // Otherwise it was successful even though we are in Err
                        // due to serde failing to deserialize a unit type
                        Ok(())
                    }
                }
            }
        }
    }
}

/// Fetches the given route with the provided http service.
macro_rules! fetch {
    ($http:expr, $route:ident) => {
        $http.fetch($route, None::<u8>)
    };
    ($http:expr, $route:ident, $payload:expr) => {
        $http.fetch($route, Some($payload))
    };
}

pub(crate) use fetch;

#[cfg(test)]
mod test {
    use crate::models::ErrorCode;
    use crate::models::HttpError;

    struct FakeHttp;

    impl FakeHttp {
        pub fn fetch(&self, route: u8, payload: Option<u8>) -> u8 {
            let mut res = route;
            if let Some(p) = payload {
                res += p;
            }

            res
        }
    }

    #[test]
    fn response_error() {
        let res: Result<(), _> = response_error!(ErrorCode::NotFound, "not found!");

        assert_eq!(
            res,
            Err(HttpError::new(
                ErrorCode::NotFound,
                String::from("not found!")
            ))
        );
    }

    #[test]
    fn fetch_no_payload() {
        let route = 69;
        let res = fetch!(FakeHttp, route);

        assert_eq!(res, 69);
    }

    #[test]
    fn fetch_with_payload() {
        let route = 69;
        let res = fetch!(FakeHttp, route, 1);

        assert_eq!(res, 70);
    }
}
