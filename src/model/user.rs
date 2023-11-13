use diesel::prelude::*;
use diesel::{query_builder::AsChangeset, Insertable, Queryable, RunQueryDsl};
use serde::Deserialize;
use uuid::Uuid;

use super::{
    error::{Error, Result},
    ModelManager,
};
use crate::crypt::token::Token;
use crate::schema::{account, referral, user_picture, users};

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

    pub fn get(mm: &ModelManager, user_id: &Uuid) -> Result<User> {
        let mut connection = mm.conn()?;
        users::dsl::users
            .filter(users::dsl::id.eq(user_id))
            .first::<User>(&mut connection)
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

    pub fn list(mm: &ModelManager, user_account_id: Option<Uuid>) -> Result<Vec<User>> {
        let mut connection = mm.conn()?;

        match user_account_id {
            Some(account_id) => {
                let account = account::dsl::account
                    .filter(account::dsl::id.eq(account_id))
                    .first::<crate::model::account::Account>(&mut connection)?;
                if account.is_banned {
                    return Ok(vec![]);
                }

                if account.is_admin {
                    users::dsl::users
                        .load::<User>(&mut connection)
                        .map_err(|e| e.into())
                } else {
                    let mut public: Vec<User> = account::dsl::account
                        .filter(account::dsl::public_lvl.eq(2))
                        .filter(account::dsl::is_banned.eq(false))
                        .inner_join(
                            users::dsl::users.on(users::dsl::account_id.eq(account::dsl::id)),
                        )
                        .select(users::all_columns)
                        .load::<User>(&mut connection)?;
                    let mut users_with_referral: Vec<User> = referral::dsl::referral
                        .filter(referral::dsl::user_id.eq(account_id))
                        .filter(
                            referral::dsl::expires_at
                                .is_null()
                                .or(referral::dsl::expires_at.gt(Some(chrono::Utc::now()))),
                        )
                        .inner_join(
                            account::dsl::account
                                .on(account::dsl::id.eq(referral::dsl::referrer_id)),
                        )
                        .filter(account::dsl::public_lvl.ge(1))
                        .filter(account::dsl::is_banned.eq(false))
                        .inner_join(
                            users::dsl::users.on(users::dsl::account_id.eq(account::dsl::id)),
                        )
                        .select(users::all_columns)
                        .load::<User>(&mut connection)?;

                    public.append(&mut users_with_referral);
                    public.sort_by(|a, b| a.id.cmp(&b.id));
                    public.dedup_by(|a, b| a.id == b.id);
                    public.sort_by(|a, b| a.nick.cmp(&b.nick));

                    Ok(public)
                }
            }
            None => users::dsl::users
                .inner_join(account::dsl::account)
                .filter(account::dsl::public_lvl.eq(2))
                .filter(account::dsl::is_banned.eq(false))
                .select(users::all_columns)
                .load::<User>(&mut connection)
                .map_err(|e| e.into()),
        }
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
