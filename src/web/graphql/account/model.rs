use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::access::Accesship;
use crate::graphql::guard::{Role, RoleGuard};
use crate::graphql::scalars::{DateTime, Id, PublicLvl};
use crate::model::account::AccountBmc;
use crate::model::user::UserBmc;
use crate::model::ModelManager;
use crate::web::graphql::error::Error as GraphQLError;
use crate::web::graphql::user::model::User;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(guard = "RoleGuard::new(Role::Admin)", complex)]
pub struct Account {
    pub id: Id,
    pub fullname: String,
    pub email: String,
    pub kind: Role,
    pub is_admin: bool,
    pub public_lvl: PublicLvl,
    pub is_banned: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[graphql(skip)]
    pub access_lvl: Option<Accesship>,
}

#[ComplexObject]
impl Account {
    pub async fn user(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let user =
            UserBmc::get_by_account_id(mm, &self.id.into()).map_err(GraphQLError::ModelError)?;
        Ok(user.into())
    }

    pub async fn referrals(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Account>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let users =
            AccountBmc::get_referrals(mm, &self.id.into()).map_err(GraphQLError::ModelError)?;
        Ok(users.into_iter().map(|r| r.into()).collect())
    }
}

impl From<crate::model::account::Account> for Account {
    fn from(account: crate::model::account::Account) -> Self {
        Self {
            id: account.id.into(),
            fullname: account.fullname,
            email: account.email,
            kind: account.kind.into(),
            is_admin: account.is_admin,
            public_lvl: account.public_lvl.into(),
            is_banned: account.is_banned,
            created_at: account.created_at.into(),
            updated_at: account.updated_at.into(),
            access_lvl: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct AccountForCreate {
    pub fullname: String,
    pub email: String,
    pub kind: Option<Role>,
    pub public_lvl: Option<PublicLvl>,
}

impl Into<crate::model::account::AccountForCreate> for AccountForCreate {
    fn into(self) -> crate::model::account::AccountForCreate {
        crate::model::account::AccountForCreate {
            id: Uuid::new_v4(),
            fullname: self.fullname,
            email: self.email,
            kind: self.kind.map(|r| r.into()),
            public_lvl: self.public_lvl.map(|pl| pl.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct AccountForUpdate {
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub kind: Option<Role>,
    pub is_admin: Option<bool>,
    pub public_lvl: Option<PublicLvl>,
    pub is_banned: Option<bool>,
}

impl Into<crate::model::account::AccountForUpdate> for AccountForUpdate {
    fn into(self) -> crate::model::account::AccountForUpdate {
        crate::model::account::AccountForUpdate {
            fullname: self.fullname,
            email: self.email,
            kind: self.kind.map(|r| r.into()),
            is_admin: self.is_admin,
            public_lvl: self.public_lvl.map(|pl| pl.into()),
            is_banned: self.is_banned,
            updated_at: chrono::Utc::now(),
        }
    }
}
