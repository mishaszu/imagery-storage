use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::graphql::guard::{CreatorGuard, Role, RoleGuard};
use crate::graphql::scalars::Id;
use crate::model::post::PostBmc;
use crate::model::ModelManager;
use crate::web::graphql::error::Error as GraphQLError;

use super::model::{Post, PostForCreate, PostForUpdate, PostImageForCreate};

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    #[graphql(guard = "RoleGuard::new(Role::Creator)")]
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        post: PostForCreate,
        post_images: Vec<Id>,
    ) -> Result<Post> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let user_id = match ctx.data_opt::<crate::ctx::Ctx>() {
            Some(ctx) => ctx.user_id,
            None => return Err(GraphQLError::AuthError.into()),
        };

        let db_post = post.into_db(user_id);
        let db_post_images: Vec<Uuid> = post_images.into_iter().map(|r| r.into()).collect();

        let post = PostBmc::create(mm, db_post, db_post_images)
            .map_err(GraphQLError::from_model_to_graphql)?;

        Ok(post.into())
    }

    #[graphql(guard = "CreatorGuard::new(post_id, false)")]
    async fn update_post(
        &self,
        ctx: &Context<'_>,
        post_id: Id,
        post: PostForUpdate,
    ) -> Result<Post> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let post = PostBmc::update(mm, &post_id.into(), post.into())
            .map_err(GraphQLError::from_model_to_graphql)?;

        Ok(post.into())
    }

    #[graphql(guard = "CreatorGuard::new(post_id, true)")]
    async fn delete_post_image(
        &self,
        ctx: &Context<'_>,
        post_id: Id,
        post_image_id: Id,
    ) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Ok("".to_string()),
        };

        todo!()
    }

    #[graphql(guard = "CreatorGuard::new(post_id, true)")]
    async fn delete_post(&self, ctx: &Context<'_>, post_id: Id) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Ok("".to_string()),
        };

        PostBmc::delete(mm, &post_id.into()).map_err(GraphQLError::from_model_to_graphql)?;

        Ok("Post deleted".to_string())
    }
}
