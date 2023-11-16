use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::access::Accesship;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{
    graphql::scalars::{DateTime, Id, PublicLvl},
    model::{image::ImageBmc, ModelManager},
    web::graphql::image::model::Image,
};

/// ### Blog post
/// Post is a main content of blog.\
/// Images could be highlighted in ui, but post suits as main skeleton of blog feed and user feed
#[derive(SimpleObject, Debug, Clone, Serialize)]
#[graphql(complex)]
pub struct Post {
    pub id: Id,
    pub title: String,
    pub body: String,
    pub user_id: Id,
    pub add_to_feed: bool,
    pub disable_comments: bool,
    pub public_lvl: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[graphql(skip)]
    pub access: Option<Accesship>,
}

#[ComplexObject]
impl Post {
    /// ### All post images.
    /// Images assigned to post have the same public level as post.
    pub async fn images(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Image>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let images = ImageBmc::list_post(mm, &self.id.into()).map_err(GraphQLError::ModelError)?;

        Ok(images.into_iter().map(|r| r.into()).collect())
    }
}

impl Post {
    pub fn from_db_with_access(post: crate::model::post::Post, access: Accesship) -> Self {
        Self {
            id: post.id.into(),
            title: post.title,
            body: post.body,
            user_id: post.user_id.into(),
            add_to_feed: post.add_to_feed,
            disable_comments: post.disable_comments,
            public_lvl: post.public_lvl,
            created_at: post.created_at.into(),
            updated_at: post.updated_at.into(),
            access: Some(access),
        }
    }
}

impl TryFrom<(Accesship, Option<crate::model::post::Post>)> for Post {
    type Error = GraphQLError;
    fn try_from(value: (Accesship, Option<crate::model::post::Post>)) -> Result<Self, Self::Error> {
        let (access, post) = value;
        let post = match post {
            Some(post) => post,
            None => return Err(GraphQLError::AuthError.into()),
        };

        Ok(Self {
            id: post.id.into(),
            title: post.title,
            body: post.body,
            user_id: post.user_id.into(),
            add_to_feed: post.add_to_feed,
            disable_comments: post.disable_comments,
            public_lvl: post.public_lvl,
            created_at: post.created_at.into(),
            updated_at: post.updated_at.into(),
            access: Some(access),
        })
    }
}

impl From<crate::model::post::Post> for Post {
    fn from(post: crate::model::post::Post) -> Self {
        Self {
            id: post.id.into(),
            title: post.title,
            body: post.body,
            user_id: post.user_id.into(),
            add_to_feed: post.add_to_feed,
            disable_comments: post.disable_comments,
            public_lvl: post.public_lvl,
            created_at: post.created_at.into(),
            updated_at: post.updated_at.into(),
            access: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct PostForCreate {
    pub title: String,
    pub body: String,
    pub add_to_feed: Option<bool>,
    pub disable_comments: bool,
    pub public_lvl: PublicLvl,
}

impl PostForCreate {
    pub fn into_db(self, user_id: Uuid) -> crate::model::post::PostForCreate {
        crate::model::post::PostForCreate {
            id: uuid::Uuid::new_v4(),
            title: self.title,
            body: self.body,
            user_id,
            add_to_feed: self.add_to_feed,
            disable_comments: self.disable_comments,
            public_lvl: self.public_lvl.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct PostForUpdate {
    pub title: Option<String>,
    pub body: Option<String>,
    pub add_to_feed: Option<bool>,
    pub public_lvl: Option<i32>,
    pub disable_comments: Option<bool>,
}

impl Into<crate::model::post::PostForUpdate> for PostForUpdate {
    fn into(self) -> crate::model::post::PostForUpdate {
        crate::model::post::PostForUpdate {
            title: self.title,
            body: self.body,
            add_to_feed: self.add_to_feed,
            public_lvl: self.public_lvl,
            disable_comments: self.disable_comments,
            updated_at: chrono::Utc::now(),
        }
    }
}

#[derive(SimpleObject, Debug, Clone, Serialize)]
pub struct PostImage {
    pub id: Id,
    pub post_id: Id,
    pub image_id: Id,
}

impl From<crate::model::post::PostImage> for PostImage {
    fn from(post_image: crate::model::post::PostImage) -> Self {
        Self {
            id: post_image.id.into(),
            post_id: post_image.post_id.into(),
            image_id: post_image.image_id.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct PostImageForCreate {
    pub post_id: Id,
    pub image_id: Id,
}

impl Into<crate::model::post::PostImageForCreate> for PostImageForCreate {
    fn into(self) -> crate::model::post::PostImageForCreate {
        crate::model::post::PostImageForCreate {
            id: uuid::Uuid::new_v4(),
            post_id: self.post_id.into(),
            image_id: self.image_id.into(),
        }
    }
}
