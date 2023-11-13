use async_graphql::{Context, Object, Result};

use crate::graphql::guard::UserQueryGuard;
use crate::model::user::UserBmc;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::{User, UserSelf};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard = "UserQueryGuard::new(id)")]
    async fn user(&self, ctx: &Context<'_>, id: Id) -> Result<User> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let user = UserBmc::get(mm, &id.into()).map_err(GraphQLError::ModelError)?;

        Ok(user.into())
    }

    async fn self_user(&self, ctx: &Context<'_>) -> Result<UserSelf> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let user_id = ctx.data_opt::<crate::ctx::Ctx>().map(|r| r.user_id);

        match user_id {
            Some(user_id) => {
                let user = UserBmc::get(mm, &user_id).map_err(GraphQLError::ModelError)?;
                Ok(user.into())
            }
            None => Err(GraphQLError::AuthError.into()),
        }
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let user_account_id = ctx.data_opt::<crate::ctx::Ctx>().map(|r| r.account_id);

        let users = UserBmc::list(mm, user_account_id).map_err(GraphQLError::ModelError)?;

        Ok(users.into_iter().map(|r| r.into()).collect())
    }
}
