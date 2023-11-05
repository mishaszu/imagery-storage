use diesel::{
    prelude::{Identifiable, Insertable, Queryable},
    query_builder::AsChangeset,
    query_dsl::methods::FilterDsl,
    ExpressionMethods, PgTextExpressionMethods, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::tag_category;

use super::{ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = tag_category)]
pub struct TagCategory {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = tag_category)]
pub struct TagCategoryForCreate {
    pub id: Uuid,
    pub name: String,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = tag_category)]
pub struct TagCategoryForUpdate {
    pub name: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct TagCategoryBmc;

impl TagCategoryBmc {
    pub fn create(mm: &ModelManager, tag_category: TagCategoryForCreate) -> Result<TagCategory> {
        let mut connection = mm.conn()?;
        diesel::insert_into(tag_category::dsl::tag_category)
            .values(&tag_category)
            .get_result::<TagCategory>(&mut connection)
            .map_err(|e| e.into())
    }
    pub fn get(mm: &ModelManager, id: Uuid) -> Result<TagCategory> {
        let mut connection = mm.conn()?;
        tag_category::dsl::tag_category
            .filter(tag_category::dsl::id.eq(&id))
            .first::<TagCategory>(&mut connection)
            .map_err(|e| e.into())
    }
    pub fn list(mm: &ModelManager) -> Result<Vec<TagCategory>> {
        let mut connection = mm.conn()?;
        tag_category::dsl::tag_category
            .load::<TagCategory>(&mut connection)
            .map_err(|e| e.into())
    }
    pub fn get_by_name(mm: &ModelManager, name: String) -> Result<Vec<TagCategory>> {
        let mut connection = mm.conn()?;
        tag_category::dsl::tag_category
            .filter(tag_category::dsl::name.ilike(format!("%{}%", name)))
            .load::<TagCategory>(&mut connection)
            .map_err(|e| e.into())
    }
    pub fn update(
        mm: &ModelManager,
        id: Uuid,
        tag_category: TagCategoryForUpdate,
    ) -> Result<TagCategory> {
        let mut connection = mm.conn()?;
        diesel::update(tag_category::dsl::tag_category)
            .filter(tag_category::dsl::id.eq(&id))
            .set(&tag_category)
            .get_result::<TagCategory>(&mut connection)
            .map_err(|e| e.into())
    }
    pub fn delete(mm: &ModelManager, id: Uuid) -> Result<TagCategory> {
        let mut connection = mm.conn()?;
        diesel::delete(tag_category::dsl::tag_category)
            .filter(tag_category::dsl::id.eq(&id))
            .get_result::<TagCategory>(&mut connection)
            .map_err(|e| e.into())
    }
}
