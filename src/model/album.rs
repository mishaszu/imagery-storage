use diesel::prelude::*;
use uuid::Uuid;

use super::{post::Post, ModelManager, Result};

use crate::schema::{album, album_post, post};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = album)]
pub struct Album {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub picture: Option<Uuid>,
    pub public_lvl: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = album)]
pub struct AlbumForCreate {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = album)]
pub struct AlbumForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub picture: Option<Uuid>,
    pub public_lvl: Option<i32>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = album_post)]
pub struct AlbumPost {
    pub id: Uuid,
    pub album_id: Uuid,
    pub post_id: Uuid,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = album_post)]
pub struct AlbumPostForCreate {
    pub id: Uuid,
    pub album_id: Uuid,
    pub post_id: Uuid,
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

    pub fn create_album_post(mm: &ModelManager, input: AlbumPostForCreate) -> Result<AlbumPost> {
        let mut connection = mm.conn()?;

        diesel::insert_into(album_post::dsl::album_post)
            .values(input)
            .get_result::<AlbumPost>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn delete_album_post(mm: &ModelManager, post_id: &Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;

        diesel::delete(album_post::dsl::album_post.filter(album_post::dsl::post_id.eq(post_id)))
            .execute(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_posts(mm: &ModelManager, id: Uuid) -> Result<Vec<Post>> {
        let mut connection = mm.conn()?;

        album_post::dsl::album_post
            .filter(album_post::dsl::album_id.eq(id))
            .inner_join(post::dsl::post)
            .select(post::all_columns)
            .load::<Post>(&mut connection)
            .map_err(|e| e.into())
    }
}
