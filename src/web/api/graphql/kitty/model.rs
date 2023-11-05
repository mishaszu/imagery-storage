use async_graphql::{ComplexObject, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{
    graphql::scalars::Id,
    model::{album::AlbumBmc, image::ImageBmc, kitty::KittyBmc},
    web::api::graphql::{
        album::model::Album, image::model::Image, rating_score::model::RatingScore, tag::model::Tag,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Kitty {
    pub id: Id,
    pub name: String,
    pub names: Option<Vec<String>>,
    pub picture_id: Option<Id>,
    pub album_id: Option<Id>,
    pub description: Option<String>,
    pub age: Option<i32>,
    pub origin: Option<String>,
    pub is_favorite: bool,
    pub fc: i32,
    pub wsic: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[ComplexObject]
impl Kitty {
    async fn picture(&self, ctx: &async_graphql::Context<'_>) -> Result<Option<Image>> {
        match self.picture_id {
            Some(id) => {
                let mm = ctx.data_opt::<crate::model::ModelManager>();
                let mm = match mm {
                    Some(mm) => mm,
                    None => {
                        return Err(
                            crate::web::api::graphql::error::Error::ModalManagerNotInContext.into(),
                        )
                    }
                };

                let image = ImageBmc::get(mm, id.into())
                    .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
                Ok(Some(image.into()))
            }
            None => Ok(None),
        }
    }

    async fn album(&self, ctx: &async_graphql::Context<'_>) -> Result<Option<Album>> {
        match self.album_id {
            Some(id) => {
                let mm = ctx.data_opt::<crate::model::ModelManager>();
                let mm = match mm {
                    Some(mm) => mm,
                    None => {
                        return Err(
                            crate::web::api::graphql::error::Error::ModalManagerNotInContext.into(),
                        )
                    }
                };

                let album = AlbumBmc::get(mm, id.into())
                    .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
                Ok(Some(album.into()))
            }
            None => Ok(None),
        }
    }

    async fn albums(&self, ctx: &async_graphql::Context<'_>) -> Result<Vec<Album>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let albums = KittyBmc::get_albums(mm, self.id.into())
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(albums.into_iter().map(|a| a.into()).collect())
    }

    async fn tags(&self, ctx: &async_graphql::Context<'_>) -> Result<Vec<Tag>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let tags = KittyBmc::get_tags(mm, self.id.into())
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(tags.into_iter().map(|t| t.into()).collect())
    }

    async fn images(&self, ctx: &async_graphql::Context<'_>) -> Result<Vec<Image>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let images = KittyBmc::get_images(mm, self.id.into())
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(images.into_iter().map(|i| i.into()).collect())
    }

    async fn scores(&self, ctx: &async_graphql::Context<'_>) -> Result<Vec<RatingScore>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let scores = KittyBmc::get_scores(mm, self.id.into())
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(scores.into_iter().map(|s| s.into()).collect())
    }
}

impl From<crate::model::kitty::Kitty> for Kitty {
    fn from(kitty: crate::model::kitty::Kitty) -> Self {
        Self {
            id: kitty.id.into(),
            name: kitty.name,
            names: kitty.names.map(|names| {
                names
                    .split(",")
                    .into_iter()
                    .map(|name| name.to_string())
                    .collect::<Vec<String>>()
            }),
            picture_id: kitty.picture_id.map(|id| id.into()),
            album_id: kitty.album_id.map(|id| id.into()),
            description: kitty.description,
            age: kitty.age,
            origin: kitty.origin,
            is_favorite: kitty.is_favorite,
            fc: kitty.fc,
            wsic: kitty.wsic,
            created_at: kitty.created_at.to_string(),
            updated_at: kitty.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct KittyForCreate {
    pub name: String,
    pub names: Option<Vec<String>>,
    pub description: Option<String>,
    pub age: Option<i32>,
    pub origin: Option<String>,
    pub is_favorite: Option<bool>,
}

impl Into<crate::model::kitty::KittyForCreate> for KittyForCreate {
    fn into(self) -> crate::model::kitty::KittyForCreate {
        crate::model::kitty::KittyForCreate {
            id: uuid::Uuid::new_v4(),
            name: self.name,
            names: self.names.map(|names| names.join(",")),
            description: self.description,
            age: self.age,
            origin: self.origin,
            is_favorite: self.is_favorite,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct KittyForUpdate {
    pub name: Option<String>,
    pub names: Option<Vec<String>>,
    pub picture_id: Option<Id>,
    pub album_id: Option<Id>,
    pub description: Option<String>,
    pub age: Option<i32>,
    pub origin: Option<String>,
    pub is_favorite: Option<bool>,
    pub fc: Option<i32>,
    pub wsic: Option<i32>,
}

impl Into<crate::model::kitty::KittyForUpdate> for KittyForUpdate {
    fn into(self) -> crate::model::kitty::KittyForUpdate {
        crate::model::kitty::KittyForUpdate {
            name: self.name,
            names: self.names.map(|names| names.join(",")),
            picture_id: self.picture_id.map(|id| id.into()),
            album_id: self.album_id.map(|id| id.into()),
            description: self.description,
            age: self.age,
            origin: self.origin,
            is_favorite: self.is_favorite,
            fc: self.fc,
            wsic: self.wsic,
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct KittyTagForCreate {
    pub kitty_id: Id,
    pub tag_id: Id,
}

impl Into<crate::model::kitty::KittyTagForCreate> for KittyTagForCreate {
    fn into(self) -> crate::model::kitty::KittyTagForCreate {
        crate::model::kitty::KittyTagForCreate {
            id: uuid::Uuid::new_v4(),
            kitty_id: self.kitty_id.into(),
            tag_id: self.tag_id.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct KittyAlbumForCreate {
    pub kitty_id: Id,
    pub album_id: Id,
}

impl Into<crate::model::kitty::KittyAlbumForCreate> for KittyAlbumForCreate {
    fn into(self) -> crate::model::kitty::KittyAlbumForCreate {
        crate::model::kitty::KittyAlbumForCreate {
            id: uuid::Uuid::new_v4(),
            kitty_id: self.kitty_id.into(),
            album_id: self.album_id.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct KittyImageForCreate {
    pub kitty_id: Id,
    pub image_id: Id,
}

impl Into<crate::model::kitty::KittyImageForCreate> for KittyImageForCreate {
    fn into(self) -> crate::model::kitty::KittyImageForCreate {
        crate::model::kitty::KittyImageForCreate {
            id: uuid::Uuid::new_v4(),
            kitty_id: self.kitty_id.into(),
            image_id: self.image_id.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct KittyScoreForCreate {
    pub kitty_id: Id,
    pub rating_score_id: Id,
}

impl Into<crate::model::kitty::KittyRatingScoreForCreate> for KittyScoreForCreate {
    fn into(self) -> crate::model::kitty::KittyRatingScoreForCreate {
        crate::model::kitty::KittyRatingScoreForCreate {
            id: uuid::Uuid::new_v4(),
            kitty_id: self.kitty_id.into(),
            rating_score_id: self.rating_score_id.into(),
        }
    }
}
