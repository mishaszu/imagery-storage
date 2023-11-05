use std::collections::HashMap;

use diesel::prelude::*;
use diesel::{
    query_builder::AsChangeset, ExpressionMethods, Identifiable, Insertable, Queryable, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::{
    album, image, kitty, kitty_album, kitty_image, kitty_rating_score, kitty_tag, rating_score, tag,
};

use super::album::Album;
use super::image::Image;
use super::rating_score::RatingScore;
use super::{tag::Tag, ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Queryable, Identifiable)]
#[diesel(table_name = kitty)]
pub struct Kitty {
    pub id: Uuid,
    pub name: String,
    pub names: Option<String>,
    pub picture_id: Option<Uuid>,
    pub album_id: Option<Uuid>,
    pub description: Option<String>,
    pub age: Option<i32>,
    pub origin: Option<String>,
    pub is_favorite: bool,
    pub fc: i32,
    pub wsic: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = kitty)]
pub struct KittyForCreate {
    pub id: Uuid,
    pub name: String,
    pub names: Option<String>,
    pub description: Option<String>,
    pub age: Option<i32>,
    pub origin: Option<String>,
    pub is_favorite: Option<bool>,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = kitty)]
pub struct KittyForUpdate {
    pub name: Option<String>,
    pub names: Option<String>,
    pub picture_id: Option<Uuid>,
    pub album_id: Option<Uuid>,
    pub description: Option<String>,
    pub age: Option<i32>,
    pub origin: Option<String>,
    pub is_favorite: Option<bool>,
    pub fc: Option<i32>,
    pub wsic: Option<i32>,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Queryable, Identifiable)]
#[diesel(table_name = kitty_tag)]
pub struct KittyTag {
    pub id: Uuid,
    pub kitty_id: Uuid,
    pub tag_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = kitty_tag)]
pub struct KittyTagForCreate {
    pub id: Uuid,
    pub kitty_id: Uuid,
    pub tag_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Queryable, Identifiable)]
#[diesel(table_name = kitty_image)]
pub struct KittyImage {
    pub id: Uuid,
    pub kitty_id: Uuid,
    pub image_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = kitty_image)]
pub struct KittyImageForCreate {
    pub id: Uuid,
    pub image_id: Uuid,
    pub kitty_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Queryable, Identifiable)]
#[diesel(table_name = kitty_album)]
pub struct KittyAlbum {
    pub id: Uuid,
    pub kitty_id: Uuid,
    pub album_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = kitty_album)]
pub struct KittyAlbumForCreate {
    pub id: Uuid,
    pub kitty_id: Uuid,
    pub album_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Queryable, Identifiable)]
#[diesel(table_name = kitty_rating_score)]
pub struct KittyRatingScore {
    pub id: Uuid,
    pub kitty_id: Uuid,
    pub rating_score_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = kitty_rating_score)]
pub struct KittyRatingScoreForCreate {
    pub id: Uuid,
    pub kitty_id: Uuid,
    pub rating_score_id: Uuid,
}

pub struct KittyBmc;

impl KittyBmc {
    pub fn create(mm: &ModelManager, input: KittyForCreate) -> Result<Kitty> {
        let mut connection = mm.conn()?;

        diesel::insert_into(kitty::dsl::kitty)
            .values(input)
            .get_result::<Kitty>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get(mm: &ModelManager, id: Uuid) -> Result<Kitty> {
        let mut connection = mm.conn()?;

        kitty::dsl::kitty
            .filter(kitty::dsl::id.eq(id))
            .first::<Kitty>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get_by_name(mm: &ModelManager, name: String) -> Result<Vec<Kitty>> {
        let mut connection = mm.conn()?;

        let mut kitties: Vec<Kitty> = kitty::dsl::kitty
            .filter(kitty::dsl::name.ilike(format!("%{}%", name)))
            .load::<Kitty>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })?;

        let mut kitties2: Vec<Kitty> = kitty::dsl::kitty
            .filter(kitty::dsl::names.ilike(format!("%{}%", name)))
            .load::<Kitty>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })?;

        kitties.append(&mut kitties2);

        let all: HashMap<Uuid, Kitty> = HashMap::new();

        let all = kitties.into_iter().fold(all, |mut acc, kitty| {
            match acc.get(&kitty.id) {
                Some(_) => {}
                None => {
                    acc.insert(kitty.id, kitty);
                }
            }
            acc
        });

        let kitties: Vec<Kitty> = all.into_values().collect();

        Ok(kitties)
    }

    pub fn list(mm: &ModelManager) -> Result<Vec<Kitty>> {
        let mut connection = mm.conn()?;

        kitty::dsl::kitty
            .load::<Kitty>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn update(mm: &ModelManager, id: Uuid, input: KittyForUpdate) -> Result<Kitty> {
        let mut connection = mm.conn()?;

        diesel::update(kitty::dsl::kitty.filter(kitty::dsl::id.eq(id)))
            .set(input)
            .get_result::<Kitty>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn delete(mm: &ModelManager, id: Uuid) -> Result<Kitty> {
        let mut connection = mm.conn()?;

        diesel::delete(kitty::dsl::kitty.filter(kitty::dsl::id.eq(id)))
            .get_result::<Kitty>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }
    pub fn create_kitty_tag(mm: &ModelManager, input: KittyTagForCreate) -> Result<KittyTag> {
        let mut connection = mm.conn()?;

        diesel::insert_into(kitty_tag::dsl::kitty_tag)
            .values(input)
            .get_result::<KittyTag>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get_tags(mm: &ModelManager, id: Uuid) -> Result<Vec<Tag>> {
        let mut connection = mm.conn()?;

        kitty_tag::dsl::kitty_tag
            .filter(kitty_tag::dsl::kitty_id.eq(id))
            .inner_join(tag::dsl::tag)
            .select(tag::all_columns)
            .load::<Tag>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn create_kitty_image(mm: &ModelManager, input: KittyImageForCreate) -> Result<KittyImage> {
        let mut connection = mm.conn()?;

        diesel::insert_into(kitty_image::dsl::kitty_image)
            .values(input)
            .get_result::<KittyImage>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get_images(mm: &ModelManager, id: Uuid) -> Result<Vec<Image>> {
        let mut connection = mm.conn()?;

        kitty_image::dsl::kitty_image
            .filter(kitty_image::dsl::kitty_id.eq(id))
            .inner_join(image::dsl::image)
            .select(image::all_columns)
            .load::<Image>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn create_kitty_album(mm: &ModelManager, input: KittyAlbumForCreate) -> Result<KittyAlbum> {
        let mut connection = mm.conn()?;

        diesel::insert_into(kitty_album::dsl::kitty_album)
            .values(input)
            .get_result::<KittyAlbum>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get_albums(mm: &ModelManager, id: Uuid) -> Result<Vec<Album>> {
        let mut connection = mm.conn()?;

        kitty_album::dsl::kitty_album
            .filter(kitty_album::dsl::kitty_id.eq(id))
            .inner_join(album::dsl::album)
            .select(album::all_columns)
            .load::<Album>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn create_kitty_rating_score(
        mm: &ModelManager,
        input: KittyRatingScoreForCreate,
    ) -> Result<KittyRatingScore> {
        let mut connection = mm.conn()?;

        diesel::insert_into(kitty_rating_score::dsl::kitty_rating_score)
            .values(input)
            .get_result::<KittyRatingScore>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }

    pub fn get_scores(mm: &ModelManager, id: Uuid) -> Result<Vec<RatingScore>> {
        let mut connection = mm.conn()?;

        kitty_rating_score::dsl::kitty_rating_score
            .filter(kitty_rating_score::dsl::kitty_id.eq(id))
            .inner_join(rating_score::dsl::rating_score)
            .select(rating_score::all_columns)
            .load::<RatingScore>(&mut connection)
            .map_err(|e| -> crate::model::Error { e.into() })
    }
}
