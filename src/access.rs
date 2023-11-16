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

/// ResourceAccess is a trait that defines how to check access to a resource.
/// Filter value is meant to be used to resource like post that can have either user access or album access
/// ExtraSearch is meant to be used to search for a resource that is not the target resource, like searching for a post by user id
pub trait ResourceAccess {
    type ExtraSearch;
    type Filter;
    type Resource;

    fn has_access(
        mm: &crate::model::ModelManager,
        target_resource_id: &Uuid,
        seeker_user_id: Option<Uuid>,
        filter: Self::Filter,
    ) -> crate::model::Result<(Accesship, Option<Self::Resource>)>;

    fn has_access_list(
        mm: &crate::model::ModelManager,
        seeker_user_id: Option<Uuid>,
        extra_search_params: Self::ExtraSearch,
        filter: Self::Filter,
    ) -> crate::model::Result<Vec<(Accesship, Option<Self::Resource>)>>;

    fn get_with_access(
        mm: &crate::model::ModelManager,
        target_resource_id: &Uuid,
        access: Accesship,
        filter: Self::Filter,
    ) -> crate::model::Result<Option<Self::Resource>>;

    /// if access is already stored in the resource, use this function
    fn list_with_access(
        mm: &crate::model::ModelManager,
        extra_search_params: Self::ExtraSearch,
        filter: Self::Filter,
    ) -> crate::model::Result<Vec<Option<Self::Resource>>>;
}
