use async_graphql::{ComplexObject, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{graphql::scalars::Id, model::image::ImageBmc};

#[derive(Debug, Clone, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct Image {
    pub id: Id,
    pub user_id: Id,
    pub name: Option<String>,
    pub description: Option<String>,
    pub path: Id,
    pub is_public: bool,
    pub created_at: String,
    pub updated_at: String,
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
            description: image.description,
            path: Id(image.path),
            is_public: image.is_public,
            created_at: image.created_at.to_string(),
            updated_at: image.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct ImageForCreate {
    pub path: String,
    pub user_id: Id,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Into<crate::model::image::ImageForCreate> for ImageForCreate {
    fn into(self) -> crate::model::image::ImageForCreate {
        crate::model::image::ImageForCreate {
            id: uuid::Uuid::new_v4(),
            user_id: self.user_id.into(),
            name: self.name,
            description: self.description,
            path: self.path.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct ImageForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub path: Option<Id>,
    pub updated_at: String,
}

impl Into<crate::model::image::ImageForUpdate> for ImageForUpdate {
    fn into(self) -> crate::model::image::ImageForUpdate {
        crate::model::image::ImageForUpdate {
            name: self.name,
            description: self.description,
            path: self.path.map(|id| id.0),
            updated_at: self.updated_at.parse().unwrap(),
        }
    }
}
