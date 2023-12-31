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

    pub fn get_by_user_id(mm: &ModelManager, user_id: &Uuid) -> Result<Account> {
        let mut connection = mm.conn()?;

        users::dsl::users
            .filter(users::dsl::id.eq(user_id))
            .inner_join(account::dsl::account.on(users::dsl::id.eq(account::dsl::id)))
            .select(account::all_columns)
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

impl Account {
    fn compare_access(&self, mm: &ModelManager, account: Option<Account>) -> Result<Accessship> {
        let account = match (account, self.public_lvl) {
            (Some(account), _) => account,
            (None, 2) => return Ok(Accessship::AllowedPublic),
            (None, 1) => return Ok(Accessship::DisallowedSubscriber),
            _ => return Ok(Accessship::None),
        };

        if account.is_admin {
            return Ok(Accessship::Admin);
        }

        if account.is_banned {
            return Ok(Accessship::None);
        }

        if account.id == self.id {
            return Ok(Accessship::Owner);
        }

        let has_refferal = account.has_refferal(mm, self.id)?;
        match (self.public_lvl, has_refferal) {
            (2, _) => Ok(Accessship::AllowedPublic),
            (lvl, true) if lvl > 0 => Ok(Accessship::AllowedSubscriber),
            _ => Ok(Accessship::None),
        }
    }

    // check if passed user account id has access to self account
    pub fn has_access(&self, mm: &ModelManager, account_id: Option<Uuid>) -> Result<Accessship> {
        let mut connection = mm.conn()?;

        let user_account = account_id.and_then(|account_id| {
            account::dsl::account
                .filter(account::dsl::id.eq(account_id))
                .first::<Account>(&mut connection)
                .ok()
        });

        self.compare_access(mm, user_account)
    }

    // check if passed user id has access to self account
    pub fn has_user_access(&self, mm: &ModelManager, user_id: Option<Uuid>) -> Result<Accessship> {
        let user_account =
            user_id.and_then(|user_id| AccountBmc::get_by_user_id(mm, &user_id).ok());

        self.compare_access(mm, user_account)
    }

    pub fn has_refferal(&self, mm: &ModelManager, account_id: Uuid) -> Result<bool> {
        let mut connection = mm.conn()?;

        let has_refferal = referral::dsl::referral
            .filter(referral::dsl::user_id.eq(account_id))
            .filter(referral::dsl::referrer_id.eq(self.id))
            .first::<Referral>(&mut connection)
            .is_ok();

        Ok(has_refferal)
    }
}
