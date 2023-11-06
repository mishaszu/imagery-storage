use diesel::prelude::*;
use uuid::Uuid;

use super::{ModelManager, Result};

use crate::schema::fav;

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = fav)]
pub struct Fav {
    id: Uuid,
    user_id: Uuid,
    post_id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = fav)]
pub struct FavForCreate {
    pub id: Uuid,
    user_id: Uuid,
    post_id: Uuid,
}

pub struct FavBmc;

impl FavBmc {
    pub fn create(mm: &ModelManager, fav: FavForCreate) -> Result<Fav> {
        let mut connection = mm.conn()?;

        diesel::insert_into(fav::dsl::fav)
            .values(fav)
            .get_result::<Fav>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get(mm: &ModelManager, search_id: &Uuid) -> Result<Fav> {
        let mut connection = mm.conn()?;

        fav::dsl::fav
            .filter(fav::dsl::id.eq(search_id))
            .first::<Fav>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn list(mm: &ModelManager, user_id: &Uuid) -> Result<Vec<Fav>> {
        let mut connection = mm.conn()?;

        fav::dsl::fav
            .filter(fav::dsl::user_id.eq(user_id))
            .load::<Fav>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn delete(mm: &ModelManager, id: &Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;

        diesel::delete(fav::dsl::fav)
            .filter(fav::dsl::id.eq(id))
            .execute(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }
}
