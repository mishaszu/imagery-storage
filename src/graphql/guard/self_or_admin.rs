use async_graphql::{Context, Guard, Result};
use uuid::Uuid;

use crate::web::graphql::error::Error as GraphQLError;
use crate::{
    ctx::Ctx,
    model::{account::AccountBmc, ModelManager},
};

pub struct SelfOrAdminGuard {
    id: Uuid,
}

impl SelfOrAdminGuard {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[async_trait::async_trait]
impl Guard for SelfOrAdminGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let app_ctx = ctx.data_opt::<Ctx>();
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let (account_id, user_id) = match app_ctx {
            Some(ctx) => (ctx.account_id, ctx.user_id),
            None => return Err(GraphQLError::AccessError("No user logged in".to_string()).into()),
        };

        if user_id == self.id {
            return Ok(());
        }

        let account = AccountBmc::get(mm, &account_id).map_err(GraphQLError::ModelError)?;

        if account.is_admin {
            return Ok(());
        } else {
            return Err(GraphQLError::AuthError.into());
        }
    }
}
