use diesel::prelude::*;
use uuid::Uuid;

use super::{ModelManager, Result};

use crate::schema::comment;

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = comment)]
pub struct Comment {
    id: Uuid,
    user_id: Uuid,
    post_id: Uuid,
    body: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = comment)]
pub struct CommentForCreate {
    pub id: Uuid,
    user_id: Uuid,
    post_id: Uuid,
    body: String,
}

#[derive(Default, Debug, Clone, AsChangeset)]
#[diesel(table_name = comment)]
pub struct CommentForUpdate {
    body: Option<String>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct CommentBmc;

impl CommentBmc {
    pub fn create(mm: &ModelManager, comment: CommentForCreate) -> Result<Comment> {
        let mut connection = mm.conn()?;

        diesel::insert_into(comment::dsl::comment)
            .values(comment)
            .get_result::<Comment>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get(mm: &ModelManager, search_id: &Uuid) -> Result<Comment> {
        let mut connection = mm.conn()?;

        comment::dsl::comment
            .filter(comment::dsl::id.eq(search_id))
            .first::<Comment>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn list(mm: &ModelManager, post_id: &Uuid) -> Result<Vec<Comment>> {
        let mut connection = mm.conn()?;

        comment::dsl::comment
            .filter(comment::dsl::post_id.eq(post_id))
            .load::<Comment>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn update(
        mm: &ModelManager,
        search_id: &Uuid,
        comment: CommentForUpdate,
    ) -> Result<Comment> {
        let mut connection = mm.conn()?;

        diesel::update(comment::dsl::comment.filter(comment::dsl::id.eq(search_id)))
            .set(comment)
            .get_result::<Comment>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn delete(mm: &ModelManager, search_id: &Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;

        diesel::delete(comment::dsl::comment.filter(comment::dsl::id.eq(search_id)))
            .execute(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }
}
