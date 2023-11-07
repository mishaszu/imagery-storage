use crate::config;

use self::store::{DbConn, Pool};

pub mod account;
pub mod album;
pub mod comment;
pub mod error;
pub mod favorite_image;
pub mod image;
pub mod post;
pub mod referral;
mod store;
pub mod sys_config;
pub mod tag;
pub mod user;

pub use self::error::{Error, Result};

#[derive(Clone)]
pub struct ModelManager {
    db: Pool,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = store::new_db_pool(&config().DB_URL).await?;

        Ok(Self { db })
    }

    pub fn run_migration(&self) {
        let mut conn = self.db.get().unwrap();
        store::run_migration(&mut conn);
    }

    pub(in crate::model) fn conn(&self) -> Result<DbConn> {
        self.db.get().map_err(|_| Error::DbPoolConnectionFailed)
    }
}
