use diesel::prelude::*;
use diesel::{
    query_builder::AsChangeset, ExpressionMethods, Identifiable, Insertable, Queryable, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::account::is_public;
use crate::schema::{album_image, image};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = image)]
pub struct Image {
    id: Uuid,
    user_id: Uuid,
    name: Option<String>,
    description: Option<String>,
    path: Uuid,
    is_public: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = image)]
pub struct ImageForCreate {
    pub id: Uuid,
    user_id: Uuid,
    name: Option<String>,
    description: Option<String>,
    path: Uuid,
}

#[derive(Default, Debug, Clone, AsChangeset)]
#[diesel(table_name = image)]
pub struct ImageForUpdate {
    name: Option<String>,
    description: Option<String>,
    path: Option<Uuid>,
    updated_at: chrono::NaiveDateTime,
}

pub struct ImageBmc;
impl ImageBmc {
    pub fn create(
        mm: &crate::model::ModelManager,
        image: ImageForCreate,
    ) -> crate::model::Result<Image> {
        let mut connection = mm.conn()?;

        diesel::insert_into(image::dsl::image)
            .values(image)
            .get_result::<Image>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get(mm: &crate::model::ModelManager, search_id: Uuid) -> crate::model::Result<Image> {
        let mut connection = mm.conn()?;

        image::dsl::image
            .filter(image::dsl::id.eq(search_id))
            .first::<Image>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn list(
        mm: &crate::model::ModelManager,
        user_id: Option<&Uuid>,
        album_id: Option<&Uuid>,
        is_public: Option<bool>,
    ) -> crate::model::Result<Vec<Image>> {
        let mut connection = mm.conn()?;

        let mut builder = image::dsl::image;

        if let Some(album_id) = album_id {
            builder = builder.inner_join(album_image::dsl::album_image).filter(
                album_image::dsl::album_id
                    .eq(album_id)
                    .and(album_image::dsl::image_id.eq(image::dsl::id)),
            );
        }

        if let Some(user_id) = user_id {
            builder = builder.filter(image::dsl::user_id.eq(user_id));
        }

        if let Some(is_public) = is_public {
            builder = builder.filter(image::dsl::is_public.eq(is_public));
        }

        builder
            .load::<Image>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn update(
        mm: &crate::model::ModelManager,
        id: Uuid,
        image: ImageForUpdate,
    ) -> crate::model::Result<Image> {
        let mut connection = mm.conn()?;

        diesel::update(image::dsl::image.filter(image::dsl::id.eq(id)))
            .set(image)
            .get_result::<Image>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn delete(mm: &crate::model::ModelManager, id: Uuid) -> crate::model::Result<usize> {
        let mut connection = mm.conn()?;

        diesel::delete(image::dsl::image.filter(image::dsl::id.eq(id)))
            .execute(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn delete_many(
        mm: &crate::model::ModelManager,
        ids: Vec<Uuid>,
    ) -> crate::model::Result<()> {
        let mut connection = mm.conn()?;

        connection
            .transaction(|connection| {
                for id in ids.iter() {
                    diesel::delete(image::dsl::image.filter(image::dsl::id.eq(id)))
                        .execute(connection)?;
                }
                diesel::QueryResult::Ok(())
            })
            .map_err(|e| -> crate::model::Error { e.into() })
    }
}
