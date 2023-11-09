use async_graphql::{ComplexObject, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::scalars::{DateTime, Id, PublicLvl};

#[derive(SimpleObject, Debug, Clone, Serialize)]
pub struct Post {
    pub id: Id,
    pub title: String,
    pub body: String,
    pub user_id: Id,
    pub disable_comments: bool,
    pub public_lvl: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<crate::model::post::Post> for Post {
    fn from(post: crate::model::post::Post) -> Self {
        Self {
            id: post.id.into(),
            title: post.title,
            body: post.body,
            user_id: post.user_id.into(),
            disable_comments: post.disable_comments,
            public_lvl: post.public_lvl,
            created_at: post.created_at.into(),
            updated_at: post.updated_at.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct PostForCreate {
    pub title: String,
    pub body: String,
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
            disable_comments: self.disable_comments,
            public_lvl: self.public_lvl.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct PostForUpdate {
    pub title: Option<String>,
    pub body: Option<String>,
    pub public_lvl: Option<i32>,
    pub disable_comments: Option<bool>,
}

impl Into<crate::model::post::PostForUpdate> for PostForUpdate {
    fn into(self) -> crate::model::post::PostForUpdate {
        crate::model::post::PostForUpdate {
            title: self.title,
            body: self.body,
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
