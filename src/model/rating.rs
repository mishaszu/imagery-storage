use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::rating;

use super::{Error, ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = rating)]
pub struct Rating {
    pub id: Uuid,
    pub name: String,
    pub scale: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = rating)]
pub struct RatingForCreate {
    pub id: Uuid,
    pub name: String,
    pub scale: String,
    pub description: String,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = rating)]
pub struct RatingForUpdate {
    pub name: Option<String>,
    pub scale: Option<String>,
    pub description: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct RatingBmc;

impl RatingBmc {
    pub fn create(mm: &ModelManager, rating: RatingForCreate) -> Result<Rating> {
        let mut connection = mm.conn()?;

        diesel::insert_into(rating::dsl::rating)
            .values(rating)
            .get_result::<Rating>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }
    pub fn get(mm: &ModelManager, id: Uuid) -> Result<Rating> {
        let mut connection = mm.conn()?;

        rating::dsl::rating
            .filter(rating::dsl::id.eq(id))
            .first::<Rating>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }
    pub fn list(mm: &ModelManager) -> Result<Vec<Rating>> {
        let mut connection = mm.conn()?;

        rating::dsl::rating
            .load::<Rating>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }
    pub fn update(mm: &ModelManager, id: Uuid, rating: RatingForUpdate) -> Result<Rating> {
        let mut connection = mm.conn()?;

        diesel::update(rating::dsl::rating.filter(rating::dsl::id.eq(id)))
            .set(rating)
            .get_result::<Rating>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }
    pub fn delete(mm: &ModelManager, id: Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;

        diesel::delete(rating::dsl::rating.filter(rating::dsl::id.eq(id)))
            .execute(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }
}
