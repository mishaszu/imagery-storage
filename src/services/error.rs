use derive_more::Display;
use serde::Serialize;
use tracing::debug;

use super::lust::LustError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    ReqwestFailed(String),

    LustError(LustError),

    UrlParseFailed(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestFailed(e.to_string())
    }
}

impl From<LustError> for Error {
    fn from(e: LustError) -> Self {
        Error::LustError(e)
    }
}

impl Into<async_graphql::Error> for Error {
    fn into(self) -> async_graphql::Error {
        debug!("{:<12} - {:?}", "SERVICE", self);
        match self {
            Error::ReqwestFailed(e) => async_graphql::Error::new(format!("Reqwest error: {}", e)),
            Error::UrlParseFailed(s) => {
                async_graphql::Error::new(format!("Url parse error: {}", s))
            }
            Error::LustError(e) => async_graphql::Error::new(e.to_string()),
        }
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::ReqwestFailed(e) => format!("Reqwest error: {}", e),
            Error::UrlParseFailed(s) => format!("Url parse error: {}", s),
            Error::LustError(e) => e.to_string(),
        }
    }
}
