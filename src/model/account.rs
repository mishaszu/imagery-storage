use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    graphql::guard::Accessship,
    model::referral::Referral,
    schema::{account, referral, users},
};

use super::{referral::ReferralForCreate, ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = account)]
pub struct Account {
    pub id: Uuid,
    pub fullname: String,
    pub email: String,
    pub kind: String,
    pub is_admin: bool,
    pub public_lvl: i32,
    pub is_banned: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = account)]
pub struct AccountForCreate {
    pub id: Uuid,
    pub fullname: String,
    pub email: String,
    pub kind: Option<String>,
    pub public_lvl: Option<i32>,
}

#[derive(AsChangeset, Insertable, Default)]
#[diesel(table_name = account)]
pub struct AccountForUpdate {
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub kind: Option<String>,
    pub is_admin: Option<bool>,
    pub public_lvl: Option<i32>,
    pub is_banned: Option<bool>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct AccountBmc;

impl AccountBmc {
    pub fn create(mm: &ModelManager, new_account: AccountForCreate) -> Result<Account> {
        let mut connection = mm.conn()?;

        diesel::insert_into(account::dsl::account)
            .values(new_account)
            .get_result(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn create_with_referral(
        mm: &ModelManager,
        referrer_id: &Uuid,
        exp: Option<chrono::DateTime<chrono::Utc>>,
        new_account: AccountForCreate,
    ) -> Result<Account> {
        let mut connection = mm.conn()?;

        connection.transaction(|connection| {
            let account = diesel::insert_into(account::dsl::account)
                .values(new_account)
                .get_result::<Account>(connection)?;

            let referral_to_create = ReferralForCreate::new(referrer_id, &account.id, exp);
            diesel::insert_into(referral::dsl::referral)
                .values(referral_to_create)
                .execute(connection)?;

            Ok(account)
        })
    }

    pub fn add_referral(
        mm: &ModelManager,
        referrer_id: &Uuid,
        user_id: &Uuid,
        exp: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Referral> {
        let mut connection = mm.conn()?;

        let referral_to_create = ReferralForCreate::new(referrer_id, user_id, exp);
        diesel::insert_into(referral::dsl::referral)
            .values(referral_to_create)
            .get_result(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get(mm: &ModelManager, id: &Uuid) -> Result<Account> {
        let mut connection = mm.conn()?;

        account::dsl::account
            .filter(account::dsl::id.eq(id))
            .first(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_referrals(mm: &ModelManager, id: &Uuid) -> Result<Vec<Account>> {
        let mut connection = mm.conn()?;

        referral::dsl::referral
            .filter(referral::dsl::referrer_id.eq(id))
            .inner_join(account::dsl::account.on(referral::dsl::user_id.eq(account::dsl::id)))
            .select(account::all_columns)
            .load(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn has_access(
        mm: &ModelManager,
        user_account_id: Option<Uuid>,
        target_user_id: &Uuid,
    ) -> Result<Accessship> {
        let mut connection = mm.conn()?;

        let target_account = users::dsl::users
            .filter(users::dsl::id.eq(target_user_id))
            .inner_join(account::dsl::account)
            .filter(account::dsl::is_banned.eq(false))
            .select(account::all_columns)
            .first::<Account>(&mut connection)?;

        let user_account_id = match user_account_id {
            Some(user_account_id) => user_account_id,
            None => return Ok(Accessship::None),
        };

        let user_account = account::dsl::account
            .filter(account::dsl::id.eq(user_account_id))
            .filter(account::dsl::is_banned.eq(false))
            .first::<Account>(&mut connection)?;

        if user_account.id == target_account.id {
            return Ok(Accessship::Owner);
        }

        if user_account.is_admin {
            return Ok(Accessship::Admin);
        }

        let has_refferal = referral::dsl::referral
            .filter(referral::dsl::user_id.eq(user_account.id))
            .filter(referral::dsl::referrer_id.eq(target_account.id))
            .first::<Referral>(&mut connection)
            .is_ok();

        match (target_account.public_lvl, has_refferal) {
            (lvl, true) if lvl > 0 => Ok(Accessship::AllowedSubscriber),
            (1, false) | (2, _) => Ok(Accessship::AllowedPublic),
            _ => Ok(Accessship::None),
        }
    }

    pub fn list(mm: &ModelManager) -> Result<Vec<Account>> {
        let mut connection = mm.conn()?;

        account::dsl::account
            .load(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn update(
        mm: &ModelManager,
        id: &Uuid,
        update_account: AccountForUpdate,
    ) -> Result<Account> {
        let mut connection = mm.conn()?;

        diesel::update(account::dsl::account.filter(account::dsl::id.eq(id)))
            .set(update_account)
            .get_result(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn delete(mm: &ModelManager, id: &Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;

        diesel::delete(account::dsl::account.filter(account::dsl::id.eq(id)))
            .execute(&mut connection)
            .map_err(|e| e.into())
    }
}
