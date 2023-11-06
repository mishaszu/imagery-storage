use async_graphql::{Context, Object, Result};

use crate::model::ModelManager;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::tag::TagBmc};

use super::model::{Tag, TagForCreate, TagForUpdate};

#[derive(Default)]
pub struct TagMutation;

#[Object]
impl TagMutation {
    async fn create_tag(&self, ctx: &Context<'_>, tag_for_create: TagForCreate) -> Result<Tag> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let tag = TagBmc::create(mm, tag_for_create.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(tag.into())
    }
    async fn update_tag(
        &self,
        ctx: &Context<'_>,
        id: Id,
        tag_for_udpate: TagForUpdate,
    ) -> Result<Tag> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let tag = TagBmc::update(mm, id.into(), tag_for_udpate.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(tag.into())
    }
    async fn delete_tag(&self, ctx: &Context<'_>, id: Id) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        TagBmc::delete(mm, id.into()).map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Tag deleted".to_string())
    }
}
