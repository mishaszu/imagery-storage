use async_graphql::{Context, Error, Object, Result};
use reqwest::Client;

use crate::config;
use crate::ctx::Ctx;
use crate::graphql::guard::ImageCreatorGuard;
use crate::model::account::AccountBmc;
use crate::model::image::ImageBmc;
use crate::services::lust::Lust;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::{Image, ImageDeleteResult, ImageForUpdate};

#[derive(Default)]
pub struct ImageMutation;

#[Object]
impl ImageMutation {
    #[graphql(guard = "ImageCreatorGuard::new(id, false)")]
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

    #[graphql(guard = "ImageCreatorGuard::new(id, true)")]
    async fn delete_image(&self, ctx: &Context<'_>, id: Id) -> Result<ImageDeleteResult> {
        println!("id: {:?}", id);
        let client = ctx
            .data::<Client>()
            .map_err(|_| -> Error { GraphQLError::ClientNotInContext.into() })?;
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        println!("id: {:?}", id);
        let image = ImageBmc::get(mm, &id.into()).map_err(GraphQLError::ModelError)?;
        let lust_del_res = Lust::delete_file(client, &image.kind, image.path.to_string()).await;
        match lust_del_res {
            Ok(_) => {
                let image = ImageBmc::delete(mm, id.0).map_err(GraphQLError::ModelError);
                match image {
                    Ok(_) => Ok(ImageDeleteResult::delete_success(id)),
                    Err(e) => Ok(ImageDeleteResult::delete_failure(id, e.to_string())),
                }
            }
            Err(e) => Ok(ImageDeleteResult::delete_failure(id, e.to_string())),
        }
    }

    async fn delete_many_images(
        &self,
        ctx: &Context<'_>,
        ids: Vec<Id>,
    ) -> Result<Vec<ImageDeleteResult>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let client = ctx
            .data::<Client>()
            .map_err(|_| -> Error { GraphQLError::ClientNotInContext.into() })?;

        let (user_id, account_id) = ctx
            .data::<&Ctx>()
            .map(|ctx| (ctx.user_id, ctx.account_id))
            .map_err(|_| -> Error {
                GraphQLError::AccessError("User not in context".to_string()).into()
            })?;

        let account = AccountBmc::get(mm, &account_id).map_err(GraphQLError::ModelError)?;
        let is_admin = account.is_admin;

        let mut img_result: Vec<ImageDeleteResult> = Vec::new();

        for id in ids {
            let image = ImageBmc::get(mm, &id.into()).map_err(GraphQLError::ModelError);

            let image = match image {
                Ok(image) => image,
                Err(e) => {
                    img_result.push(ImageDeleteResult::delete_failure(id, e.to_string()));
                    continue;
                }
            };

            if image.user_id == user_id || is_admin {
                let file_delete_result =
                    Lust::delete_file(&client, &config().LUST_IMAGE_BUCKET, image.path.to_string())
                        .await;

                match file_delete_result {
                    Ok(_) => (),
                    Err(e) => {
                        img_result.push(ImageDeleteResult::delete_failure(id, e.to_string()));
                        continue;
                    }
                }

                let result = ImageBmc::delete(mm, id.into()).map_err(GraphQLError::ModelError);

                match result {
                    Ok(_) => {
                        img_result.push(ImageDeleteResult::delete_success(id));
                    }
                    Err(e) => {
                        img_result.push(ImageDeleteResult::delete_failure(id, e.to_string()));
                        continue;
                    }
                }
            } else {
                img_result.push(ImageDeleteResult::delete_failure(
                    id,
                    "Access denied".to_string(),
                ))
            }
        }

        Ok(img_result)
    }
}
