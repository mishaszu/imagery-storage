use async_graphql::{
    Context, Enum, Error, Guard, InputValueError, InputValueResult, Result, Scalar, ScalarType,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
            None => return Err("Unauthorized 3".into()),
        };

        let (account_id, user_id) = match app_ctx {
            Some(ctx) => (ctx.account_id, ctx.user_id),
            None => return Err("Unauthorized 1".into()),
        };

        if user_id == self.id {
            return Ok(());
        }

        let account =
            AccountBmc::get(mm, &account_id).map_err(|_| -> Error { "Unauthorized 2".into() })?;

        if account.is_admin {
            return Ok(());
        } else {
            return Err("Unauthorized 1".into());
        }
    }
}
