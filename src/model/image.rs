use diesel::prelude::*;
use diesel::{
    query_builder::AsChangeset, ExpressionMethods, Identifiable, Insertable, Queryable, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::image;

use super::Result;

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = image)]
pub struct Image {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Option<String>,
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
    pub path: Uuid,
}

#[derive(Default, Debug, Clone, AsChangeset)]
#[diesel(table_name = image)]
pub struct ImageForUpdate {
    pub name: Option<String>,
    pub path: Option<Uuid>,
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

    pub fn get(mm: &crate::model::ModelManager, search_id: Uuid) -> Result<Image> {
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

    pub fn delete_many(mm: &crate::model::ModelManager, ids: Vec<Uuid>) -> Result<()> {
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
