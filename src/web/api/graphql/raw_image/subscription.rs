use async_graphql::*;
use async_graphql::{InputObject, Result};
use futures_util::stream::Stream;
use reqwest::Client;
use serde::Deserialize;

use super::model::LustRes;

use crate::graphql::scalars::Id;
use crate::model::image::ImageBmc;
use crate::model::ModelManager;
use crate::services::{lust::Lust, raw_file::read_file};
use crate::web::api::graphql::error::Error as GraphQLError;

#[derive(Default)]
pub struct RawImageSubscription;

#[derive(Deserialize, InputObject)]
pub struct UploadImage {
    id: Id,
    image_id: String,
    album_id: String,
}

#[Subscription]
impl RawImageSubscription {
    async fn test(&self, ctx: &Context<'_>) -> impl Stream<Item = String> {
        let mm = ctx.data::<ModelManager>();
        let mm = match mm {
            Ok(mm) => mm.clone(),
            Err(_) => return panic!("ModelManager not in context"),
        };
        async_stream::stream! {
            let test = mm;
            for i in 0..10 {
                yield format!("test {}", i);
            }
        }
    }
    async fn multiimage(
        &self,
        ctx: &Context<'_>,
        images: Vec<UploadImage>,
    ) -> Result<impl Stream<Item = Result<LustRes>>> {
        let client = Client::new();
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm.clone(),
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        // Returns the number from 0 to `condition`.
        let stream = async_stream::stream! {
            for image in images {
                // make tokio wait for 2 seconds
                let file = read_file(&format!("{}/{}", image.album_id, image.image_id))
                    .await
                    .map_err(|e| -> async_graphql::Error { e.into() });
                let res: Result<LustRes> = match file {
                    Ok(file) => {
                        let res = Lust::post_file(&client, file).await.map(|res| {
                            res.into()
                        }).map_err(|e| -> async_graphql::Error { e.into()});
                        res
                    },
                    Err(e) => {
                        Err(e.into())
                    }
                };
                match res {
                    Ok(res) => {
                        ImageBmc::update(
                            &mm,
                            image.id.into(),
                            crate::model::image::ImageForUpdate {
                                path: Some(res.image_id.clone()),
                                is_uploaded: Some(true),
                                ..Default::default()
                            },
                        ).map_err(GraphQLError::from_model_to_graphql)?;
                        yield Ok(res);
                    },
                    Err(e) => {
                        yield Err(e);
                    }
                }
            }
        };
        Ok(stream)
    }
}
