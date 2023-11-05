use async_graphql::{Context, Error, Object, Result};
use reqwest::Client;

use crate::services::{lust::Lust, raw_file::read_file};
use crate::web::api::graphql::error::Error as GraphQLError;

use super::model::LustRes;

#[derive(Default)]
pub struct RawImageMutation;

#[Object]
impl RawImageMutation {
    async fn post_raw_image(
        &self,
        ctx: &Context<'_>,
        image_id: String,
        dir_id: String,
    ) -> Result<LustRes> {
        let client = ctx
            .data::<Client>()
            .map_err(|_| -> Error { GraphQLError::ClientNotInContext.into() })?;

        let file = read_file(&format!("{}/{}", dir_id, image_id))
            .await
            .map_err(|e| -> async_graphql::Error { e.into() })?;

        let response = Lust::post_file(&client, file)
            .await
            .map_err(|e| -> Error { e.into() })?;
        Ok(response.into())
    }

    async fn delete_raw_image(&self, ctx: &Context<'_>, image_id: String) -> Result<String> {
        let client = ctx
            .data::<Client>()
            .map_err(|_| -> Error { GraphQLError::ClientNotInContext.into() })?;

        Lust::delete_file(client, image_id)
            .await
            .map_err(|e| -> Error { e.into() })?;

        Ok("File Deleted from Lust".to_string())
    }
}
