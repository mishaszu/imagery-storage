use std::cmp::Ordering;

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
            Self::None => Err(crate::model::Error::AccessDeniedReturnNoInfo),
        }
    }
}

impl From<i32> for Accesship {
    fn from(i: i32) -> Self {
        match i {
            2 => Self::AllowedPublic,
            1 => Self::AllowedSubscriber,
            0 => Self::Owner,
            _ => Self::None,
        }
    }
}

impl PartialOrd for Accesship {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_i: i32 = self.clone().try_into().unwrap_or(3);
        let other_i: i32 = other.clone().try_into().unwrap_or(3);
        self_i.partial_cmp(&other_i)
    }
}

/// ResourceAccess is a trait that defines how to check access to a resource.
/// Filter value is meant to be used to resource like post that can have either user access or album access
/// ExtraSearch is meant to be used to search for a resource that is not the target resource, like searching for a post by user id
pub trait ResourceAccess {
    type ExtraSearch;
    type Resource;

    fn has_access(
        mm: &crate::model::ModelManager,
        target_resource_id: &Uuid,
        seeker_user_id: Option<Uuid>,
    ) -> crate::model::Result<(Accesship, Option<Self::Resource>)>;

    fn get_with_access(
        mm: &crate::model::ModelManager,
        target_resource_id: &Uuid,
        access: Accesship,
    ) -> crate::model::Result<Option<Self::Resource>>;

    fn has_access_list(
        mm: &crate::model::ModelManager,
        seeker_user_id: Option<Uuid>,
        extra_search_param: Self::ExtraSearch,
    ) -> crate::model::Result<Vec<(Accesship, Option<Self::Resource>)>>;

    /// if access is already stored in the resource, use this function
    fn list_with_access(
        mm: &crate::model::ModelManager,
        access: Accesship,
        extra_search_param: Self::ExtraSearch,
    ) -> crate::model::Result<Vec<Option<Self::Resource>>>;
}
