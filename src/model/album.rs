use diesel::prelude::*;
use uuid::Uuid;

use super::{
    image::{Image, ImageForCreate},
    ModelManager, Result,
};

use crate::schema::{album, album_image, image};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = album)]
pub struct Album {
    id: Uuid,
    user_id: Uuid,
    name: String,
    description: String,
    picture: Option<Uuid>,
    is_public: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = album)]
pub struct AlbumForCreate {
    id: Uuid,
    name: String,
    description: Option<String>,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = album)]
pub struct AlbumForUpdate {
    name: Option<String>,
    description: Option<String>,
    picture: Option<Uuid>,
    is_public: Option<bool>,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = album_image)]
pub struct AlbumImage {
    id: Uuid,
    album_id: Uuid,
    image_id: Uuid,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = album_image)]
pub struct AlbumImageForCreate {
    id: Uuid,
    album_id: Uuid,
    image_id: Uuid,
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

    pub fn delete(mm: &ModelManager, id: Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;

        diesel::delete(album::dsl::album.filter(album::dsl::id.eq(id)))
            .execute(&mut connection)
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
