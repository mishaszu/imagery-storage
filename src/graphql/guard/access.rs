use async_graphql::{Context, Error, Guard, Result};
use tracing::debug;
use uuid::Uuid;

use crate::web::graphql::error::Error as GraphQLError;
use crate::{ctx::Ctx, graphql::scalars::Id, model::ModelManager};

#[derive(Eq, PartialEq, Debug)]
pub enum Accessship {
    AllowedPublic,
    AllowedSubscriber,
    Admin,
    Owner,
    None,
}

impl TryInto<i32> for Accessship {
    type Error = crate::model::Error;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Self::AllowedPublic => Ok(2),
            Self::AllowedSubscriber => Ok(1),
            Self::Admin => Ok(0),
            Self::Owner => Ok(0),
            Self::None => Err(crate::model::Error::AccessDenied),
        }
    }
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
            None => {
                debug!("{:<12} - No User Logged in", "CreatorGuard");
                return Err("Not Found".into());
            }
        };

        let user_account_id = match app_ctx {
            Some(ctx) => ctx.account_id,
            None => return Err(GraphQLError::AuthError.into()),
        };

        let post = crate::model::post::PostBmc::get(mm, &self.post_id)
            .map_err(|_| -> Error { "Unauthorized".into() })?;

        let has_access = post
            .user_access(mm, &user_account_id)
            .map_err(|_| -> Error { "Unauthorized".into() })?;

        match (has_access, self.admin_allowed) {
            (Accessship::Admin, true) | (Accessship::Owner, _) => Ok(()),
            _ => Err("Unauthorized".into()),
        }
    }
}
