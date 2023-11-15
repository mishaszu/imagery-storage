use diesel::result::Error as DieselError;
use serde::Serialize;
use serde_with::serde_as;
use tracing::debug;
use uuid::Uuid;

use super::store;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    Store(store::Error),

    // diesel / database errors
    DbError,
    DbEntityNotFound,
    DBEntityAlreadyExists,
    DbRollbackTransaction,
    DbQueryBuilderError,
    DbSerializationError,
    DbDeserializationError,

    DbPoolConnectionFailed,

    AccessDenied,
    AccessDeniedReturnNoInfo,

    EntityNotFound { entity: &'static str, id: Uuid },
}

impl From<store::Error> for Error {
    fn from(e: store::Error) -> Self {
        Error::Store(e)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: DieselError) -> Self {
        debug!("{:<12}Diesel error: {:?}", "MODEL ERROR", e);
        match e {
            DieselError::NotFound => Error::DbEntityNotFound,
            DieselError::DatabaseError(kind, _) => match kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => Error::DBEntityAlreadyExists,
                _ => Error::DbError,
            },
            DieselError::RollbackTransaction => Error::DbRollbackTransaction,
            DieselError::QueryBuilderError(_) => Error::DbQueryBuilderError,
            DieselError::SerializationError(_) => Error::DbSerializationError,
            DieselError::DeserializationError(_) => Error::DbDeserializationError,

            _ => Error::DbError,
        }
    }
}
