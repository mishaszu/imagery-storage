use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    graphql::scalars::Id,
    model::{album::AlbumBmc, image::ImageBmc, ModelManager},
    web::api::graphql::{image::model::Image, rating_score::model::RatingScore, tag::model::Tag},
};

#[derive(Debug, Clone, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct Album {
    pub id: Id,
    pub title: String,
    pub description: Option<String>,
    pub picture_id: Option<Id>,
    pub is_favorite: bool,
    pub is_hidden: bool,
    pub is_print_album: bool,
    pub is_printed: bool,
    pub image_per_page: Option<i32>,
    pub album_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[ComplexObject]
impl Album {
    async fn picture(&self, ctx: &Context<'_>) -> Result<Option<Image>> {
        match self.picture_id {
            Some(id) => {
                let mm = ctx.data_opt::<ModelManager>();
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

    async fn images(&self, ctx: &Context<'_>) -> Result<Vec<Image>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let images = AlbumBmc::get_images(mm, self.id.into())
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(images.into_iter().map(|r| r.into()).collect())
    }

    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<Tag>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let tags = AlbumBmc::get_tags(mm, self.id.into())
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(tags.into_iter().map(|t| t.into()).collect())
    }

    async fn scores(&self, ctx: &Context<'_>) -> Result<Vec<RatingScore>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let scores = AlbumBmc::get_scores(mm, self.id.0)
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(scores.into_iter().map(|r| r.into()).collect())
    }

    async fn kitties(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<crate::web::api::graphql::kitty::model::Kitty>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let kitties = AlbumBmc::get_kitties(mm, self.id.into())
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(kitties.into_iter().map(|r| r.into()).collect())
    }
}

impl From<crate::model::album::Album> for Album {
    fn from(album: crate::model::album::Album) -> Self {
        Self {
            id: Id(album.id),
            title: album.title,
            description: album.description,
            picture_id: album.picture_id.map(|id| Id(id)),
            is_favorite: album.is_favorite,
            is_hidden: album.is_hidden,
            is_print_album: album.is_print_album,
            is_printed: album.is_printed,
            image_per_page: album.image_per_page,
            album_date: album.album_date.map(|d| d.to_string()),
            created_at: album.created_at.to_string(),
            updated_at: album.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct AlbumForCreate {
    pub title: String,
    pub description: Option<String>,
}

impl Into<crate::model::album::AlbumForCreate> for AlbumForCreate {
    fn into(self) -> crate::model::album::AlbumForCreate {
        crate::model::album::AlbumForCreate {
            id: Uuid::new_v4(),
            title: self.title,
            description: self.description,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct AlbumForUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub picture_id: Option<Id>,
    pub is_favorite: Option<bool>,
    pub is_hidden: Option<bool>,
    pub is_print_album: Option<bool>,
    pub is_printed: Option<bool>,
    pub image_per_page: Option<i32>,
    pub album_date: Option<String>,
}

impl Into<crate::model::album::AlbumForUpdate> for AlbumForUpdate {
    fn into(self) -> crate::model::album::AlbumForUpdate {
        crate::model::album::AlbumForUpdate {
            title: self.title,
            description: self.description,
            picture_id: self.picture_id.map(|id| id.0),
            is_favorite: self.is_favorite,
            is_hidden: self.is_hidden,
            is_print_album: self.is_print_album,
            is_printed: self.is_printed,
            image_per_page: self.image_per_page,
            album_date: self.album_date.map(|d| d.parse().unwrap()),
            updated_at: chrono::Local::now().naive_local(),
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
            album_id: self.album_id.0,
            image_id: self.image_id.0,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct AlbumTagForCreate {
    pub album_id: Id,
    pub tag_id: Id,
}

impl Into<crate::model::album::AlbumTagForCreate> for AlbumTagForCreate {
    fn into(self) -> crate::model::album::AlbumTagForCreate {
        crate::model::album::AlbumTagForCreate {
            id: Uuid::new_v4(),
            album_id: self.album_id.0,
            tag_id: self.tag_id.0,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct AlbumScoreForCreate {
    pub album_id: Id,
    pub rating_score_id: Id,
}

impl Into<crate::model::album::AlbumRatingScoreForCreate> for AlbumScoreForCreate {
    fn into(self) -> crate::model::album::AlbumRatingScoreForCreate {
        crate::model::album::AlbumRatingScoreForCreate {
            id: Uuid::new_v4(),
            album_id: self.album_id.into(),
            rating_score_id: self.rating_score_id.into(),
        }
    }
}
