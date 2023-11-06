use diesel::prelude::*;
use uuid::Uuid;

use super::{ModelManager, Result};

use crate::schema::follow;

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = follow)]
pub struct Follow {
    id: Uuid,
    follower_id: Uuid,
    followee_id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = follow)]
pub struct FollowForCreate {
    pub id: Uuid,
    follower_id: Uuid,
    followee_id: Uuid,
}

pub struct FollowBmc;

impl FollowBmc {
    pub fn create(mm: &ModelManager, follow: FollowForCreate) -> Result<Follow> {
        let mut connection = mm.conn()?;

        diesel::insert_into(follow::dsl::follow)
            .values(follow)
            .get_result::<Follow>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get(mm: &ModelManager, search_id: &Uuid) -> Result<Follow> {
        let mut connection = mm.conn()?;

        follow::dsl::follow
            .filter(follow::dsl::id.eq(search_id))
            .first::<Follow>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn list(mm: &ModelManager, follower_id: &Uuid) -> Result<Vec<Follow>> {
        let mut connection = mm.conn()?;

        follow::dsl::follow
            .filter(follow::dsl::follower_id.eq(follower_id))
            .load::<Follow>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn delete(mm: &ModelManager, id: &Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;

        diesel::delete(follow::dsl::follow)
            .filter(follow::dsl::id.eq(id))
            .execute(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }
}
