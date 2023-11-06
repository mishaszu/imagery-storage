use crate::web::middleware::CtxExtError;
use crate::{crypt, model, web};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    // LoginFailUsernameNotFound,
    // LoginFailUserHasNoPwd { user_id: Uuid },
    LoginFailPwdNotMatching { user_id: String },
    FailedToRedirect,
    LustBadStatus(u16),

    // -- CtxExtError
    CtxExt(web::middleware::CtxExtError),

    // -- Modules
    Model(model::Error),
    FailedToReadFile,
    Crypt(crypt::Error),

    ServiceError(String),

    // -- External Modules
    SerdeJson(String),

    BadUuidFormat,
    // EntityNotFound,
}

// region:    --- Froms
impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Error::Model(val)
    }
}

impl From<crypt::Error> for Error {
    fn from(val: crypt::Error) -> Self {
        Self::Crypt(val)
    }
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Self::SerdeJson(val.to_string())
    }
}

impl From<uuid::Error> for Error {
    fn from(_val: uuid::Error) -> Self {
        Self::BadUuidFormat
    }
}

impl From<crate::services::error::Error> for Error {
    fn from(val: crate::services::error::Error) -> Self {
        match val {
            crate::services::error::Error::LustBadStatus(s) => Self::LustBadStatus(s),
            _ => Self::ServiceError("Service error".to_string()),
        }
    }
}
// endregion: --- Froms

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        let mut response = match self {
            Error::LustBadStatus(status) if status == 404 => StatusCode::NOT_FOUND.into_response(),
            Error::LustBadStatus(status) if status == 400 => {
                StatusCode::BAD_REQUEST.into_response()
            }
            Error::CtxExt(_) => StatusCode::UNAUTHORIZED.into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        response.extensions_mut().insert(self);

        response
    }
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
