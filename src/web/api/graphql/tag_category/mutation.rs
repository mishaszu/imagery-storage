use async_graphql::{Context, Object, Result};

use crate::graphql::scalars::Id;
use crate::model::{tag_category::TagCategoryBmc, ModelManager};
use crate::web::api::graphql::error::Error as GraphQLError;

use super::model::{TagCategory, TagCategoryForCreate, TagCategoryForUpdate};

#[derive(Default)]
pub struct TagCategoryMutation;

#[Object]
impl TagCategoryMutation {
    async fn create_tag_category(
        &self,
        ctx: &Context<'_>,
        tag_for_create: TagCategoryForCreate,
    ) -> Result<TagCategory> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let tag_category = TagCategoryBmc::create(mm, tag_for_create.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(tag_category.into())
    }
    async fn update_tag_category(
        &self,
        ctx: &Context<'_>,
        id: Id,
        tag_for_update: TagCategoryForUpdate,
    ) -> Result<TagCategory> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let tag_category = TagCategoryBmc::update(mm, id.0, tag_for_update.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(tag_category.into())
    }
    async fn delete_tag_category(&self, ctx: &Context<'_>, id: Id) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        TagCategoryBmc::delete(mm, id.0).map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Tag category deleted".to_string())
    }
}
