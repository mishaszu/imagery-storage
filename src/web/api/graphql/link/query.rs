use async_graphql::{Context, Object, Result};

use crate::web::api::graphql::error::Error as GraphQLError;
use crate::{
    graphql::scalars::Id,
    model::{link::LinkBmc, ModelManager},
};

use super::model::LinkOutput;

#[derive(Default)]
pub struct LinkQuery;

#[Object]
impl LinkQuery {
    async fn link(&self, ctx: &Context<'_>, id: Id) -> Result<LinkOutput> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let link = LinkBmc::get(mm, id.0).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(link.into())
    }
    async fn links(&self, ctx: &Context<'_>) -> Result<Vec<LinkOutput>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let links = LinkBmc::list(mm).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(links.into_iter().map(|link| link.into()).collect())
    }
}
