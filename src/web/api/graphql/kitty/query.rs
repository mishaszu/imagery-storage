use async_graphql::{Context, Object, Result};

use crate::model::kitty::KittyBmc;
use crate::web::api::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

#[derive(Default)]
pub struct KittyQuery;

#[Object]
impl KittyQuery {
    async fn kitty(
        &self,
        ctx: &Context<'_>,
        id: Id,
    ) -> Result<crate::web::api::graphql::kitty::model::Kitty> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let kitty = KittyBmc::get(mm, id.into()).map_err(GraphQLError::from_model_to_graphql)?;

        Ok(kitty.into())
    }

    async fn kitties(
        &self,
        ctx: &Context<'_>,
        name: Option<String>,
    ) -> Result<Vec<crate::web::api::graphql::kitty::model::Kitty>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let kitties = match name {
            Some(name) => {
                KittyBmc::get_by_name(mm, name).map_err(GraphQLError::from_model_to_graphql)?
            }
            None => KittyBmc::list(mm).map_err(GraphQLError::from_model_to_graphql)?,
        };

        Ok(kitties.into_iter().map(|k| k.into()).collect())
    }
}
