use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::access::Accesship;
use crate::crypt::pass::encrypt_pwd;
use crate::ctx::Ctx;
use crate::graphql::guard::{Role, RoleGuard};
use crate::graphql::scalars::{DateTime, Id};
use crate::model::account::AccountBmc;
use crate::model::post::PostBmc;
use crate::model::ModelManager;
use crate::web::graphql::account::model::Account;
use crate::web::graphql::error::Error as GraphQLError;

#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct User {
    pub id: Id,
    // TODO allow user access for owner
    /// user's email. Access only for admin
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub email: String,
    pub nick: String,
    /// user's account id. Access only for admin
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub account_id: Id,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[graphql(skip)]
    pub access_lvl: Option<Accesship>,
}

#[ComplexObject]
impl User {
    /// user's account - Access only for admin
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn account(&self, ctx: &Context<'_>) -> Result<Account> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let account =
            AccountBmc::get(mm, &self.account_id.into()).map_err(GraphQLError::ModelError)?;

        Ok(account.into())
    }

    // pub async fn posts(
    //     &self,
    //     ctx: &Context<'_>,
    // ) -> Result<Vec<Option<crate::web::graphql::post::model::Post>>> {
    //     let mm = ctx.data_opt::<ModelManager>();
    //     let mm = match mm {
    //         Some(mm) => mm,
    //         None => return Err(GraphQLError::ModalManagerNotInContext.into()),
    //     };

    //     let user_id = ctx.data_opt::<Ctx>().map(|r| r.user_id);
    //     let posts =
    //         PostBmc::list(mm, user_id, &self.id.into()).map_err(GraphQLError::ModelError)?;

    //     let posts = posts.into_iter().map(|r| r.map(|r| r.into())).collect();

    //     Ok(posts)
    // }
}

impl TryFrom<(Accesship, Option<crate::model::user::User>)> for User {
    type Error = async_graphql::Error;
    fn try_from(
        value: (Accesship, Option<crate::model::user::User>),
    ) -> std::prelude::v1::Result<Self, Self::Error> {
        let (access, user) = value;
        let user = user.ok_or_else(|| -> async_graphql::Error {
            GraphQLError::AccessError("No user found".to_string()).into()
        })?;

        Ok(Self {
            id: user.id.into(),
            email: user.email,
            nick: user.nick,
            account_id: user.account_id.into(),
            created_at: user.created_at.into(),
            updated_at: user.updated_at.into(),
            access_lvl: Some(access),
        })
    }
}

impl From<(Accesship, crate::model::user::User)> for User {
    fn from((access, user): (Accesship, crate::model::user::User)) -> Self {
        Self {
            id: user.id.into(),
            email: user.email,
            nick: user.nick,
            account_id: user.account_id.into(),
            created_at: user.created_at.into(),
            updated_at: user.updated_at.into(),
            access_lvl: Some(access),
        }
    }
}

#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
pub struct UserSelf {
    pub id: Id,
    pub email: String,
    pub nick: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[graphql(skip)]
    pub access_lvl: Option<Accesship>,
}

impl TryFrom<(Accesship, Option<crate::model::user::User>)> for UserSelf {
    type Error = async_graphql::Error;
    fn try_from(
        value: (Accesship, Option<crate::model::user::User>),
    ) -> std::prelude::v1::Result<Self, Self::Error> {
        let (access, user) = value;
        let user = user.ok_or_else(|| -> async_graphql::Error {
            GraphQLError::AccessError("No user found".to_string()).into()
        })?;

        Ok(Self {
            id: user.id.into(),
            email: user.email,
            nick: user.nick,
            created_at: user.created_at.into(),
            updated_at: user.updated_at.into(),
            access_lvl: Some(access),
        })
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
            hash: encrypt_pwd(&self.password).map_err(|_| -> async_graphql::Error {
                GraphQLError::FailedToEncryptPassword.into()
            })?,
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
