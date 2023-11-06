use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::guard::{Role, RoleGuard};
use crate::graphql::scalars::{DateTime, Id, PublicLvl};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(guard = "RoleGuard::new(Role::Admin)", complex)]
pub struct Account {
    pub id: Id,
    pub referral_id: Option<Id>,
    pub email: String,
    pub kind: Role,
    pub followee_id: Option<Id>,
    pub is_admin: bool,
    pub public_lvl: PublicLvl,
    pub is_banned: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[ComplexObject]
impl Account {
    // pub async fn user(&self, ctx: &Context<'_>) -> Result<User> {
    //     let mm = ctx.data::<ModelManager>()?;
    //     let user = UserBmc::get(mm, &self.user_id).await?;
    //     Ok(user.into())
    // }
}

impl From<crate::model::account::Account> for Account {
    fn from(account: crate::model::account::Account) -> Self {
        Self {
            id: account.id.into(),
            referral_id: account.referral_id.map(|id| id.into()),
            email: account.email,
            kind: account.kind.into(),
            followee_id: account.followee_id.map(|id| id.into()),
            is_admin: account.is_admin,
            public_lvl: account.public_lvl.into(),
            is_banned: account.is_banned,
            created_at: account.created_at.into(),
            updated_at: account.updated_at.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct AccountForCreate {
    pub referral_id: Option<Id>,
    pub email: String,
    pub kind: Option<Role>,
    pub followee_id: Option<Id>,
    pub public_lvl: Option<PublicLvl>,
}

impl Into<crate::model::account::AccountForCreate> for AccountForCreate {
    fn into(self) -> crate::model::account::AccountForCreate {
        crate::model::account::AccountForCreate {
            id: Uuid::new_v4(),
            referral_id: self.referral_id.map(|id| id.into()),
            email: self.email,
            kind: self.kind.map(|r| r.into()),
            followee_id: self.followee_id.map(|id| id.into()),
            public_lvl: self.public_lvl.map(|pl| pl.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct AccountForUpdate {
    pub referral_id: Option<Id>,
    pub email: Option<String>,
    pub kind: Option<Role>,
    pub followee_id: Option<Id>,
    pub is_admin: Option<bool>,
    pub public_lvl: Option<PublicLvl>,
    pub is_banned: Option<bool>,
}

impl Into<crate::model::account::AccountForUpdate> for AccountForUpdate {
    fn into(self) -> crate::model::account::AccountForUpdate {
        crate::model::account::AccountForUpdate {
            referral_id: self.referral_id.map(|id| id.into()),
            email: self.email,
            kind: self.kind.map(|r| r.into()),
            followee_id: self.followee_id.map(|id| id.into()),
            is_admin: self.is_admin,
            public_lvl: self.public_lvl.map(|pl| pl.into()),
            is_banned: self.is_banned,
            updated_at: chrono::Utc::now(),
        }
    }
}
