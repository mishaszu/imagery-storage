use async_graphql::{Context, Object, Result};

use crate::graphql::guard::{Role, RoleGuard};
use crate::model::account::AccountBmc;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{
    graphql::scalars::{DateTime, Id},
    model::ModelManager,
};

use super::model::{Account, AccountForCreate, AccountForUpdate};

#[derive(Default)]
pub struct AccountMutation;

#[Object]
impl AccountMutation {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn create_account(&self, ctx: &Context<'_>, input: AccountForCreate) -> Result<Account> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let account = AccountBmc::create(mm, input.into()).map_err(GraphQLError::ModelError)?;
        Ok(account.into())
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn create_account_with_referral(
        &self,
        ctx: &Context<'_>,
        referrer_id: Id,
        input: AccountForCreate,
    ) -> Result<Account> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let account = AccountBmc::create_with_referral(mm, &referrer_id.into(), None, input.into())
            .map_err(GraphQLError::ModelError)?;
        Ok(account.into())
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn update_account(
        &self,
        ctx: &Context<'_>,
        id: Id,
        input: AccountForUpdate,
    ) -> Result<Account> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let account =
            AccountBmc::update(mm, &id.into(), input.into()).map_err(GraphQLError::ModelError)?;
        Ok(account.into())
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn delete_account(&self, ctx: &Context<'_>, id: Id) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        AccountBmc::delete(mm, &id.into()).map_err(GraphQLError::ModelError)?;
        Ok("Account deleted".to_string())
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn add_referral(
        &self,
        ctx: &Context<'_>,
        referrer_id: Id,
        user_id: Id,
        exp: Option<DateTime>,
    ) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        AccountBmc::add_referral(
            mm,
            &referrer_id.into(),
            &user_id.into(),
            exp.map(|x| x.into()),
        )
        .map_err(GraphQLError::ModelError)?;
        Ok("Referral added".to_string())
    }
}
