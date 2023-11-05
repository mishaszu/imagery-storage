use async_graphql::{Context, Error, Object, Result};
use reqwest::Client;
use uuid::Uuid;

use crate::model::image::ImageBmc;
use crate::services::lust::Lust;
use crate::services::raw_file::read_file;
use crate::web::api::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::{
    Image, ImageForCreate, ImageForUpdate, ImageRatingScoreForCreate, ImageTagForCreate,
};

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
        let image =
            ImageBmc::create(mm, input.into()).map_err(GraphQLError::from_model_to_graphql)?;
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
        let image = ImageBmc::update(mm, id.0, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(image.into())
    }

    /// Upload image stored in local storage to Lust
    async fn upload_iamge(&self, ctx: &Context<'_>, image: Id) -> Result<Image> {
        let client = ctx
            .data::<Client>()
            .map_err(|_| -> Error { GraphQLError::ClientNotInContext.into() })?;
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let image = ImageBmc::get(mm, image.0).map_err(GraphQLError::from_model_to_graphql)?;

        let file = read_file(&image.path)
            .await
            .map_err(|e| -> async_graphql::Error { e.into() })?;

        let response = Lust::post_file(&client, file)
            .await
            .map_err(|e| -> Error { e.into() })?;

        let updated_image = ImageBmc::update(
            mm,
            image.id,
            crate::model::image::ImageForUpdate {
                path: Some(response.image_id),
                is_uploaded: Some(true),
                ..Default::default()
            },
        )
        .map_err(GraphQLError::from_model_to_graphql)?;

        Ok(updated_image.into())
    }

    async fn upload_iamges(&self, ctx: &Context<'_>, images: Vec<Id>) -> Result<Vec<Image>> {
        let client = ctx
            .data::<Client>()
            .map_err(|_| -> Error { GraphQLError::ClientNotInContext.into() })?;
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let mut updated_images: Vec<Image> = Vec::new();
        for image in images {
            let image = ImageBmc::get(mm, image.0).map_err(GraphQLError::from_model_to_graphql)?;

            let file = read_file(&image.path)
                .await
                .map_err(|e| -> async_graphql::Error { e.into() })?;

            let response = Lust::post_file(&client, file)
                .await
                .map_err(|e| -> Error { e.into() })?;

            let updated_image = ImageBmc::update(
                mm,
                image.id,
                crate::model::image::ImageForUpdate {
                    path: Some(response.image_id),
                    is_uploaded: Some(true),
                    ..Default::default()
                },
            )
            .map_err(GraphQLError::from_model_to_graphql)?;
            updated_images.push(updated_image.into());
        }

        Ok(updated_images)
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
        let image = ImageBmc::get(mm, id.0).map_err(GraphQLError::from_model_to_graphql)?;
        Lust::delete_file(client, image.path)
            .await
            .map_err(|e| -> Error { e.into() })?;
        ImageBmc::delete(mm, id.0).map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Image deleted".to_string())
    }

    async fn delete_many_images(&self, ctx: &Context<'_>, ids: Vec<Id>) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let ids: Vec<Uuid> = ids.into_iter().map(|id| id.0).collect();

        ImageBmc::delete_many(mm, ids).map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Images deleted".to_string())
    }

    async fn add_tag_to_image(
        &self,
        ctx: &Context<'_>,
        input: ImageTagForCreate,
    ) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        ImageBmc::create_tag_image(mm, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Tag added to image".to_string())
    }

    async fn add_score_to_image(
        &self,
        ctx: &Context<'_>,
        input: ImageRatingScoreForCreate,
    ) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        ImageBmc::create_rating_score_image(mm, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Score added to image".to_string())
    }
}
