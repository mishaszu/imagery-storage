use async_graphql::{ComplexObject, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{
    graphql::{
        scalars::{DateTime, Id},
        ImageKind,
    },
    model::image::ImageBmc,
};

#[derive(Debug, Clone, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct Image {
    pub id: Id,
    pub user_id: Id,
    pub name: Option<String>,
    pub kind: ImageKind,
    pub path: Id,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[ComplexObject]
impl Image {
    // async fn tags(
    //     &self,
    //     ctx: &async_graphql::Context<'_>,
    // ) -> async_graphql::Result<Vec<crate::web::api::graphql::tag::model::Tag>> {
    //     let mm = ctx.data_opt::<crate::model::ModelManager>();
    //     let mm = match mm {
    //         Some(mm) => mm,
    //         None => {
    //             return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
    //         }
    //     };
    //     let tags = ImageBmc::get_tags(mm, self.id.0)
    //         .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
    //     Ok(tags.into_iter().map(|r| r.into()).collect())
    // }

    // async fn albums(
    //     &self,
    //     ctx: &async_graphql::Context<'_>,
    // ) -> async_graphql::Result<Vec<crate::web::api::graphql::album::model::Album>> {
    //     let mm = ctx.data_opt::<crate::model::ModelManager>();
    //     let mm = match mm {
    //         Some(mm) => mm,
    //         None => {
    //             return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
    //         }
    //     };
    //     let albums = ImageBmc::get_albums(mm, self.id.0)
    //         .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
    //     Ok(albums.into_iter().map(|r| r.into()).collect())
    // }
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
pub struct ImageForCreate {
    pub path: String,
    pub user_id: Id,
    pub name: Option<String>,
    pub kind: ImageKind,
}

impl Into<crate::model::image::ImageForCreate> for ImageForCreate {
    fn into(self) -> crate::model::image::ImageForCreate {
        crate::model::image::ImageForCreate {
            id: uuid::Uuid::new_v4(),
            user_id: self.user_id.into(),
            name: self.name,
            kind: self.kind.to_string(),
            path: self.path.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct ImageForUpdate {
    pub name: Option<String>,
    pub path: Option<Id>,
}

impl Into<crate::model::image::ImageForUpdate> for ImageForUpdate {
    fn into(self) -> crate::model::image::ImageForUpdate {
        crate::model::image::ImageForUpdate {
            name: self.name,
            path: self.path.map(|id| id.0),
            updated_at: chrono::Utc::now(),
        }
    }
}
