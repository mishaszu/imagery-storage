use async_graphql::{Context, MergedObject, Schema};
use reqwest::Client;

use crate::web::api::graphql::error::Error;
use crate::{ctx::Ctx, model::ModelManager};

use super::{
    album::{AlbumMutation, AlbumQuery},
    image::{ImageMutation, ImageQuery},
    kitty::{KittyMutation, KittyQuery},
    link::{LinkMutation, LinkQuery},
    rating::{RatingMutation, RatingQuery},
    rating_score::{RatingScoreMutation, RatingScoreQuery},
    raw_image::{subscription::RawImageSubscription, RawImageMutation, RawImageQuery},
    tag::{TagMutation, TagQuery},
    tag_category::{TagCategoryMutation, TagCategoryQuery},
};

#[derive(Default)]
struct DefaultQuery;

#[async_graphql::Object]
impl DefaultQuery {
    async fn hello(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
        let user_ctx = ctx.data_opt::<Ctx>();
        match user_ctx {
            Some(ctx) => Ok(format!("User logged in: {}", ctx.name)),
            None => Err(Error::AuthError.into()),
        }
    }
}

#[derive(Default)]
struct DefaultMutation;

#[async_graphql::Object]
impl DefaultMutation {
    async fn hello(&self, input: String) -> String {
        format!("{} world!", input)
    }
}

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    DefaultQuery,
    LinkQuery,
    TagCategoryQuery,
    TagQuery,
    RatingQuery,
    RatingScoreQuery,
    ImageQuery,
    KittyQuery,
    AlbumQuery,
    RawImageQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    DefaultMutation,
    LinkMutation,
    TagCategoryMutation,
    TagMutation,
    RatingMutation,
    RatingScoreMutation,
    ImageMutation,
    KittyMutation,
    AlbumMutation,
    RawImageMutation,
);

pub type WooBooSchema = Schema<QueryRoot, MutationRoot, RawImageSubscription>;

pub fn create_schema(mm: ModelManager, req_client: Client) -> WooBooSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        RawImageSubscription,
    )
    .data(mm)
    .data(req_client)
    .finish()
}
