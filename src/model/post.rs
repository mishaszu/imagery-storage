use diesel::prelude::*;
use diesel::{
    query_builder::AsChangeset, ExpressionMethods, Identifiable, Insertable, Queryable, RunQueryDsl,
};
use uuid::Uuid;

use crate::access::{Accesship, ResourceAccess};
use crate::schema::{account, post, post_image, users};

use super::account::{Account, AccountBmc};
use super::{ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = post)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub user_id: Uuid,
    pub add_to_feed: bool,
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
    pub add_to_feed: Option<bool>,
    pub disable_comments: bool,
    pub public_lvl: i32,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = post)]
pub struct PostForUpdate {
    pub title: Option<String>,
    pub body: Option<String>,
    pub add_to_feed: Option<bool>,
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
    pub fn get(mm: &ModelManager, post_id: &Uuid) -> Result<(Post, Account)> {
        let mut connection = mm.conn()?;
        let post = post::dsl::post
            .filter(post::id.eq(post_id))
            .inner_join(users::dsl::users)
            .inner_join(account::dsl::account.on(users::dsl::account_id.eq(account::dsl::id)))
            .select((post::all_columns, account::all_columns))
            .first::<(Post, Account)>(&mut connection)?;

        Ok(post)
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

    pub fn list(mm: &ModelManager, user_id: &Uuid) -> Result<Vec<Post>> {
        let mut connection = mm.conn()?;

        let posts = post::dsl::post
            .filter(post::user_id.eq(user_id))
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

#[derive(Debug, Clone, PartialEq)]
pub enum PostAccess {
    Admin,
    ToUser,
    ToAlbum,
}

impl ResourceAccess for PostBmc {
    type Resource = Post;
    type Filter = PostAccess;
    type ExtraSearch = Uuid;

    fn has_access(
        mm: &crate::model::ModelManager,
        // post id
        target_resource_id: &Uuid,
        seeker_user_id: Option<Uuid>,
        filter: Self::Filter,
    ) -> crate::model::Result<(crate::access::Accesship, Option<Self::Resource>)> {
        let seeker_account = seeker_user_id.and_then(|id| AccountBmc::get_by_user_id(mm, &id).ok());
        let (target_post, target_account) = Self::get(mm, target_resource_id)?;

        let access = target_account.compare_access(mm, seeker_account);

        let post = Self::get_with_access(mm, target_resource_id, access, filter)?;

        Ok((access, post))
    }

    fn get_with_access(
        mm: &crate::model::ModelManager,
        target_resource_id: &Uuid,
        access: Accesship,
        _filter: Self::Filter,
    ) -> crate::model::Result<Option<Self::Resource>> {
        let (target_post, _) = Self::get(mm, target_resource_id)?;

        match (access, target_post.public_lvl) {
            (Accesship::Admin, _) => Ok(Some(target_post)),
            (Accesship::Owner, _) => Ok(Some(target_post)),
            (Accesship::AllowedPublic, 2) => Ok(Some(target_post)),
            (Accesship::AllowedPublic, 1) => Ok(None),
            (Accesship::AllowedSubscriber, lvl) if lvl > 0 => Ok(Some(target_post)),
            _ => Err(crate::model::Error::AccessDeniedReturnNoInfo),
        }
    }

    fn has_access_list(
        mm: &crate::model::ModelManager,
        seeker_user_id: Option<Uuid>,
        extra_search_params: Self::ExtraSearch,
        filter: Self::Filter,
    ) -> crate::model::Result<Vec<(crate::access::Accesship, Option<Self::Resource>)>> {
        let seeker_account = seeker_user_id.and_then(|id| AccountBmc::get_by_user_id(mm, &id).ok());
        let target_account = AccountBmc::get_by_user_id(mm, &extra_search_params)?;

        let access = target_account.compare_access(mm, seeker_account);
        let access_lvl: i32 = access.try_into()?;

        let posts = Self::list(mm, &extra_search_params)?;

        let filtered_posts = Self::list_with_access(mm, access, extra_search_params, filter)?;

        let filtered_posts = filtered_posts
            .into_iter()
            .map(|post| (access, post))
            .collect();

        Ok(filtered_posts)
    }

    fn list_with_access(
        mm: &crate::model::ModelManager,
        access: Accesship,
        extra_search_params: Self::ExtraSearch,
        filter: Self::Filter,
    ) -> crate::model::Result<Vec<Option<Self::Resource>>> {
        let access_lvl: i32 = access.try_into()?;

        let posts = Self::list(mm, &extra_search_params)?;

        let filtered_posts = posts
            .into_iter()
            .filter(|post| {
                // filter out all private post if user is subscriber or no user
                if access_lvl != 0 && post.public_lvl == 0 {
                    false
                } else {
                    true
                }
            })
            .filter(|post| {
                if post.add_to_feed && filter == PostAccess::ToUser {
                    true
                // Album filter should check if post is added to album
                } else if !post.add_to_feed && filter == PostAccess::ToAlbum {
                    true
                } else if filter == PostAccess::Admin && access_lvl == 0 {
                    true
                } else {
                    false
                }
            })
            .map(|post| {
                // for non sub and no user display only public posts and rest should be null
                if post.public_lvl <= access_lvl {
                    Some(post)
                } else {
                    None
                }
            })
            .collect::<Vec<Option<Post>>>();

        Ok(filtered_posts)
    }
}
