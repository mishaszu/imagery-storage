use async_graphql::{Context, Object, Result};

use crate::model::rating::RatingBmc;
use crate::web::api::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::Rating;

#[derive(Default)]
pub struct RatingQuery;

#[Object]
impl RatingQuery {
    async fn rating(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Rating> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let rating = RatingBmc::get(mm, id.0).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(rating.into())
    }
    async fn ratings(&self, ctx: &Context<'_>) -> Result<Vec<Rating>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let ratings = RatingBmc::list(mm).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(ratings.into_iter().map(|r| r.into()).collect())
    }
}
