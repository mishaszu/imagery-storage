use diesel::prelude::*;
use diesel::{
    prelude::{Identifiable, Insertable, Queryable},
    query_builder::AsChangeset,
    ExpressionMethods, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::theme;

use super::{ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = theme)]
pub struct Theme {
    id: Uuid,
    name: String,
    color: String,
    picture: Option<Uuid>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = theme)]
pub struct ThemeForCreate {
    id: Uuid,
    name: String,
    color: String,
    picture: Option<Uuid>,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = theme)]
pub struct ThemeForUpdate {
    name: Option<String>,
    color: Option<String>,
    picture: Option<Uuid>,
    updated_at: chrono::NaiveDateTime,
}

pub struct ThemeBmc;

impl ThemeBmc {
    pub fn create(mm: &ModelManager, theme: ThemeForCreate) -> Result<Theme> {
        let mut connection = mm.conn()?;

        diesel::insert_into(theme::dsl::theme)
            .values(theme)
            .get_result::<Theme>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get(mm: &ModelManager, search_id: Uuid) -> Result<Theme> {
        let mut connection = mm.conn()?;

        theme::dsl::theme
            .filter(theme::dsl::id.eq(search_id))
            .first::<Theme>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn list(mm: &ModelManager) -> Result<Vec<Theme>> {
        let mut connection = mm.conn()?;

        theme::dsl::theme
            .load::<Theme>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn update(mm: &ModelManager, search_id: Uuid, theme: ThemeForUpdate) -> Result<Theme> {
        let mut connection = mm.conn()?;

        diesel::update(theme::dsl::theme.filter(theme::dsl::id.eq(search_id)))
            .set(theme)
            .get_result::<Theme>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn delete(mm: &ModelManager, search_id: Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;

        diesel::delete(theme::dsl::theme.filter(theme::dsl::id.eq(search_id)))
            .execute(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }
}
