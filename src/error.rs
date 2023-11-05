use std::fmt;

use serde::Serialize;
use serde_with::serde_as;

use crate::model;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EnvVar(String, String),
    Model(model::Error),
    Encryption(String),
    ConfigWrongFormat(String),
}

impl From<model::Error> for Error {
    fn from(e: model::Error) -> Self {
        Self::Model(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
