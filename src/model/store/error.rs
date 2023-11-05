use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    FailToCreatePool(String),
}
