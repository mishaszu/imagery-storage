use async_graphql::{Context, Object, Result};

use crate::graphql::guard::{Role, RoleGuard};
use crate::model::account::AccountBmc;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::Account;

#[derive(Default)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn account(&self, ctx: &Context<'_>, id: Id) -> Result<Account> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let account =
            AccountBmc::get(mm, &id.into()).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(account.into())
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn accounts(&self, ctx: &Context<'_>) -> Result<Vec<Account>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let accounts = AccountBmc::list(mm).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(accounts.into_iter().map(|r| r.into()).collect())
    }
}
