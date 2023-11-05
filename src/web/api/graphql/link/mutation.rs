use async_graphql::{Context, Object, Result};

use crate::{
    graphql::scalars::Id,
    model::{link::LinkBmc, ModelManager},
    web::api::graphql::error::Error as GraphQLError,
};

use super::model::{LinkForCreate, LinkForUpdate, LinkOutput};

#[derive(Default)]
pub struct LinkMutation;

#[Object]
impl LinkMutation {
    async fn create_link(&self, ctx: &Context<'_>, link: LinkForCreate) -> Result<LinkOutput> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let link = LinkBmc::create(mm, link.into()).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(link.into())
    }
    async fn update_link(
        &self,
        ctx: &Context<'_>,
        id: Id,
        link: LinkForUpdate,
    ) -> Result<LinkOutput> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let link =
            LinkBmc::update(mm, id.0, link.into()).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(link.into())
    }
    async fn delete_link(&self, ctx: &Context<'_>, id: Id) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        LinkBmc::delete(mm, id.0).map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Link deleted".to_string())
    }
}
