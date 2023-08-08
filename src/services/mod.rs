mod http;
mod keys;

pub use http::*;
pub use keys::*;
use serde::Deserialize;

use crate::models::ErrorCode;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ServiceResult<T> {
    Ok(T),
    Err(crate::models::HttpError),
}

macro_rules! service_error {
    ($code:expr, $err:expr) => {
        crate::services::ServiceResult::Err(crate::models::HttpError::new($code, $err.to_string()))
    };
}

async fn unwind_response<T: for<'a> Deserialize<'a>>(response: HttpResult) -> ServiceResult<T> {
    if response.is_err() {
        return service_error!(ErrorCode::Unknown, response.unwrap_err());
    }

    let data = response.unwrap().json::<ServiceResult<T>>().await;

    match data {
        Ok(data) => data,
        Err(e) => service_error!(ErrorCode::Unknown, e),
    }
}
