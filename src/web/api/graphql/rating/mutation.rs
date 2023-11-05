use async_graphql::{Context, Object, Result};

use crate::{
    graphql::scalars::Id,
    model::{rating::RatingBmc, ModelManager},
    web::api::graphql::error::Error as GraphQLError,
};

use super::model::{Rating, RatingForCreate, RatingForUpdate};

#[derive(Default)]
pub struct RatingMutation;

#[Object]
impl RatingMutation {
    async fn create_rating(&self, ctx: &Context<'_>, input: RatingForCreate) -> Result<Rating> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let rating =
            RatingBmc::create(mm, input.into()).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(rating.into())
    }
    async fn update_rating(
        &self,
        ctx: &Context<'_>,
        id: Id,
        input: RatingForUpdate,
    ) -> Result<Rating> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let rating = RatingBmc::update(mm, id.0, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(rating.into())
    }
    async fn delete_rating(&self, ctx: &Context<'_>, id: Id) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        RatingBmc::delete(mm, id.0).map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Rating deleted".to_string())
    }
}
