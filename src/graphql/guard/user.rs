use async_graphql::{Context, Error, Guard, Result};
use uuid::Uuid;

use crate::{
    ctx::Ctx,
    graphql::scalars::Id,
    model::{account::AccountBmc, ModelManager},
};

use super::Accessship;

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

        let user_account_id = app_ctx.map(|r| r.account_id);

        let has_access = AccountBmc::has_access(mm, user_account_id, &self.id.into())
            .map_err(|_| -> Error { "Unauthorized".into() })?;

        match has_access {
            Accessship::None => Err("Unauthorized".into()),
            _ => Ok(()),
        }
    }
}
