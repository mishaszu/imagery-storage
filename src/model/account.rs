use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::{account, users};

use super::{ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = account)]
pub struct Account {
    pub id: Uuid,
    referral_id: Option<Uuid>,
    email: String,
    pub kind: String,
    is_admin: bool,
    is_public: bool,
    is_active: bool,
    is_banned: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = account)]
pub struct AccountForCreate {
    id: Uuid,
    referral_id: Option<Uuid>,
    email: String,
    kind: Option<String>,
    is_public: bool,
    is_active: bool,
}

#[derive(AsChangeset, Insertable, Default)]
#[diesel(table_name = account)]
pub struct AccountForUpdate {
    referral_id: Option<Uuid>,
    email: Option<String>,
    kind: Option<String>,
    is_admin: Option<bool>,
    is_public: Option<bool>,
    is_active: Option<bool>,
    is_banned: Option<bool>,
    updated_at: chrono::NaiveDateTime,
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

    pub fn get(mm: &ModelManager, id: &Uuid) -> Result<Account> {
        let mut connection = mm.conn()?;

        account::dsl::account
            .filter(account::dsl::id.eq(id))
            .first(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_user(mm: &ModelManager, id: &Uuid) -> Result<Account> {
        let mut connection = mm.conn()?;

        users::dsl::users
            .inner_join(account::dsl::account)
            .filter(users::dsl::id.eq(id))
            .select(account::all_columns)
            .first(&mut connection)
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
