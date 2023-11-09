use diesel::prelude::*;
use diesel::{
    query_builder::AsChangeset, ExpressionMethods, Identifiable, Insertable, Queryable, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::{post, post_image};

use super::{ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = post)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub user_id: Uuid,
    pub disable_comments: bool,
    pub public_lvl: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = post)]
pub struct PostForCreate {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub user_id: Uuid,
    pub disable_comments: bool,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = post)]
pub struct PostForUpdate {
    pub title: Option<String>,
    pub body: Option<String>,
    pub public_lvl: Option<i32>,
    pub disable_comments: Option<bool>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = post_image)]
pub struct PostImage {
    pub id: Uuid,
    pub post_id: Uuid,
    pub image_id: Uuid,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = post_image)]
pub struct PostImageForCreate {
    pub id: Uuid,
    pub post_id: Uuid,
    pub image_id: Uuid,
}

pub struct PostBmc;

impl PostBmc {
    pub fn create(
        mm: &ModelManager,
        post: &PostForCreate,
        images: &Vec<PostImageForCreate>,
    ) -> Result<Post> {
        let mut connection = mm.conn()?;

        connection.transaction(|conn| {
            let post = diesel::insert_into(post::table)
                .values(post)
                .get_result::<Post>(conn)?;

            let images = diesel::insert_into(post_image::table)
                .values(images)
                .get_results::<PostImage>(conn)?;

            Ok(post)
        })
    }

    pub fn add_post_image(mm: &ModelManager, post_id: &Uuid, image_id: &Uuid) -> Result<PostImage> {
        let mut connection = mm.conn()?;

        let post_image = diesel::insert_into(post_image::table)
            .values(PostImageForCreate {
                id: Uuid::new_v4(),
                post_id: post_id.clone(),
                image_id: image_id.clone(),
            })
            .get_result::<PostImage>(&mut connection)?;

        Ok(post_image)
    }

    pub fn update(mm: &ModelManager, post_id: Uuid, post: &PostForUpdate) -> Result<Post> {
        let mut connection = mm.conn()?;
        let post = diesel::update(post::table)
            .filter(post::id.eq(post_id))
            .set(post)
            .get_result::<Post>(&mut connection)?;

        Ok(post)
    }

    pub fn list(mm: &ModelManager) -> Result<Vec<Post>> {
        let mut connection = mm.conn()?;
        let posts = post::dsl::post.load::<Post>(&mut connection)?;

        Ok(posts)
    }

    pub fn delete_post_image(
        mm: &ModelManager,
        post_id: &Uuid,
        post_image_id: &Uuid,
    ) -> Result<usize> {
        let mut connection = mm.conn()?;
        let post_image = diesel::delete(post_image::table)
            .filter(post_image::post_id.eq(post_id))
            .filter(post_image::id.eq(post_image_id))
            .execute(&mut connection)?;

        Ok(post_image)
    }

    pub fn delete(mm: &ModelManager, post_id: Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;
        let post = diesel::delete(post::table)
            .filter(post::id.eq(post_id))
            .execute(&mut connection)?;

        Ok(post)
    }
}
