use async_graphql::{Context, Object, Result};

use crate::model::tag::TagBmc;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::Tag;

#[derive(Default)]
pub struct TagQuery;

#[Object]
impl TagQuery {
    async fn tag(&self, ctx: &Context<'_>, id: Id) -> Result<Tag> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let tag = TagBmc::get(mm, id.0).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(tag.into())
    }
    // async fn tags(
    //     &self,
    //     ctx: &Context<'_>,
    //     name: Option<String>,
    // ) -> Result<Vec<Tag>> {
    //     let mm = ctx.data_opt::<ModelManager>();
    //     let mm = match mm {
    //         Some(mm) => mm,
    //         None => return Err(GraphQLError::ModalManagerNotInContext.into()),
    //     };
    //     let tags = match category_id {
    //         Some(id) => TagBmc::get_by_tag_category_id(mm, id.0, name)
    //             .map_err(GraphQLError::from_model_to_graphql)?,
    //         None => match name {
    //             Some(name) => {
    //                 TagBmc::get_by_name(mm, name).map_err(GraphQLError::from_model_to_graphql)?
    //             }
    //             None => TagBmc::list(mm).map_err(GraphQLError::from_model_to_graphql)?,
    //         },
    //     };
    //     Ok(tags.into_iter().map(|tag| tag.into()).collect())
    // }
}
