mod http;
mod keys;

pub use http::*;
pub use keys::*;

pub type ServiceResult<T> = Result<T, crate::models::HttpError>;
