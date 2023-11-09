use async_graphql::{Context, Error, Guard, Result};
use uuid::Uuid;

use crate::{
    ctx::Ctx,
    graphql::scalars::Id,
    model::{account::AccountBmc, ModelManager},
};

#[derive(Eq, PartialEq)]
pub enum Accessship {
    AllowedPublic,
    AllowedSubscriber,
    Admin,
    Owner,
    None,
}

pub struct CreatorGuard {
    post_id: Uuid,
    admin_allowed: bool,
}

impl CreatorGuard {
    pub fn new(id: Id, admin_allowed: bool) -> Self {
        Self {
            post_id: id.into(),
            admin_allowed,
        }
    }
}

#[async_trait::async_trait]
impl Guard for CreatorGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let app_ctx = ctx.data_opt::<Ctx>();
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err("Unauthorized".into()),
        };

        let user_account_id = app_ctx.map(|r| r.account_id);

        let (_, target_account) = crate::model::post::PostBmc::get_with_account(mm, &self.post_id)
            .map_err(|_| -> Error { "Unauthorized".into() })?;

        let has_access = AccountBmc::has_access(mm, user_account_id, &target_account.id)
            .map_err(|_| -> Error { "Unauthorized".into() })?;

        match (has_access, self.admin_allowed) {
            (Accessship::AllowedSubscriber, false)
            | (Accessship::AllowedPublic, false)
            | (Accessship::Admin, true)
            | (Accessship::Owner, true) => Ok(()),
            _ => Err("Unauthorized".into()),
        }
    }
}
