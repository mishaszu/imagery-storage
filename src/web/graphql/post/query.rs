use async_graphql::{Context, Object, Result};

use crate::access::ResourceAccess;
use crate::ctx::Ctx;
use crate::graphql::guard::{Role, RoleGuard};
use crate::model::post::{PostAccess, PostBmc};
use crate::web::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::Post;

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery {
    async fn post(&self, ctx: &Context<'_>, post_id: Id) -> Result<Option<Post>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let user_id = ctx.data_opt::<Ctx>().map(|ctx| ctx.user_id);

        let (access, post) =
            PostBmc::has_access(mm, &post_id.into(), user_id).map_err(GraphQLError::ModelError)?;

        let post = post.map(|post| Post::from_db_with_access(post, access));

        Ok(post)
    }

    async fn posts(&self, ctx: &Context<'_>, user_id: Id) -> Result<Vec<Option<Post>>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let seeker_user_id = ctx.data_opt::<Ctx>().map(|ctx| ctx.user_id);

        let posts =
            PostBmc::has_access_list(mm, seeker_user_id, PostAccess::ToUser(user_id.into()))
                .map_err(GraphQLError::ModelError)?;

        Ok(posts
            .into_iter()
            .map(|(access, post)| post.map(|p| Post::from_db_with_access(p, access)))
            .collect())
    }

    /// get all posts - allowed only for admin
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn posts_all(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let posts = PostBmc::list_admin(mm).map_err(GraphQLError::ModelError)?;

        Ok(posts.into_iter().map(|r| r.into()).collect())
    }
}
