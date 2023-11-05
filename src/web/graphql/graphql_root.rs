use async_graphql::{Context, EmptySubscription, MergedObject, Schema};
use reqwest::Client;

use super::error::Error;
use crate::{ctx::Ctx, model::ModelManager};

use super::{
    album::{AlbumMutation, AlbumQuery},
    image::{ImageMutation, ImageQuery},
    tag::{TagMutation, TagQuery},
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
pub struct QueryRoot(DefaultQuery, TagQuery, ImageQuery, AlbumQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(DefaultMutation, TagMutation, ImageMutation, AlbumMutation);

pub type WooBooSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(mm: ModelManager, req_client: Client) -> WooBooSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription::default(),
    )
    .data(mm)
    .data(req_client)
    .finish()
}
