use async_graphql::{Context, Object, Result};

use crate::model::rating_score::RatingScoreBmc;
use crate::web::api::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::{RatingScore, RatingScoreForCreate, RatingScoreForUpdate};

#[derive(Default)]
pub struct RatingScoreMutation;

#[Object]
impl RatingScoreMutation {
    async fn create_rating_score(
        &self,
        ctx: &Context<'_>,
        input: RatingScoreForCreate,
    ) -> Result<RatingScore> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let rating_score = RatingScoreBmc::create(mm, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(rating_score.into())
    }
    async fn update_rating_score(
        &self,
        ctx: &Context<'_>,
        id: Id,
        input: RatingScoreForUpdate,
    ) -> Result<RatingScore> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let rating_score = RatingScoreBmc::update(mm, id.0, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(rating_score.into())
    }
    async fn delete_rating_score(&self, ctx: &Context<'_>, id: Id) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        RatingScoreBmc::delete(mm, id.0).map_err(GraphQLError::from_model_to_graphql)?;
        Ok("RatingScore deleted".to_string())
    }
}
