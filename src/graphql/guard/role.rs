use async_graphql::{Context, Enum, Error, Guard, Result};
use serde::{Deserialize, Serialize};

use crate::{
    ctx::Ctx,
    model::{account::AccountBmc, ModelManager},
};

#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, Enum)]
pub enum Role {
    Admin,
    Creator,
    Commenter,
    Guest,
    None,
}

pub struct RoleGuard {
    pub role: Role,
}

impl RoleGuard {
    pub fn new(role: Role) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let app_ctx = ctx.data_opt::<Ctx>();
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err("Unauthorized".into()),
        };

        let user = match app_ctx {
            Some(ctx) => ctx.account_id,
            None => match self.role {
                Role::Guest => return Ok(()),
                _ => return Err("Unauthorized 1".into()),
            },
        };
        let account = AccountBmc::get(mm, &user).map_err(|_| -> Error { "Unauthorized".into() })?;

        if account.is_admin && self.role == Role::Admin {
            return Ok(());
        }

        if account.is_admin == false && self.role == Role::Admin {
            return Err("Unauthorized".into());
        }

        let kind = account.kind.as_str();

        match (kind, self.role) {
            // when account is creator
            ("creator", Role::Creator)
            | ("creator", Role::Commenter)
            | ("creator", Role::Guest) => Ok(()),
            ("commenter", Role::Commenter) | ("commenter", Role::Guest) => Ok(()),
            ("guest", Role::Guest) => Ok(()),
            _ => Err("Unauthorized 1".into()),
        }
    }
}

impl Into<String> for Role {
    fn into(self) -> String {
        match self {
            Role::Admin => "admin".to_string(),
            Role::Creator => "creator".to_string(),
            Role::Commenter => "commenter".to_string(),
            Role::Guest => "guest".to_string(),
            Role::None => "none".to_string(),
        }
    }
}

impl From<String> for Role {
    fn from(s: String) -> Self {
        match s.as_str() {
            "admin" => Role::Admin,
            "creator" => Role::Creator,
            "commenter" => Role::Commenter,
            "guest" => Role::Guest,
            _ => Role::None,
        }
    }
}
