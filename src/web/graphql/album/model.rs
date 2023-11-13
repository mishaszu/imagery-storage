use async_graphql::{ComplexObject, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::scalars::{DateTime, Id};

#[derive(Debug, Clone, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct Album {
    pub id: Id,
    pub user_id: Id,
    pub name: String,
    pub description: String,
    pub is_wall: bool,
    pub picture: Option<Id>,
    pub public_lvl: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
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
            is_wall: album.is_wall,
            picture: album.picture.map(|id| id.into()),
            public_lvl: album.public_lvl,
            created_at: album.created_at.into(),
            updated_at: album.updated_at.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct AlbumForCreate {
    pub user_id: Id,
    pub name: String,
    pub description: Option<String>,
    pub is_wall: bool,
}

impl Into<crate::model::album::AlbumForCreate> for AlbumForCreate {
    fn into(self) -> crate::model::album::AlbumForCreate {
        crate::model::album::AlbumForCreate {
            id: Uuid::new_v4(),
            user_id: self.user_id.into(),
            name: self.name,
            description: self.description,
            is_wall: self.is_wall,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct AlbumForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub picture: Option<Id>,
    pub public_lvl: Option<i32>,
}

impl Into<crate::model::album::AlbumForUpdate> for AlbumForUpdate {
    fn into(self) -> crate::model::album::AlbumForUpdate {
        crate::model::album::AlbumForUpdate {
            name: self.name,
            description: self.description,
            picture: self.picture.map(|id| id.0),
            public_lvl: self.public_lvl,
            updated_at: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct AlbumPostForCreate {
    pub album_id: Id,
    pub post_id: Id,
}

impl Into<crate::model::album::AlbumPostForCreate> for AlbumPostForCreate {
    fn into(self) -> crate::model::album::AlbumPostForCreate {
        crate::model::album::AlbumPostForCreate {
            id: Uuid::new_v4(),
            album_id: self.album_id.into(),
            post_id: self.post_id.into(),
        }
    }
}
