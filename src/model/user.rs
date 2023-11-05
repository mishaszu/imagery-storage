use diesel::prelude::*;
use diesel::{query_builder::AsChangeset, Insertable, Queryable, RunQueryDsl};
use serde::Deserialize;
use uuid::Uuid;

use super::{
    error::{Error, Result},
    ModelManager,
};
use crate::crypt::token::Token;
use crate::ctx::Ctx;
use crate::schema::users;

#[allow(dead_code)]
#[derive(Queryable, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    id: Uuid,
    name: String,
    email: String,
    hash: String,
    fp: i32,
    wsic: i32,
    is_admin: bool,
    subscription: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

impl Into<Ctx> for User {
    fn into(self) -> Ctx {
        Ctx::new(self.id, &self.name, &self.subscription)
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserForCreate {
    name: String,
    email: String,
    hash: String,
    subscription: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserForUpdate {
    name: Option<String>,
    email: Option<String>,
    hash: Option<String>,
    subscription: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct AdminForChange {
    is_admin: Option<bool>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserCoutnerForChange {
    fp: Option<i32>,
    wsic: Option<i32>,
}

pub struct UserBmc {}

impl UserBmc {
    #[allow(dead_code)]
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
    pub fn get_by_name(mm: &ModelManager, user_name: &str) -> Result<User> {
        let mut connection = mm.conn()?;
        users::dsl::users
            .filter(users::dsl::name.eq(user_name))
            .first::<User>(&mut connection)
            .map_err(|e| e.into())
    }
    #[allow(dead_code)]
    pub fn lsit(mm: &ModelManager) -> Result<Vec<User>> {
        let mut connection = mm.conn()?;
        users::dsl::users
            .load::<User>(&mut connection)
            .map_err(|e| e.into())
    }
    #[allow(dead_code)]
    pub fn update(mm: &ModelManager, user_id: &Uuid, new_user: UserForUpdate) -> Result<User> {
        let mut connection = mm.conn()?;
        diesel::update(users::dsl::users)
            .filter(users::dsl::id.eq(user_id))
            .set(&new_user)
            .get_result::<User>(&mut connection)
            .map_err(|e| e.into())
    }
    #[allow(dead_code)]
    pub fn update_admin(
        mm: &ModelManager,
        user_id: &Uuid,
        new_admin: AdminForChange,
    ) -> Result<User> {
        let mut connection = mm.conn()?;
        diesel::update(users::dsl::users)
            .filter(users::dsl::id.eq(user_id))
            .set(&new_admin)
            .get_result::<User>(&mut connection)
            .map_err(|e| e.into())
    }
    #[allow(dead_code)]
    pub fn update_user_counter(
        mm: &ModelManager,
        user_id: &Uuid,
        new_user_counter: UserCoutnerForChange,
    ) -> Result<User> {
        let mut connection = mm.conn()?;
        diesel::update(users::dsl::users)
            .filter(users::dsl::id.eq(user_id))
            .set(&new_user_counter)
            .get_result::<User>(&mut connection)
            .map_err(|e| e.into())
    }
    #[allow(dead_code)]
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
        Token::new(&self.id, &self.name, &self.subscription)
    }
}
