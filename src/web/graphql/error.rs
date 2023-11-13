use std::fmt::Display;

use derive_more::Display;
use tracing::debug;

use crate::model::error::Error as ModelError;

#[derive(Debug)]
pub enum Error {
    ModalManagerNotInContext,
    ClientNotInContext,

    ServerError(crate::services::error::Error),

    FailedToReadFile,

    ModelError(ModelError),

    AuthError,
    AccessError(String),

    FailedToEncryptPassword,

    NotFound(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug!("{:<12} - graphql error - {self:?}", "GRAPHQL");
        match self {
            Error::AuthError => write!(f, "User not logged in"),
            Error::ModelError(ModelError::DbEntityNotFound)
            | Error::AccessError(_)
            | Error::NotFound(_) => write!(f, "Not found"),
            Error::ServerError(_)
            | Error::ModelError(_)
            | Error::ClientNotInContext
            | Error::FailedToReadFile
            | Error::FailedToEncryptPassword
            | Error::ModalManagerNotInContext => write!(f, "Internal server error"),
        }
    }
}

// impl Into<async_graphql::Error> for Error {
//     fn into(self) -> async_graphql::Error {
//         debug!("{:<12} - graphql error - {self:?}", "GRAPHQL");
//         match self {
//             Error::AuthError => async_graphql::Error::new("User not logged in"),
//             Error::NotFound => async_graphql::Error::new("Not found"),
//             Error::ModelError(crate::model::error::Error::DbEntityNotFound) => {
//                 async_graphql::Error::new("Not found")
//             }
//             Error::ModelError(crate::model::error::Error::DBEntityAlreadyExists) => {
//                 async_graphql::Error::new("Already exists")
//             }
//             Error::ServerError(_)
//             | Error::ClientNotInContext
//             | Error::ModelError(_)
//             | Error::FailedToReadFile
//             | Error::FailedToEncryptPassword
//             | Error::ModalManagerNotInContext => async_graphql::Error::new("Internal server error"),
//         }
//     }
// }

impl From<ModelError> for Error {
    fn from(e: ModelError) -> Self {
        Error::ModelError(e)
    }
}
