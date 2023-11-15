use diesel::prelude::*;
use uuid::Uuid;

use super::{ModelManager, Result};

use crate::schema::sys_config;

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = sys_config)]
pub struct SysConfig {
    id: Uuid,
    allow_registration: bool,
    single_user_feed: Option<Uuid>,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = sys_config)]
pub struct SysConfigForUpdate {
    allow_registration: Option<bool>,
    single_user_feed: Option<Uuid>,
}

pub struct SysConfigBmc;

impl SysConfigBmc {
    pub fn get(mm: &ModelManager) -> Result<SysConfig> {
        let mut connection = mm.conn()?;

        sys_config::dsl::sys_config
            .first::<SysConfig>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn update(mm: &ModelManager, sys_config: SysConfigForUpdate) -> Result<SysConfig> {
        let mut connection = mm.conn()?;

        diesel::update(sys_config::dsl::sys_config)
            .set(sys_config)
            .get_result::<SysConfig>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }
}
