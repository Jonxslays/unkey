use serde::Deserialize;

use super::models::ErrorCode;

pub type HttpResult = Result<reqwest::Response, reqwest::Error>;

#[derive(Deserialize, Debug, Clone)]
pub enum Response<T> {
    #[serde(rename = "error")]
    Err(crate::models::HttpError),
    #[serde(untagged)]
    Ok(T),
}

#[macro_export]
macro_rules! response_error {
    ($code:expr, $err:expr) => {
        crate::types::Response::Err(crate::models::HttpError::new($code, $err.to_string()))
    };
}

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
