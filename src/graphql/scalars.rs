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
