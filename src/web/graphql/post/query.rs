use async_graphql::{Context, Object, Result};

use crate::ctx::Ctx;
use crate::graphql::guard::{Accessship, Role, RoleGuard};
use crate::model::account::AccountBmc;
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
        let user_account_id = app_ctx.map(|r| r.account_id);

        let post =
            PostBmc::get(mm, &post_id.into()).map_err(GraphQLError::from_model_to_graphql)?;

        let has_access = AccountBmc::has_access(mm, user_account_id, &post.user_id)
            .map_err(GraphQLError::from_model_to_graphql)?;

        match (has_access, post.public_lvl) {
            //
            (Accessship::AllowedSubscriber, 1)
            | (Accessship::AllowedSubscriber, 2)
            | (Accessship::AllowedPublic, 2)
            | (Accessship::Admin, _)
            | (Accessship::Owner, _) => Ok(post.into()),
            _ => return Err(GraphQLError::AuthError.into()),
        }
    }

    async fn posts(&self, ctx: &Context<'_>, user_id: Id) -> Result<Vec<Post>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let app_ctx = ctx.data_opt::<Ctx>();
        let user_account_id = app_ctx.map(|r| r.account_id);

        let posts = PostBmc::list(mm, user_account_id, &user_id.into())
            .map_err(GraphQLError::from_model_to_graphql)?;

        Ok(posts.into_iter().map(|r| r.into()).collect())
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
