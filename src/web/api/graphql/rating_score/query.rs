use async_graphql::{Context, Object, Result};

use crate::model::rating_score::RatingScoreBmc;
use crate::web::api::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::RatingScore;

#[derive(Default)]
pub struct RatingScoreQuery;

#[Object]
impl RatingScoreQuery {
    async fn rating_score(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<RatingScore> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let rating_score =
            RatingScoreBmc::get(mm, id.0).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(rating_score.into())
    }
    async fn rating_scores(&self, ctx: &Context<'_>) -> Result<Vec<RatingScore>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let rating_scores =
            RatingScoreBmc::list(mm).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(rating_scores.into_iter().map(|r| r.into()).collect())
    }
}
