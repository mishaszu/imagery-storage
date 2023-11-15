use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, Enum)]
pub enum Accesship {
    AllowedPublic,
    AllowedSubscriber,
    Admin,
    Owner,
    None,
}

impl TryInto<i32> for Accesship {
    type Error = crate::model::Error;

    fn try_into(self) -> async_graphql::Result<i32, Self::Error> {
        match self {
            Self::AllowedPublic => Ok(2),
            Self::AllowedSubscriber => Ok(1),
            Self::Admin => Ok(0),
            Self::Owner => Ok(0),
            Self::None => Err(crate::model::Error::AccessDenied),
        }
    }
}

pub trait ResourceAccess {
    type Resource;
    fn has_access(
        mm: &crate::model::ModelManager,
        target_resource_id: &Uuid,
        seeker_user_id: Option<Uuid>,
    ) -> crate::model::Result<(Accesship, Option<Self::Resource>)>;

    fn has_access_list(
        mm: &crate::model::ModelManager,
        seeker_user_id: Option<Uuid>,
    ) -> crate::model::Result<Vec<(Accesship, Option<Self::Resource>)>>;
}
