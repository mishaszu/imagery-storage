use diesel::prelude::*;
use diesel::{
    query_builder::AsChangeset, ExpressionMethods, Identifiable, Insertable, Queryable, RunQueryDsl,
};
use uuid::Uuid;

use crate::schema::referral;

use super::{ModelManager, Result};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = referral)]
pub struct Referral {
    pub id: Uuid,
    pub referrer_id: Uuid,
    pub user_id: Uuid,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = referral)]
pub struct ReferralForCreate {
    pub id: Uuid,
    pub referrer_id: Uuid,
    pub user_id: Uuid,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ReferralForCreate {
    pub fn new(
        referrer_id: &Uuid,
        user_id: &Uuid,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            referrer_id: referrer_id.clone(),
            user_id: user_id.clone(),
            expires_at,
        }
    }
}

#[derive(AsChangeset, Insertable, Default)]
#[diesel(table_name = referral)]
pub struct ReferralForUpdate {
    pub referrer_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct ReferralBmc;

impl ReferralBmc {
    pub fn create(mm: &ModelManager, new_referral: ReferralForCreate) -> Result<Referral> {
        let mut connection = mm.conn()?;

        diesel::insert_into(referral::dsl::referral)
            .values(new_referral)
            .get_result(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get(mm: &ModelManager, referral_id: &Uuid) -> Result<Referral> {
        let mut connection = mm.conn()?;
        referral::dsl::referral
            .filter(referral::dsl::id.eq(referral_id))
            .get_result(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_by_referrer_id(mm: &ModelManager, referrer_id: &Uuid) -> Result<Vec<Referral>> {
        let mut connection = mm.conn()?;
        referral::dsl::referral
            .filter(referral::dsl::referrer_id.eq(referrer_id))
            .get_results(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn get_by_user_id(mm: &ModelManager, user_id: &Uuid) -> Result<Vec<Referral>> {
        let mut connection = mm.conn()?;
        referral::dsl::referral
            .filter(referral::dsl::user_id.eq(user_id))
            .get_results(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn update(
        mm: &ModelManager,
        referral_id: &Uuid,
        new_referral: ReferralForUpdate,
    ) -> Result<Referral> {
        let mut connection = mm.conn()?;
        diesel::update(referral::dsl::referral)
            .filter(referral::dsl::id.eq(referral_id))
            .set(new_referral)
            .get_result(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn delete(mm: &ModelManager, referral_id: &Uuid) -> Result<usize> {
        let mut connection = mm.conn()?;
        diesel::delete(referral::dsl::referral)
            .filter(referral::dsl::id.eq(referral_id))
            .execute(&mut connection)
            .map_err(|e| e.into())
    }

    pub fn is_expired(mm: &ModelManager, user_id: &Uuid, referrer_id: &Uuid) -> Result<bool> {
        let mut connection = mm.conn()?;
        let now = chrono::Utc::now();
        let result = referral::dsl::referral
            .filter(referral::dsl::user_id.eq(user_id))
            .filter(referral::dsl::referrer_id.eq(referrer_id))
            .filter(referral::dsl::expires_at.lt(now))
            .get_result::<Referral>(&mut connection);
        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
