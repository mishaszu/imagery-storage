use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::graphql::{
    scalars::{DateTime, Id},
    ImageKind,
};

#[derive(Debug, Clone, Serialize, SimpleObject)]
pub struct Image {
    pub id: Id,
    pub user_id: Id,
    pub name: Option<String>,
    pub kind: ImageKind,
    pub path: Id,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<crate::model::image::Image> for Image {
    fn from(image: crate::model::image::Image) -> Self {
        Self {
            id: Id(image.id),
            user_id: Id(image.user_id),
            name: image.name,
            kind: image.kind.into(),
            path: Id(image.path),
            created_at: image.created_at.into(),
            updated_at: image.updated_at.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct ImageForUpdate {
    pub name: Option<String>,
}

impl Into<crate::model::image::ImageForUpdate> for ImageForUpdate {
    fn into(self) -> crate::model::image::ImageForUpdate {
        crate::model::image::ImageForUpdate {
            name: self.name,
            updated_at: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, SimpleObject)]
pub struct ImageDeleteResult {
    image_id: Id,
    success: bool,
    error: Option<String>,
}

impl ImageDeleteResult {
    pub fn delete_success(image_id: Id) -> Self {
        Self {
            image_id,
            success: true,
            error: None,
        }
    }

    pub fn delete_failure(image_id: Id, error: String) -> Self {
        Self {
            image_id,
            success: false,
            error: Some(error),
        }
    }
}
