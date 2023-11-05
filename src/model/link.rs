use diesel::{
    query_builder::AsChangeset, query_dsl::methods::FilterDsl, ExpressionMethods, Identifiable,
    Insertable, Queryable, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::link;

use super::{Error, ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = link)]
pub struct Link {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// Link for create
#[derive(Insertable)]
#[diesel(table_name = link)]
pub struct LinkForCreate {
    pub id: Uuid,
    pub title: String,
    pub url: String,
}

// Link for update
#[derive(AsChangeset, Insertable)]
#[diesel(table_name = link)]
pub struct LinkForUpdate {
    pub title: Option<String>,
    pub url: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct LinkBmc;

impl LinkBmc {
    pub fn create(mm: &ModelManager, link: LinkForCreate) -> Result<Link> {
        let mut connection = mm.conn()?;
        diesel::insert_into(link::dsl::link)
            .values(&link)
            .get_result::<Link>(&mut connection)
            .map_err(|e| e.into())
    }
    pub fn get(mm: &ModelManager, id: Uuid) -> Result<Link> {
        let mut connection = mm.conn()?;
        link::dsl::link
            .filter(link::dsl::id.eq(&id))
            .first::<Link>(&mut connection)
            .map_err(|e| e.into())
    }
    // TODO: implement to get list of titles that contain string slice
    #[allow(dead_code)]
    pub fn get_by_title(mm: &ModelManager, title: String) -> Result<Link> {
        let mut connection = mm.conn()?;
        link::dsl::link
            .filter(link::dsl::title.eq(&title))
            .first::<Link>(&mut connection)
            .map_err(|e| e.into())
    }
    pub fn list(mm: &ModelManager) -> Result<Vec<Link>> {
        let mut connection = mm.conn()?;
        link::dsl::link
            .load::<Link>(&mut connection)
            .map_err(|e| e.into())
    }
    pub fn update(mm: &ModelManager, id: Uuid, link: LinkForUpdate) -> Result<Link> {
        let mut connection = mm.conn()?;
        diesel::update(link::dsl::link.filter(link::dsl::id.eq(&id)))
            .set(&link)
            .get_result::<Link>(&mut connection)
            .map_err(|e| e.into())
    }
    pub fn delete(mm: &ModelManager, id: Uuid) -> Result<()> {
        let mut connection = mm.conn()?;
        diesel::delete(link::dsl::link.filter(link::dsl::id.eq(&id)))
            .execute(&mut connection)
            .map_err(|e| -> Error { e.into() })?;
        Ok(())
    }
}
