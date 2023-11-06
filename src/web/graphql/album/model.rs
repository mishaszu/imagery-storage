use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::scalars::Id;

#[derive(Debug, Clone, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct Album {
    pub id: Id,
    pub user_id: Id,
    pub name: String,
    pub description: String,
    pub picture: Option<Id>,
    pub is_public: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[ComplexObject]
impl Album {
    // async fn picture(&self, ctx: &Context<'_>) -> Result<Option<Image>> {
    //     match self.picture_id {
    //         Some(id) => {
    //             let mm = ctx.data_opt::<ModelManager>();
    //             let mm = match mm {
    //                 Some(mm) => mm,
    //                 None => {
    //                     return Err(
    //                         crate::web::api::graphql::error::Error::ModalManagerNotInContext.into(),
    //                     )
    //                 }
    //             };

    //             let image = ImageBmc::get(mm, id.into())
    //                 .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
    //             Ok(Some(image.into()))
    //         }
    //         None => Ok(None),
    //     }
    // }

    // async fn images(&self, ctx: &Context<'_>) -> Result<Vec<Image>> {
    //     let mm = ctx.data_opt::<ModelManager>();
    //     let mm = match mm {
    //         Some(mm) => mm,
    //         None => {
    //             return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
    //         }
    //     };
    //     let images = AlbumBmc::get_images(mm, self.id.into())
    //         .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
    //     Ok(images.into_iter().map(|r| r.into()).collect())
    // }

    // async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<Tag>> {
    //     let mm = ctx.data_opt::<ModelManager>();
    //     let mm = match mm {
    //         Some(mm) => mm,
    //         None => {
    //             return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
    //         }
    //     };
    //     let tags = AlbumBmc::get_tags(mm, self.id.into())
    //         .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
    //     Ok(tags.into_iter().map(|t| t.into()).collect())
    // }
}

impl From<crate::model::album::Album> for Album {
    fn from(album: crate::model::album::Album) -> Self {
        Album {
            id: album.id.into(),
            user_id: album.user_id.into(),
            name: album.name,
            description: album.description,
            picture: album.picture.map(|id| id.into()),
            is_public: album.is_public,
            created_at: album.created_at.to_string(),
            updated_at: album.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct AlbumForCreate {
    pub name: String,
    pub description: Option<String>,
}

impl Into<crate::model::album::AlbumForCreate> for AlbumForCreate {
    fn into(self) -> crate::model::album::AlbumForCreate {
        crate::model::album::AlbumForCreate {
            id: Uuid::new_v4(),
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct AlbumForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub picture: Option<Id>,
    pub is_public: Option<bool>,
}

impl Into<crate::model::album::AlbumForUpdate> for AlbumForUpdate {
    fn into(self) -> crate::model::album::AlbumForUpdate {
        crate::model::album::AlbumForUpdate {
            name: self.name,
            description: self.description,
            picture: self.picture.map(|id| id.0),
            is_public: self.is_public,
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct AlbumImageForCreate {
    pub album_id: Id,
    pub image_id: Id,
}

impl Into<crate::model::album::AlbumImageForCreate> for AlbumImageForCreate {
    fn into(self) -> crate::model::album::AlbumImageForCreate {
        crate::model::album::AlbumImageForCreate {
            id: Uuid::new_v4(),
            album_id: self.album_id.into(),
            image_id: self.image_id.into(),
        }
    }
}
