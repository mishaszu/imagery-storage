use async_graphql::{Context, Object, Result};

use crate::ctx::Ctx;
use crate::graphql::guard::{Accessship, Role, RoleGuard};
use crate::model::post::PostBmc;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::Post;

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery {
    async fn post(&self, ctx: &Context<'_>, post_id: Id) -> Result<Post> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let app_ctx = ctx.data_opt::<Ctx>();
        let user_account_id = match app_ctx {
            Some(ctx) => ctx.account_id,
            None => return Err(GraphQLError::AuthError.into()),
        };

        let post =
            PostBmc::get(mm, &post_id.into()).map_err(GraphQLError::from_model_to_graphql)?;

        let access = post
            .user_access(mm, &user_account_id)
            .map_err(GraphQLError::from_model_to_graphql)?;

        match (access, post.public_lvl) {
            //
            (Accessship::AllowedSubscriber, 1)
            | (Accessship::AllowedSubscriber, 2)
            | (Accessship::AllowedPublic, 2)
            | (Accessship::Admin, _)
            | (Accessship::Owner, _) => Ok(post.into()),
            _ => return Err(GraphQLError::AuthError.into()),
        }
    }

    async fn posts(&self, ctx: &Context<'_>, user_id: Option<Id>) -> Result<Vec<Option<Post>>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let app_ctx = ctx.data_opt::<Ctx>();
        let (search_user_id, user_account_id) = match (user_id, app_ctx) {
            (Some(user_id), Some(user)) => (user_id.into(), user.account_id),
            (None, Some(user)) => (user.user_id.into(), user.account_id),
            _ => return Ok(vec![]),
        };

        let posts = PostBmc::list(mm, user_account_id, &search_user_id)
            .map_err(GraphQLError::from_model_to_graphql)?;

        Ok(posts.into_iter().map(|r| r.map(|r| r.into())).collect())
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn posts_all(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let posts = PostBmc::list_admin(mm).map_err(GraphQLError::from_model_to_graphql)?;

        Ok(posts.into_iter().map(|r| r.into()).collect())
    }
}
