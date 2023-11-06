use async_graphql::{InputValueError, Scalar, ScalarType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Id(pub uuid::Uuid);

#[Scalar]
impl ScalarType for Id {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::String(s) => {
                let uuid = uuid::Uuid::parse_str(&s)?;
                Ok(Id(uuid))
            }
            _ => Err(InputValueError::custom("uuid should be string")),
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.to_string())
    }
}

impl From<uuid::Uuid> for Id {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl Into<uuid::Uuid> for Id {
    fn into(self) -> uuid::Uuid {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DateTime(pub chrono::DateTime<chrono::Utc>);

#[Scalar]
impl ScalarType for DateTime {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::String(s) => {
                let dt = chrono::DateTime::parse_from_rfc3339(&s)?;
                Ok(DateTime(dt.into()))
            }
            _ => Err(InputValueError::custom("datetime should be string")),
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.to_rfc3339())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for DateTime {
    fn from(dt: chrono::DateTime<chrono::Utc>) -> Self {
        Self(dt)
    }
}

impl Into<chrono::DateTime<chrono::Utc>> for DateTime {
    fn into(self) -> chrono::DateTime<chrono::Utc> {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PublicLvl {
    // 0: private
    Private,
    // 1: restricted
    Restricted,
    // 2: followers
    Followers,
    // 3: public
    Public,
}

#[Scalar]
impl ScalarType for PublicLvl {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::String(i) => match i.as_str() {
                "private" => Ok(PublicLvl::Private),
                "restricted" => Ok(PublicLvl::Restricted),
                "followers" => Ok(PublicLvl::Followers),
                "public" => Ok(PublicLvl::Public),
                _ => Err(InputValueError::custom(
                    "public_lvl should: private, restricted, followers, public",
                )),
            },
            _ => Err(InputValueError::custom("public_lvl should be string")),
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        match self {
            PublicLvl::Private => async_graphql::Value::String("private".to_string()),
            PublicLvl::Restricted => async_graphql::Value::String("restricted".to_string()),
            PublicLvl::Followers => async_graphql::Value::String("followers".to_string()),
            PublicLvl::Public => async_graphql::Value::String("public".to_string()),
        }
    }
}

impl Into<i32> for PublicLvl {
    fn into(self) -> i32 {
        match self {
            PublicLvl::Private => 0,
            PublicLvl::Restricted => 1,
            PublicLvl::Followers => 2,
            PublicLvl::Public => 3,
        }
    }
}

impl From<i32> for PublicLvl {
    fn from(i: i32) -> Self {
        match i {
            0 => PublicLvl::Private,
            1 => PublicLvl::Restricted,
            2 => PublicLvl::Followers,
            3 => PublicLvl::Public,
            _ => PublicLvl::Private,
        }
    }
}
