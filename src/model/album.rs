use diesel::prelude::*;

use uuid::Uuid;

use super::{
    image::{Image, ImageForCreate},
    kitty::Kitty,
    rating_score::RatingScore,
    tag::Tag,
    ModelManager, Result,
};

use crate::schema::{
    album, album_image, album_rating_score, album_tag, image, kitty, kitty_album, rating_score, tag,
};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = album)]
pub struct Album {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub picture_id: Option<Uuid>,
    pub is_favorite: bool,
    pub is_hidden: bool,
    pub is_print_album: bool,
    pub is_printed: bool,
    pub image_per_page: Option<i32>,
    pub album_date: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = album)]
pub struct AlbumForCreate {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = album)]
pub struct AlbumForUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub picture_id: Option<Uuid>,
    pub is_favorite: Option<bool>,
    pub is_hidden: Option<bool>,
    pub is_print_album: Option<bool>,
    pub is_printed: Option<bool>,
    pub image_per_page: Option<i32>,
    pub album_date: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = album_image)]
pub struct AlbumImage {
    pub id: Uuid,
    pub album_id: Uuid,
    pub image_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = album_image)]
pub struct AlbumImageForCreate {
    pub id: Uuid,
    pub album_id: Uuid,
    pub image_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = album_tag)]
pub struct AlbumTag {
    pub id: Uuid,
    pub album_id: Uuid,
    pub tag_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = album_tag)]
pub struct AlbumTagForCreate {
    pub id: Uuid,
    pub album_id: Uuid,
    pub tag_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = album_rating_score)]
pub struct AlbumRatingScore {
    pub id: Uuid,
    pub album_id: Uuid,
    pub rating_score_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = album_rating_score)]
pub struct AlbumRatingScoreForCreate {
    pub id: Uuid,
    pub album_id: Uuid,
    pub rating_score_id: Uuid,
}

pub struct AlbumBmc;

impl AlbumBmc {
    pub fn create(mm: &ModelManager, album: AlbumForCreate) -> Result<Album> {
        let mut connection = mm.conn()?;

        diesel::insert_into(album::dsl::album)
            .values(album)
            .get_result::<Album>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get(mm: &ModelManager, id: Uuid) -> Result<Album> {
        let mut connection = mm.conn()?;

        album::dsl::album
            .filter(album::dsl::id.eq(id))
            .first::<Album>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn list(mm: &ModelManager) -> Result<Vec<Album>> {
        let mut connection = mm.conn()?;

        album::dsl::album
            .load::<Album>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn update(mm: &ModelManager, id: Uuid, album: AlbumForUpdate) -> Result<Album> {
        let mut connection = mm.conn()?;

        diesel::update(album::dsl::album.filter(album::dsl::id.eq(id)))
            .set(album)
            .get_result::<Album>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn delete(mm: &ModelManager, id: Uuid) -> Result<Album> {
        let mut connection = mm.conn()?;

        diesel::delete(album::dsl::album.filter(album::dsl::id.eq(id)))
            .get_result::<Album>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn create_album_image(mm: &ModelManager, input: AlbumImageForCreate) -> Result<AlbumImage> {
        let mut connection = mm.conn()?;

        diesel::insert_into(album_image::dsl::album_image)
            .values(input)
            .get_result::<AlbumImage>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_images(mm: &ModelManager, id: Uuid) -> Result<Vec<Image>> {
        let mut connection = mm.conn()?;

        album_image::dsl::album_image
            .filter(album_image::dsl::album_id.eq(id))
            .inner_join(image::dsl::image)
            .select(image::all_columns)
            .load::<Image>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn create_album_tag(mm: &ModelManager, input: AlbumTagForCreate) -> Result<AlbumTag> {
        let mut connection = mm.conn()?;

        diesel::insert_into(album_tag::dsl::album_tag)
            .values(input)
            .get_result::<AlbumTag>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_tags(mm: &ModelManager, id: Uuid) -> Result<Vec<Tag>> {
        let mut connection = mm.conn()?;

        album_tag::dsl::album_tag
            .filter(album_tag::dsl::album_id.eq(id))
            .inner_join(tag::dsl::tag)
            .select(tag::all_columns)
            .load::<Tag>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn create_album_rating_score(
        mm: &ModelManager,
        input: AlbumRatingScoreForCreate,
    ) -> Result<AlbumRatingScore> {
        let mut connection = mm.conn()?;

        diesel::insert_into(album_rating_score::dsl::album_rating_score)
            .values(input)
            .get_result::<AlbumRatingScore>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_scores(mm: &ModelManager, id: Uuid) -> Result<Vec<RatingScore>> {
        let mut connection = mm.conn()?;

        album_rating_score::dsl::album_rating_score
            .filter(album_rating_score::dsl::album_id.eq(id))
            .inner_join(rating_score::dsl::rating_score)
            .select(rating_score::all_columns)
            .load::<RatingScore>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_kitties(mm: &ModelManager, id: Uuid) -> Result<Vec<Kitty>> {
        let mut connection = mm.conn()?;

        kitty_album::dsl::kitty_album
            .filter(kitty_album::dsl::album_id.eq(id))
            .inner_join(kitty::dsl::kitty)
            .select(kitty::all_columns)
            .load::<Kitty>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn create_with_images(
        mm: &ModelManager,
        album: AlbumForCreate,
        images: Vec<ImageForCreate>,
    ) -> Result<Album> {
        let mut connection = mm.conn()?;

        let album_images: Vec<AlbumImageForCreate> = images
            .iter()
            .map(|i| AlbumImageForCreate {
                id: Uuid::new_v4(),
                album_id: album.id,
                image_id: i.id,
            })
            .collect();

        connection
            .transaction(|connection| {
                let album = diesel::insert_into(album::dsl::album)
                    .values(album)
                    .get_result::<Album>(connection)?;

                diesel::insert_into(image::dsl::image)
                    .values(images)
                    .execute(connection)?;

                diesel::insert_into(album_image::dsl::album_image)
                    .values(album_images)
                    .execute(connection)?;

                diesel::QueryResult::Ok(album)
            })
            .map_err(|e| e.into())
    }

    pub fn delete_with_images(mm: &ModelManager, id: Uuid) -> Result<()> {
        let mut connection = mm.conn()?;

        connection
            .transaction(|connection| {
                let images: Vec<Uuid> = album_image::dsl::album_image
                    .filter(album_image::dsl::album_id.eq(id))
                    .select(album_image::dsl::image_id)
                    .load::<Uuid>(connection)?;

                diesel::delete(album::dsl::album.filter(album::dsl::id.eq(id)))
                    .get_result::<Album>(connection)?;

                for image in images {
                    diesel::delete(image::dsl::image.filter(image::dsl::id.eq(image)))
                        .execute(connection)?;
                }

                diesel::QueryResult::Ok(())
            })
            .map_err(|e| e.into())
    }
}
