use async_graphql::{ComplexObject, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{graphql::scalars::Id, model::image::ImageBmc, web::api::graphql::kitty::model::Kitty};

#[derive(Debug, Clone, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct Image {
    pub id: Id,
    pub title: Option<String>,
    pub description: Option<String>,
    pub path: String,
    pub is_uploaded: bool,
    pub is_printable: bool,
    pub is_printed: bool,
    pub is_favorite: bool,
    pub is_hidden: bool,
    pub image_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[ComplexObject]
impl Image {
    async fn kitties(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<Kitty>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let kitties = ImageBmc::get_kitties(mm, self.id.into())
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(kitties.into_iter().map(|r| r.into()).collect())
    }

    async fn scores(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<crate::web::api::graphql::rating_score::model::RatingScore>>
    {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let scores = ImageBmc::get_scores(mm, self.id.0)
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(scores.into_iter().map(|r| r.into()).collect())
    }

    async fn tags(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<crate::web::api::graphql::tag::model::Tag>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let tags = ImageBmc::get_tags(mm, self.id.0)
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(tags.into_iter().map(|r| r.into()).collect())
    }

    async fn albums(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<crate::web::api::graphql::album::model::Album>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let albums = ImageBmc::get_albums(mm, self.id.0)
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(albums.into_iter().map(|r| r.into()).collect())
    }
}

impl From<crate::model::image::Image> for Image {
    fn from(image: crate::model::image::Image) -> Self {
        Self {
            id: Id(image.id),
            title: image.title,
            description: image.description,
            path: image.path,
            is_uploaded: image.is_uploaded,
            is_printable: image.is_printable,
            is_printed: image.is_printed,
            is_favorite: image.is_favorite,
            is_hidden: image.is_hidden,
            image_date: image.image_date.map(|d| d.to_string()),
            created_at: image.created_at.to_string(),
            updated_at: image.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct ImageForCreate {
    pub path: String,
}

impl Into<crate::model::image::ImageForCreate> for ImageForCreate {
    fn into(self) -> crate::model::image::ImageForCreate {
        crate::model::image::ImageForCreate {
            id: uuid::Uuid::new_v4(),
            path: self.path,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct ImageForUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub path: Option<String>,
    pub is_uploaded: Option<bool>,
    pub is_printable: Option<bool>,
    pub is_printed: Option<bool>,
    pub is_favorite: Option<bool>,
    pub is_hidden: Option<bool>,
    pub image_date: Option<String>,
}

impl Into<crate::model::image::ImageForUpdate> for ImageForUpdate {
    fn into(self) -> crate::model::image::ImageForUpdate {
        crate::model::image::ImageForUpdate {
            title: self.title,
            description: self.description,
            path: self.path,
            is_uploaded: self.is_uploaded,
            is_printable: self.is_printable,
            is_printed: self.is_printed,
            is_favorite: self.is_favorite,
            is_hidden: self.is_hidden,
            image_date: self.image_date.map(|d| d.parse().unwrap()),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct ImageTagForCreate {
    pub image_id: Id,
    pub tag_id: Id,
}

impl Into<crate::model::image::ImageTagForCreate> for ImageTagForCreate {
    fn into(self) -> crate::model::image::ImageTagForCreate {
        crate::model::image::ImageTagForCreate {
            id: uuid::Uuid::new_v4(),
            image_id: self.image_id.0,
            tag_id: self.tag_id.0,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct ImageRatingScoreForCreate {
    pub image_id: Id,
    pub rating_score_id: Id,
}

impl Into<crate::model::image::ImageRatingScoreForCreate> for ImageRatingScoreForCreate {
    fn into(self) -> crate::model::image::ImageRatingScoreForCreate {
        crate::model::image::ImageRatingScoreForCreate {
            id: uuid::Uuid::new_v4(),
            image_id: self.image_id.0,
            rating_score_id: self.rating_score_id.0,
        }
    }
}
