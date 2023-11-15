use async_graphql::{Context, Guard, Result};
use uuid::Uuid;

use crate::access::Accesship;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{
    ctx::Ctx,
    graphql::scalars::Id,
    model::{account::AccountBmc, ModelManager},
};

pub struct UserQueryGuard {
    id: Uuid,
}

impl UserQueryGuard {
    pub fn new(id: Id) -> Self {
        Self { id: id.into() }
    }
}

#[async_trait::async_trait]
impl Guard for UserQueryGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let app_ctx = ctx.data_opt::<Ctx>();
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let user_account_id = match app_ctx {
            Some(ctx) => ctx.account_id,
            None => return Err(GraphQLError::AccessError("No user logged in".to_string()).into()),
        };

        let user_account =
            AccountBmc::get(mm, &self.id.into()).map_err(GraphQLError::ModelError)?;

        let has_access = user_account
            .has_access(mm, Some(user_account_id))
            .map_err(GraphQLError::ModelError)?;

        match has_access {
            Accesship::None => Err(GraphQLError::AccessError(user_account_id.to_string()).into()),
            _ => Ok(()),
        }
    }
}
