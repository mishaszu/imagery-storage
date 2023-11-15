use async_graphql::{Context, Object, Result};

use crate::access::ResourceAccess;
use crate::model::user::UserBmc;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::{User, UserSelf};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(&self, ctx: &Context<'_>, id: Id) -> Result<User> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let user_account_id = ctx.data_opt::<crate::ctx::Ctx>().map(|r| r.user_id);

        let user = UserBmc::has_access(mm, &id.into(), user_account_id)
            .map_err(GraphQLError::ModelError)?;

        Ok(user.try_into()?)
    }

    async fn self_user(&self, ctx: &Context<'_>) -> Result<UserSelf> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let user_account_id = ctx.data_opt::<crate::ctx::Ctx>();
        let user_account_id = match user_account_id {
            Some(user_account_id) => user_account_id.user_id,
            None => return Err(GraphQLError::AuthError.into()),
        };

        let user = UserBmc::has_access(mm, &user_account_id, Some(user_account_id))
            .map_err(GraphQLError::ModelError)?;

        Ok(user.try_into()?)
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let user_account_id = ctx.data_opt::<crate::ctx::Ctx>().map(|r| r.account_id);

        let users =
            UserBmc::has_access_list(mm, user_account_id).map_err(GraphQLError::ModelError)?;

        let users = users
            .into_iter()
            .filter(|(_, user)| user.is_some())
            .map(|(access, user)| (access, user.unwrap()))
            .map(|r| r.into())
            .collect::<Vec<User>>();

        Ok(users)
    }
}
