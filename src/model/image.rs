use diesel::prelude::*;
use diesel::{
    query_builder::AsChangeset, ExpressionMethods, Identifiable, Insertable, Queryable, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::{
    album, album_image, image, image_rating_score, image_tag, kitty, kitty_image, rating_score,
};

use super::album::Album;
use super::kitty::Kitty;

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = image)]
pub struct Image {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub path: String,
    pub is_uploaded: bool,
    pub is_printable: bool,
    pub is_printed: bool,
    pub is_favorite: bool,
    pub is_hidden: bool,
    pub image_date: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = image_tag)]
pub struct ImageTag {
    pub id: Uuid,
    pub image_id: Uuid,
    pub tag_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = image_tag)]
pub struct ImageTagForCreate {
    pub id: Uuid,
    pub image_id: Uuid,
    pub tag_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = image_rating_score)]
pub struct ImageRatingScore {
    pub id: Uuid,
    pub image_id: Uuid,
    pub rating_score_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = image_rating_score)]
pub struct ImageRatingScoreForCreate {
    pub id: Uuid,
    pub image_id: Uuid,
    pub rating_score_id: Uuid,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = image)]
pub struct ImageForCreate {
    pub id: Uuid,
    pub path: String,
}

impl From<String> for ImageForCreate {
    fn from(path: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            path,
        }
    }
}

#[derive(Default, Debug, Clone, AsChangeset)]
#[diesel(table_name = image)]
pub struct ImageForUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub path: Option<String>,
    pub is_uploaded: Option<bool>,
    pub is_printable: Option<bool>,
    pub is_printed: Option<bool>,
    pub is_favorite: Option<bool>,
    pub is_hidden: Option<bool>,
    pub image_date: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
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

    pub fn create_tag_image(
        mm: &crate::model::ModelManager,
        image_tag: ImageTagForCreate,
    ) -> crate::model::Result<ImageTag> {
        let mut connection = mm.conn()?;

        diesel::insert_into(image_tag::dsl::image_tag)
            .values(image_tag)
            .get_result::<ImageTag>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn create_rating_score_image(
        mm: &crate::model::ModelManager,
        image_rating_score: ImageRatingScoreForCreate,
    ) -> crate::model::Result<ImageRatingScore> {
        let mut connection = mm.conn()?;

        diesel::insert_into(image_rating_score::dsl::image_rating_score)
            .values(image_rating_score)
            .get_result::<ImageRatingScore>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get(mm: &crate::model::ModelManager, id: Uuid) -> crate::model::Result<Image> {
        let mut connection = mm.conn()?;

        image::dsl::image
            .filter(image::dsl::id.eq(id))
            .first::<Image>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get_kitties(
        mm: &crate::model::ModelManager,
        id: Uuid,
    ) -> crate::model::Result<Vec<Kitty>> {
        let mut connection = mm.conn()?;

        kitty_image::dsl::kitty_image
            .filter(kitty_image::dsl::image_id.eq(id))
            .inner_join(kitty::dsl::kitty)
            .select(kitty::all_columns)
            .load::<Kitty>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get_scores(
        mm: &crate::model::ModelManager,
        id: Uuid,
    ) -> crate::model::Result<Vec<crate::model::rating_score::RatingScore>> {
        let mut connection = mm.conn()?;

        image_rating_score::dsl::image_rating_score
            .filter(image_rating_score::dsl::image_id.eq(id))
            .inner_join(rating_score::dsl::rating_score)
            .select(crate::schema::rating_score::all_columns)
            .load::<crate::model::rating_score::RatingScore>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get_tags(
        mm: &crate::model::ModelManager,
        id: Uuid,
    ) -> crate::model::Result<Vec<crate::model::tag::Tag>> {
        let mut connection = mm.conn()?;

        image_tag::dsl::image_tag
            .filter(image_tag::dsl::image_id.eq(id))
            .inner_join(crate::schema::tag::dsl::tag)
            .select(crate::schema::tag::all_columns)
            .load::<crate::model::tag::Tag>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn list(mm: &crate::model::ModelManager) -> crate::model::Result<Vec<Image>> {
        let mut connection = mm.conn()?;

        image::dsl::image
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

    pub fn get_albums(
        mm: &crate::model::ModelManager,
        id: Uuid,
    ) -> crate::model::Result<Vec<Album>> {
        let mut connection = mm.conn()?;

        album_image::dsl::album_image
            .filter(album_image::dsl::image_id.eq(id))
            .inner_join(album::dsl::album)
            .select(album::all_columns)
            .load::<Album>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }
}
