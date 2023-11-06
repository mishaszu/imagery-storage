use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::{account, users};

use super::{ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = account)]
pub struct Account {
    pub id: Uuid,
    pub referral_id: Option<Uuid>,
    pub email: String,
    pub kind: String,
    pub followee_id: Option<Uuid>,
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
    pub referral_id: Option<Uuid>,
    pub email: String,
    pub kind: Option<String>,
    pub followee_id: Option<Uuid>,
    pub public_lvl: Option<i32>,
}

#[derive(AsChangeset, Insertable, Default)]
#[diesel(table_name = account)]
pub struct AccountForUpdate {
    pub referral_id: Option<Uuid>,
    pub email: Option<String>,
    pub kind: Option<String>,
    pub followee_id: Option<Uuid>,
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
