use diesel::prelude::*;
use diesel::prelude::{Identifiable, Queryable};
use uuid::Uuid;

use crate::schema::{
    album, album_rating_score, image, image_rating_score, kitty, kitty_rating_score, rating_score,
};

use super::album::Album;
use super::image::Image;
use super::kitty::Kitty;
use super::{Error, ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = rating_score)]
pub struct RatingScore {
    pub id: Uuid,
    pub rating_id: Uuid,
    pub score: i32,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = rating_score)]
pub struct RatingScoreForCreate {
    pub id: Uuid,
    pub rating_id: Uuid,
    pub score: i32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = rating_score)]
pub struct RatingScoreForUpdate {
    pub score: Option<i32>,
    pub rating_id: Option<Uuid>,
    pub updated_at: chrono::NaiveDateTime,
    pub description: Option<String>,
}

pub struct RatingScoreBmc;

impl RatingScoreBmc {
    pub fn create(mm: &ModelManager, rating_score: RatingScoreForCreate) -> Result<RatingScore> {
        let mut connection = mm.conn()?;

        diesel::insert_into(rating_score::dsl::rating_score)
            .values(rating_score)
            .get_result::<RatingScore>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }

    pub fn get(mm: &ModelManager, id: Uuid) -> Result<RatingScore> {
        let mut connection = mm.conn()?;

        rating_score::dsl::rating_score
            .filter(rating_score::dsl::id.eq(id))
            .first::<RatingScore>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }

    pub fn get_images(mm: &ModelManager, id: Uuid) -> Result<Vec<Image>> {
        let mut connection = mm.conn()?;

        image_rating_score::dsl::image_rating_score
            .filter(image_rating_score::dsl::rating_score_id.eq(id))
            .inner_join(image::dsl::image)
            .select(image::all_columns)
            .load::<Image>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }

    pub fn get_by_rating_id(mm: &ModelManager, rating_id: Uuid) -> Result<Vec<RatingScore>> {
        let mut connection = mm.conn()?;

        rating_score::dsl::rating_score
            .filter(rating_score::dsl::rating_id.eq(rating_id))
            .load::<RatingScore>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }

    pub fn list(mm: &ModelManager) -> Result<Vec<RatingScore>> {
        let mut connection = mm.conn()?;

        rating_score::dsl::rating_score
            .load::<RatingScore>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }

    pub fn update(
        mm: &ModelManager,
        id: Uuid,
        rating_score: RatingScoreForUpdate,
    ) -> Result<RatingScore> {
        let mut connection = mm.conn()?;

        diesel::update(rating_score::dsl::rating_score.filter(rating_score::dsl::id.eq(id)))
            .set(rating_score)
            .get_result::<RatingScore>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }

    pub fn delete(mm: &ModelManager, id: Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;

        diesel::delete(rating_score::dsl::rating_score.filter(rating_score::dsl::id.eq(id)))
            .execute(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }

    pub fn get_kitties(mm: &ModelManager, id: Uuid) -> Result<Vec<Kitty>> {
        let mut connection = mm.conn()?;

        kitty_rating_score::dsl::kitty_rating_score
            .filter(kitty_rating_score::dsl::rating_score_id.eq(id))
            .inner_join(kitty::dsl::kitty)
            .select(kitty::all_columns)
            .load::<Kitty>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }

    pub fn get_albums(mm: &ModelManager, id: Uuid) -> Result<Vec<Album>> {
        let mut connection = mm.conn()?;

        album_rating_score::dsl::album_rating_score
            .filter(album_rating_score::dsl::rating_score_id.eq(id))
            .inner_join(album::dsl::album)
            .select(album::all_columns)
            .load::<Album>(&mut connection)
            .map_err(|e| -> Error { e.into() })
    }
}
