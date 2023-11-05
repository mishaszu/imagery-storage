use diesel::prelude::*;
use diesel::{
    prelude::{Identifiable, Insertable, Queryable},
    query_builder::AsChangeset,
    ExpressionMethods, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::{album, album_tag, image, image_tag, kitty, kitty_tag, tag};

use super::album::Album;
use super::kitty::Kitty;
use super::{image::Image, Error, ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = tag)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub tag_category_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// create tag for create
#[derive(Insertable)]
#[diesel(table_name = tag)]
pub struct TagForCreate {
    pub id: Uuid,
    pub name: String,
    pub tag_category_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// create tag for update
#[derive(AsChangeset, Insertable)]
#[diesel(table_name = tag)]
pub struct TagForUpdate {
    pub name: Option<String>,
    pub tag_category_id: Option<Uuid>,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct TagBmc;

impl TagBmc {
    pub fn create(mm: &ModelManager, tag: TagForCreate) -> Result<Tag> {
        let mut connection = mm.conn()?;
        diesel::insert_into(tag::dsl::tag)
            .values(&tag)
            .get_result::<Tag>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get(mm: &ModelManager, id: Uuid) -> Result<Tag> {
        let mut connection = mm.conn()?;
        tag::dsl::tag
            .filter(tag::dsl::id.eq(&id))
            .first::<Tag>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_images(mm: &ModelManager, id: Uuid) -> Result<Vec<Image>> {
        let mut connection = mm.conn()?;
        image_tag::dsl::image_tag
            .filter(image_tag::dsl::tag_id.eq(id))
            .inner_join(image::dsl::image)
            .select(image::all_columns)
            .load::<Image>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }

    pub fn get_kitties(mm: &ModelManager, id: Uuid) -> Result<Vec<Kitty>> {
        let mut connection = mm.conn()?;
        kitty_tag::dsl::kitty_tag
            .filter(kitty_tag::dsl::tag_id.eq(id))
            .inner_join(kitty::dsl::kitty)
            .select(kitty::all_columns)
            .load::<Kitty>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }

    pub fn list(mm: &ModelManager) -> Result<Vec<Tag>> {
        let mut connection = mm.conn()?;
        tag::dsl::tag
            .load::<Tag>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_by_name(mm: &ModelManager, name: String) -> Result<Vec<Tag>> {
        let mut connection = mm.conn()?;
        tag::dsl::tag
            .filter(tag::dsl::name.like(format!("%{}%", name)))
            .load::<Tag>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_by_tag_category_id(
        mm: &ModelManager,
        tag_category_id: Uuid,
        name: Option<String>,
    ) -> Result<Vec<Tag>> {
        let mut connection = mm.conn()?;

        match name {
            Some(name) => tag::dsl::tag
                .filter(
                    tag::dsl::tag_category_id
                        .eq(&tag_category_id)
                        .and(tag::dsl::name.ilike(format!("%{}%", name))),
                )
                .load::<Tag>(&mut connection)
                .map_err(|e| e.into()),
            None => tag::dsl::tag
                .filter(tag::dsl::tag_category_id.eq(&tag_category_id))
                .load::<Tag>(&mut connection)
                .map_err(|e| e.into()),
        }
    }

    pub fn update(mm: &ModelManager, id: Uuid, tag: TagForUpdate) -> Result<Tag> {
        let mut connection = mm.conn()?;
        diesel::update(tag::dsl::tag.filter(tag::dsl::id.eq(&id)))
            .set(&tag)
            .get_result::<Tag>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn delete(mm: &ModelManager, id: Uuid) -> Result<Tag> {
        let mut connection = mm.conn()?;
        diesel::delete(tag::dsl::tag.filter(tag::dsl::id.eq(&id)))
            .get_result::<Tag>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_albums(mm: &ModelManager, id: Uuid) -> Result<Vec<Album>> {
        let mut connection = mm.conn()?;
        album_tag::dsl::album_tag
            .filter(album_tag::dsl::tag_id.eq(id))
            .inner_join(album::dsl::album)
            .select(album::all_columns)
            .load::<Album>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }
}
