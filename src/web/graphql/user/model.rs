use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::crypt::pass::encrypt_pwd;
use crate::graphql::guard::{Role, RoleGuard};
use crate::graphql::scalars::{DateTime, Id};
use crate::web::graphql::error::Error;

#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct User {
    pub id: Id,
    pub nick: String,
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub account_id: Id,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[ComplexObject]
impl User {}

impl From<crate::model::user::User> for User {
    fn from(user: crate::model::user::User) -> Self {
        Self {
            id: user.id.into(),
            nick: user.nick,
            account_id: user.account_id.into(),
            created_at: user.created_at.into(),
            updated_at: user.updated_at.into(),
        }
    }
}

#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
pub struct UserRestricted {
    pub id: Id,
    pub email: String,
    pub nick: String,
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub account_id: Id,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<crate::model::user::User> for UserRestricted {
    fn from(user: crate::model::user::User) -> Self {
        Self {
            id: user.id.into(),
            email: user.email,
            nick: user.nick,
            account_id: user.account_id.into(),
            created_at: user.created_at.into(),
            updated_at: user.updated_at.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct UserForCreate {
    pub email: String,
    pub nick: String,
    pub password: String,
    pub account_id: Id,
}

impl UserForCreate {
    pub fn into_db(self) -> Result<crate::model::user::UserForCreate> {
        let user = crate::model::user::UserForCreate {
            id: Uuid::new_v4(),
            email: self.email,
            nick: self.nick,
            hash: encrypt_pwd(&self.password)
                .map_err(|_| -> async_graphql::Error { Error::FailedToEncryptPassword.into() })?,
            account_id: self.account_id.into(),
        };
        Ok(user)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct UserForUpdate {
    pub email: Option<String>,
    pub nick: Option<String>,
    pub hash: Option<String>,
}

impl Into<crate::model::user::UserForUpdate> for UserForUpdate {
    fn into(self) -> crate::model::user::UserForUpdate {
        crate::model::user::UserForUpdate {
            email: self.email,
            nick: self.nick,
            hash: self.hash,
            updated_at: chrono::Utc::now(),
        }
    }
}
