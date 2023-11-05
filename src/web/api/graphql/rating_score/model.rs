use async_graphql::{ComplexObject, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{
    graphql::scalars::Id,
    model::rating_score::RatingScoreBmc,
    web::api::graphql::{image::model::Image, kitty::model::Kitty},
};

#[derive(Debug, Clone, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct RatingScore {
    pub id: Id,
    pub rating_id: Id,
    pub score: i32,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[ComplexObject]
impl RatingScore {
    async fn kitties(&self, ctx: &async_graphql::Context<'_>) -> Result<Vec<Kitty>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let kitties = RatingScoreBmc::get_kitties(mm, self.id.into())
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(kitties.into_iter().map(|r| r.into()).collect())
    }

    async fn images(&self, ctx: &async_graphql::Context<'_>) -> Result<Vec<Image>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let images = RatingScoreBmc::get_images(mm, self.id.0)
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(images.into_iter().map(|r| r.into()).collect())
    }

    async fn albums(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Vec<crate::web::api::graphql::album::model::Album>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let albums = RatingScoreBmc::get_albums(mm, self.id.0)
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(albums.into_iter().map(|r| r.into()).collect())
    }
}

impl From<crate::model::rating_score::RatingScore> for RatingScore {
    fn from(rating_score: crate::model::rating_score::RatingScore) -> Self {
        Self {
            id: Id(rating_score.id),
            rating_id: Id(rating_score.rating_id),
            score: rating_score.score,
            description: rating_score.description,
            created_at: rating_score.created_at.to_string(),
            updated_at: rating_score.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct RatingScoreForCreate {
    pub rating_id: Id,
    pub score: i32,
    pub description: Option<String>,
}

impl Into<crate::model::rating_score::RatingScoreForCreate> for RatingScoreForCreate {
    fn into(self) -> crate::model::rating_score::RatingScoreForCreate {
        crate::model::rating_score::RatingScoreForCreate {
            id: uuid::Uuid::new_v4(),
            rating_id: self.rating_id.0,
            score: self.score,
            description: self.description,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct RatingScoreForUpdate {
    pub rating_id: Option<Id>,
    pub score: Option<i32>,
    pub description: Option<String>,
}

impl Into<crate::model::rating_score::RatingScoreForUpdate> for RatingScoreForUpdate {
    fn into(self) -> crate::model::rating_score::RatingScoreForUpdate {
        crate::model::rating_score::RatingScoreForUpdate {
            rating_id: self.rating_id.map(|id| id.0),
            score: self.score,
            description: self.description,
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}
