use diesel::prelude::*;
use diesel::{
    query_builder::AsChangeset, ExpressionMethods, Identifiable, Insertable, Queryable, RunQueryDsl,
};
use serde::Serialize;
use uuid::Uuid;

use crate::graphql::guard::HasAccess;
use crate::schema::{image, post_image};

use super::account::AccountBmc;
use super::Result;

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable, Serialize)]
#[diesel(table_name = image)]
pub struct Image {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Option<String>,
    pub kind: String,
    pub path: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = image)]
pub struct ImageForCreate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Option<String>,
    pub kind: String,
    pub path: Uuid,
}

#[derive(Default, Debug, Clone, AsChangeset)]
#[diesel(table_name = image)]
pub struct ImageForUpdate {
    pub name: Option<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct ImageBmc;
impl ImageBmc {
    pub fn create(mm: &crate::model::ModelManager, image: ImageForCreate) -> Result<Image> {
        let mut connection = mm.conn()?;

        diesel::insert_into(image::dsl::image)
            .values(image)
            .get_result::<Image>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get(mm: &crate::model::ModelManager, search_id: &Uuid) -> Result<Image> {
        let mut connection = mm.conn()?;

        image::dsl::image
            .filter(image::dsl::id.eq(search_id))
            .first::<Image>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn list(mm: &crate::model::ModelManager) -> Result<Vec<Image>> {
        let mut connection = mm.conn()?;

        image::dsl::image
            .load::<Image>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn list_user(mm: &crate::model::ModelManager, user_id: &Uuid) -> Result<Vec<Image>> {
        let mut connection = mm.conn()?;

        image::dsl::image
            .filter(image::dsl::user_id.eq(user_id))
            .load::<Image>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn list_post(mm: &crate::model::ModelManager, post_id: &Uuid) -> Result<Vec<Image>> {
        let mut connection = mm.conn()?;

        image::dsl::image
            .inner_join(post_image::dsl::post_image)
            .filter(post_image::dsl::post_id.eq(post_id))
            .select(image::all_columns)
            .load::<Image>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn update(
        mm: &crate::model::ModelManager,
        id: Uuid,
        image: ImageForUpdate,
    ) -> Result<Image> {
        let mut connection = mm.conn()?;

        diesel::update(image::dsl::image.filter(image::dsl::id.eq(id)))
            .set(image)
            .get_result::<Image>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn delete(mm: &crate::model::ModelManager, id: Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;

        diesel::delete(image::dsl::image.filter(image::dsl::id.eq(id)))
            .execute(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }
}

impl HasAccess for ImageBmc {
    fn is_allowed(
        mm: &super::ModelManager,
        allow_admin: bool,
        resource_id: &Uuid,
        user: &Uuid,
    ) -> crate::model::error::Result<bool> {
        let mut connection = mm.conn()?;

        let account = AccountBmc::get_by_user_id(mm, user)?;

        if account.is_banned {
            return Err(crate::model::Error::AccessDenied);
        }

        if account.is_admin && allow_admin {
            return Ok(true);
        }

        let query = image::dsl::image
            .filter(image::dsl::id.eq(resource_id))
            .filter(image::dsl::user_id.eq(user));

        diesel::select(diesel::dsl::exists(query)).get_result::<bool>(&mut connection)?;
        Ok(true)
    }
}
