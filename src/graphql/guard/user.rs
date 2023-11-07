use async_graphql::{
    Context, Enum, Error, Guard, InputValueError, InputValueResult, Result, Scalar, ScalarType,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
            None => return Err("Unauthorized".into()),
        };

        // check if user is logged in
        let (user_id, user_account_id) = match app_ctx {
            Some(ctx) => (ctx.user_id, ctx.account_id),
            None => return Err("Unauthorized".into()),
        };

        let has_access =
            AccountBmc::has_user_access(mm, &user_id, &user_account_id, &self.id.into())
                .map_err(|_| -> Error { "Unauthorized".into() })?;

        if has_access {
            return Ok(());
        } else {
            return Err("Unauthorized".into());
        }
    }
}
