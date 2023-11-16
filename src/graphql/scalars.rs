use async_graphql::{Enum, InputValueError, Scalar, ScalarType};
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

impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.to_string()
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

#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, Enum)]
pub enum PublicLvl {
    // 0: restricted
    Private,
    // 1: followers
    Subscribers,
    // 2: public
    Public,
}

impl Into<i32> for PublicLvl {
    fn into(self) -> i32 {
        match self {
            PublicLvl::Private => 0,
            PublicLvl::Subscribers => 1,
            PublicLvl::Public => 2,
        }
    }
}

impl From<i32> for PublicLvl {
    fn from(i: i32) -> Self {
        match i {
            0 => PublicLvl::Private,
            1 => PublicLvl::Subscribers,
            2 => PublicLvl::Public,
            _ => PublicLvl::Private,
        }
    }
}
