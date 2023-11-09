use async_graphql::{Context, Object, Result};

use crate::graphql::guard::{Role, RoleGuard};
use crate::model::user::UserBmc;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::{User, UserForCreate};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn create_user(&self, ctx: &Context<'_>, input: UserForCreate) -> Result<User> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let create: crate::model::user::UserForCreate = input.into_db()?;
        let user = UserBmc::create(mm, create).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(user.into())
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn delete_user(&self, ctx: &Context<'_>, id: Id) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let user = UserBmc::get(mm, &id.into()).map_err(GraphQLError::from_model_to_graphql)?;

        UserBmc::delete(mm, &user.id).map_err(GraphQLError::from_model_to_graphql)?;

        Ok(user.id.to_string())
    }
}
