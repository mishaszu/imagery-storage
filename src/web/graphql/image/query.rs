use async_graphql::{Context, Object, Result};

use crate::web::api::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::Image;

#[derive(Default)]
pub struct ImageQuery;

#[Object]
impl ImageQuery {
    async fn image(&self, ctx: &Context<'_>, id: Id) -> Result<Image> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let image = crate::model::image::ImageBmc::get(mm, id.0)
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(image.into())
    }

    async fn images(&self, ctx: &Context<'_>) -> Result<Vec<Image>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let images =
            crate::model::image::ImageBmc::list(mm).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(images.into_iter().map(|r| r.into()).collect())
    }
}
