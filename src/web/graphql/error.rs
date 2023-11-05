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
}

impl Error {
    pub fn from_model_to_graphql(e: ModelError) -> async_graphql::Error {
        let e1: Error = e.into();
        e1.into()
    }
}

impl Into<async_graphql::Error> for Error {
    fn into(self) -> async_graphql::Error {
        debug!("{:<12} - graphql error - {self:?}", "GRAPHQL");
        match self {
            Error::AuthError => async_graphql::Error::new("User not logged in"),
            Error::ModelError(crate::model::error::Error::DbEntityNotFound) => {
                async_graphql::Error::new("Not found")
            }
            Error::ModelError(crate::model::error::Error::DBEntityAlreadyExists) => {
                async_graphql::Error::new("Already exists")
            }
            Error::ServerError(_)
            | Error::ClientNotInContext
            | Error::ModelError(_)
            | Error::FailedToReadFile
            | Error::ModalManagerNotInContext => async_graphql::Error::new("Internal server error"),
        }
    }
}

impl From<ModelError> for Error {
    fn from(e: ModelError) -> Self {
        Error::ModelError(e)
    }
}
