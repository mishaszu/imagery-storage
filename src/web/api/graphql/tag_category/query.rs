use async_graphql::{Context, Object, Result};

use crate::{
    graphql::scalars::Id,
    model::{tag_category::TagCategoryBmc, ModelManager},
    web::api::graphql::error::Error as GraphQLError,
};

use super::model::TagCategory;

#[derive(Default)]
pub struct TagCategoryQuery;

#[Object]
impl TagCategoryQuery {
    async fn tag_category(&self, ctx: &Context<'_>, id: Id) -> Result<TagCategory> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let tag_category =
            TagCategoryBmc::get(mm, id.0).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(tag_category.into())
    }
    async fn tag_categories(
        &self,
        ctx: &Context<'_>,
        name: Option<String>,
    ) -> Result<Vec<TagCategory>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let tag_categories = match name {
            Some(name) => TagCategoryBmc::get_by_name(mm, name)
                .map_err(GraphQLError::from_model_to_graphql)?,
            None => TagCategoryBmc::list(mm).map_err(GraphQLError::from_model_to_graphql)?,
        };
        Ok(tag_categories
            .into_iter()
            .map(|tag_category| tag_category.into())
            .collect())
    }
}
