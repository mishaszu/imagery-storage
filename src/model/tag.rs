use diesel::prelude::*;
use diesel::{
    prelude::{Identifiable, Insertable, Queryable},
    query_builder::AsChangeset,
    ExpressionMethods, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::tag;

use super::{ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = tag)]
pub struct Tag {
    id: Uuid,
    name: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = tag)]
pub struct TagForCreate {
    id: Uuid,
    name: String,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = tag)]
pub struct TagForUpdate {
    pub name: Option<String>,
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
}
