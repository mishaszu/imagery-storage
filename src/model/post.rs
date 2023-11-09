use diesel::prelude::*;
use diesel::{
    query_builder::AsChangeset, ExpressionMethods, Identifiable, Insertable, Queryable, RunQueryDsl,
};
use uuid::Uuid;

use crate::graphql::guard::Accessship;
use crate::schema::{account, post, post_image, users};

use super::account::{Account, AccountBmc};
use super::{Error, ModelManager, Result};

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
    pub public_lvl: i32,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = post)]
pub struct PostForUpdate {
    pub title: Option<String>,
    pub body: Option<String>,
    pub disable_comments: Option<bool>,
    pub public_lvl: Option<i32>,
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
    pub fn get(mm: &ModelManager, post_id: &Uuid) -> Result<Post> {
        let mut connection = mm.conn()?;
        let post = post::dsl::post
            .filter(post::id.eq(post_id))
            .first::<Post>(&mut connection)?;

        Ok(post)
    }

    pub fn get_with_account(mm: &ModelManager, post_id: &Uuid) -> Result<(Post, Account)> {
        let mut connection = mm.conn()?;
        let post_n_account = post::dsl::post
            .filter(post::id.eq(post_id))
            .inner_join(users::dsl::users)
            .inner_join(account::dsl::account.on(users::account_id.eq(account::id)))
            .select((post::all_columns, account::all_columns))
            .first::<(Post, Account)>(&mut connection)?;

        Ok(post_n_account)
    }

    pub fn create(mm: &ModelManager, post: PostForCreate, images: Vec<Uuid>) -> Result<Post> {
        let mut connection = mm.conn()?;

        connection.transaction(|conn| {
            let post = diesel::insert_into(post::table)
                .values(post)
                .get_result::<Post>(conn)?;

            let images = images
                .into_iter()
                .map(|image_id| PostImageForCreate {
                    id: Uuid::new_v4(),
                    post_id: post.id,
                    image_id,
                })
                .collect::<Vec<PostImageForCreate>>();

            diesel::insert_into(post_image::table)
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

    pub fn update(mm: &ModelManager, post_id: &Uuid, post: PostForUpdate) -> Result<Post> {
        let mut connection = mm.conn()?;
        let post = diesel::update(post::table)
            .filter(post::id.eq(post_id))
            .set(post)
            .get_result::<Post>(&mut connection)?;

        Ok(post)
    }

    pub fn list_admin(mm: &ModelManager) -> Result<Vec<Post>> {
        let mut connection = mm.conn()?;
        let posts = post::dsl::post
            .order(post::created_at.desc())
            .load::<Post>(&mut connection)?;

        Ok(posts)
    }

    pub fn list(
        mm: &ModelManager,
        user_account_id: Option<Uuid>,
        target_user_id: &Uuid,
    ) -> Result<Vec<Post>> {
        let mut connection = mm.conn()?;

        let has_access = AccountBmc::has_access(mm, user_account_id, &target_user_id)?;

        let access_lvl = match has_access {
            Accessship::AllowedPublic => 2,
            Accessship::AllowedSubscriber => 1,
            Accessship::Admin | Accessship::Owner => 0,
            Accessship::None => return Err(Error::AccessDenied),
        };

        let posts = post::dsl::post
            .filter(post::user_id.eq(target_user_id))
            .filter(post::public_lvl.ge(access_lvl))
            .order(post::created_at.desc())
            .load::<Post>(&mut connection)?;

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

    pub fn delete(mm: &ModelManager, post_id: &Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;
        let post = diesel::delete(post::table)
            .filter(post::id.eq(post_id))
            .execute(&mut connection)?;

        Ok(post)
    }
}
