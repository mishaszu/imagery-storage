use diesel::prelude::*;
use diesel::{query_builder::AsChangeset, Insertable, Queryable, RunQueryDsl};
use serde::Deserialize;
use uuid::Uuid;

use super::{
    error::{Error, Result},
    ModelManager,
};
use crate::crypt::token::Token;
use crate::schema::users;

#[allow(dead_code)]
#[derive(Queryable, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    email: String,
    pub nick: String,
    hash: String,
    access_key: Option<String>,
    picture: Option<String>,
    pub account_id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserForCreate {
    email: String,
    nick: String,
    hash: String,
    account_id: Uuid,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserForUpdate {
    email: Option<String>,
    nick: Option<String>,
    hash: Option<String>,
    access_key: Option<String>,
    picture: Option<String>,
    updated_at: chrono::DateTime<chrono::Utc>,
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

    pub fn get(mm: &ModelManager, user_id: &Uuid) -> Result<User> {
        let mut connection = mm.conn()?;
        users::dsl::users
            .filter(users::dsl::id.eq(user_id))
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

    pub fn list(mm: &ModelManager) -> Result<Vec<User>> {
        let mut connection = mm.conn()?;
        users::dsl::users
            .load::<User>(&mut connection)
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
