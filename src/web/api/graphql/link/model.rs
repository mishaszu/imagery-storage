use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::graphql::scalars::Id;

#[derive(Debug, Clone, Serialize, SimpleObject)]
pub struct LinkOutput {
    id: Id,
    title: String,
    url: String,
    created_at: String,
    updated_at: String,
}

impl From<crate::model::link::Link> for LinkOutput {
    fn from(link: crate::model::link::Link) -> Self {
        Self {
            id: Id(link.id),
            title: link.title,
            url: link.url,
            created_at: link.created_at.to_string(),
            updated_at: link.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct LinkForCreate {
    title: String,
    url: String,
}

impl Into<crate::model::link::LinkForCreate> for LinkForCreate {
    fn into(self) -> crate::model::link::LinkForCreate {
        crate::model::link::LinkForCreate {
            id: uuid::Uuid::new_v4(),
            title: self.title,
            url: self.url,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct LinkForUpdate {
    title: Option<String>,
    url: Option<String>,
}

impl Into<crate::model::link::LinkForUpdate> for LinkForUpdate {
    fn into(self) -> crate::model::link::LinkForUpdate {
        crate::model::link::LinkForUpdate {
            title: self.title,
            url: self.url,
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
