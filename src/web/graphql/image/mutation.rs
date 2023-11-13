use async_graphql::{Context, Error, Object, Result};
use reqwest::Client;
use uuid::Uuid;

use crate::model::image::ImageBmc;
use crate::services::lust::Lust;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::{Image, ImageForCreate, ImageForUpdate};

#[derive(Default)]
pub struct ImageMutation;

#[Object]
impl ImageMutation {
    async fn create_image(&self, ctx: &Context<'_>, input: ImageForCreate) -> Result<Image> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let image = ImageBmc::create(mm, input.into()).map_err(GraphQLError::ModelError)?;
        Ok(image.into())
    }

    async fn update_image(
        &self,
        ctx: &Context<'_>,
        id: Id,
        input: ImageForUpdate,
    ) -> Result<Image> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let image = ImageBmc::update(mm, id.0, input.into()).map_err(GraphQLError::ModelError)?;
        Ok(image.into())
    }

    async fn delete_image(&self, ctx: &Context<'_>, id: Id) -> Result<String> {
        let client = ctx
            .data::<Client>()
            .map_err(|_| -> Error { GraphQLError::ClientNotInContext.into() })?;
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let image = ImageBmc::get(mm, id.0).map_err(GraphQLError::ModelError)?;
        Lust::delete_file(client, image.path.to_string())
            .await
            .map_err(|e| -> Error { e.into() })?;
        ImageBmc::delete(mm, id.0).map_err(GraphQLError::ModelError)?;
        Ok("Image deleted".to_string())
    }

    async fn delete_many_images(&self, ctx: &Context<'_>, ids: Vec<Id>) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let ids: Vec<Uuid> = ids.into_iter().map(|id| id.0).collect();

        ImageBmc::delete_many(mm, ids).map_err(GraphQLError::ModelError)?;
        Ok("Images deleted".to_string())
    }
}
