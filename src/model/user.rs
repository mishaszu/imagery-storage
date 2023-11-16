use diesel::prelude::*;
use diesel::{query_builder::AsChangeset, Insertable, Queryable, RunQueryDsl};
use serde::Deserialize;
use uuid::Uuid;

use super::account::{Account, AccountBmc};
use super::{
    error::{Error, Result},
    ModelManager,
};
use crate::access::{Accesship, ResourceAccess};
use crate::crypt::token::Token;
use crate::schema::{account, user_picture, users};

#[allow(dead_code)]
#[derive(Queryable, Deserialize, Debug, Eq, PartialEq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub nick: String,
    pub hash: String,
    pub account_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserForCreate {
    pub id: Uuid,
    pub email: String,
    pub nick: String,
    pub hash: String,
    pub account_id: Uuid,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserForUpdate {
    pub email: Option<String>,
    pub nick: Option<String>,
    pub hash: Option<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct UserBmc {}

impl UserBmc {
    pub fn create(mm: &ModelManager, new_user: UserForCreate) -> Result<User> {
        let mut connection = mm.conn()?;
        diesel::insert_into(users::dsl::users)
            .values(&new_user)
            .get_result::<User>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get(mm: &ModelManager, user_id: &Uuid) -> Result<(User, Account)> {
        let mut connection = mm.conn()?;
        users::dsl::users
            .filter(users::dsl::id.eq(user_id))
            .inner_join(account::dsl::account)
            .select((users::all_columns, account::all_columns))
            .first::<(User, Account)>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_by_account_id(mm: &ModelManager, account_id: &Uuid) -> Result<User> {
        let mut connection = mm.conn()?;
        users::dsl::users
            .filter(users::dsl::account_id.eq(account_id))
            .first::<User>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_by_nick(mm: &ModelManager, user_nick: &str) -> Result<User> {
        let mut connection = mm.conn()?;
        users::dsl::users
            .filter(users::dsl::nick.eq(user_nick))
            .first::<User>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_by_email(mm: &ModelManager, user_email: &str) -> Result<User> {
        let mut connection = mm.conn()?;
        users::dsl::users
            .filter(users::dsl::email.eq(user_email))
            .first::<User>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn list(mm: &ModelManager) -> Result<Vec<(User, Account)>> {
        let mut connection = mm.conn()?;

        users::dsl::users
            .inner_join(account::dsl::account)
            .select((users::all_columns, account::all_columns))
            .load::<(User, Account)>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn update(mm: &ModelManager, user_id: &Uuid, new_user: UserForUpdate) -> Result<User> {
        let mut connection = mm.conn()?;
        diesel::update(users::dsl::users)
            .filter(users::dsl::id.eq(user_id))
            .set(&new_user)
            .get_result::<User>(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn delete(mm: &ModelManager, user_id: &Uuid) -> Result<()> {
        let mut connection = mm.conn()?;
        diesel::delete(users::dsl::users)
            .filter(users::dsl::id.eq(user_id))
            .execute(&mut connection)
            .map_err(|e| -> Error { e.into() })?;
        Ok(())
    }

    pub fn add_picture(mm: &ModelManager, user_id: &Uuid, picture_id: &Uuid) -> Result<()> {
        let mut connection = mm.conn()?;
        diesel::insert_into(user_picture::dsl::user_picture)
            .values((
                user_picture::dsl::user_id.eq(user_id),
                user_picture::dsl::image_id.eq(picture_id),
            ))
            .execute(&mut connection)
            .map_err(|e| -> Error { e.into() })?;
        Ok(())
    }

    pub fn delete_picture(mm: &ModelManager, user_id: &Uuid, picture_id: &Uuid) -> Result<()> {
        let mut connection = mm.conn()?;
        diesel::delete(
            user_picture::dsl::user_picture
                .filter(user_picture::dsl::user_id.eq(user_id))
                .filter(user_picture::dsl::image_id.eq(picture_id)),
        )
        .execute(&mut connection)
        .map_err(|e| -> Error { e.into() })?;
        Ok(())
    }
}

impl User {
    pub fn validate_pwd(&self, pwd: &str) -> crate::crypt::Result<()> {
        crate::crypt::pass::validate_pwd(pwd, &self.hash)
    }
}

impl User {
    pub fn into_token(self) -> crate::crypt::Result<Token> {
        Token::new(&self.id, &self.nick)
    }
}

impl ResourceAccess for UserBmc {
    type Resource = User;
    type ExtraSearch = ();

    fn has_access(
        mm: &crate::model::ModelManager,
        // user id
        target_resource_id: &Uuid,
        seeker_user_id: Option<Uuid>,
    ) -> crate::model::Result<(crate::access::Accesship, Option<Self::Resource>)> {
        let seeker_account = seeker_user_id.and_then(|id| AccountBmc::get_by_user_id(mm, &id).ok());
        let (target_user, target_account) = Self::get(mm, target_resource_id)?;

        let access = target_account.compare_access(mm, seeker_account);

        match access {
            Accesship::None => Err(Error::AccessDeniedReturnNoInfo),
            _ => Ok((access, Some(target_user))),
        }
    }

    fn get_with_access(
        mm: &crate::model::ModelManager,
        target_resource_id: &Uuid,
        access: Accesship,
    ) -> crate::model::Result<Option<Self::Resource>> {
        let (target_user, _) = Self::get(mm, target_resource_id)?;

        match access {
            Accesship::None => Err(Error::AccessDeniedReturnNoInfo),
            _ => Ok(Some(target_user)),
        }
    }

    fn has_access_list(
        mm: &crate::model::ModelManager,
        seeker_user_id: Option<Uuid>,
        _extra_search_param: Self::ExtraSearch,
    ) -> crate::model::Result<Vec<(crate::access::Accesship, Option<Self::Resource>)>> {
        let users = Self::list(mm)?;

        let seeker_account = seeker_user_id.and_then(|id| AccountBmc::get_by_user_id(mm, &id).ok());

        let filtered_users = users
            .into_iter()
            .filter(|(user, account)| {
                let access = account.compare_access(mm, seeker_account);
                match access {
                    Accesship::None => false,
                    _ => true,
                }
            })
            .map(|(user, account)| (account.compare_access(mm, seeker_account), Some(user)))
            .collect();

        Ok(filtered_users)
    }

    fn list_with_access(
        mm: &crate::model::ModelManager,
        access: Accesship,
        extra_search_param: Self::ExtraSearch,
    ) -> crate::model::Result<Vec<Option<Self::Resource>>> {
        let users = Self::list(mm)?;

        let filtered_users = users
            .into_iter()
            .filter(|(user, account)| match access {
                Accesship::None => false,
                _ => true,
            })
            .map(|(user, _)| Some(user))
            .collect();

        Ok(filtered_users)
    }
}
