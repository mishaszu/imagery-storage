use async_graphql::{ComplexObject, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{graphql::scalars::Id, model::tag::TagBmc};

#[derive(Debug, Clone, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct Tag {
    id: Id,
    name: String,
    created_at: String,
    updated_at: String,
}

#[ComplexObject]
impl Tag {
    // async fn images(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<Image>> {
    //     let mm = ctx.data_opt::<crate::model::ModelManager>();
    //     let mm = match mm {
    //         Some(mm) => mm,
    //         None => {
    //             return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
    //         }
    //     };
    //     let images = TagBmc::get_images(mm, self.id.0)
    //         .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
    //     Ok(images.into_iter().map(|r| r.into()).collect())
    // }
}

impl From<crate::model::tag::Tag> for Tag {
    fn from(tag: crate::model::tag::Tag) -> Self {
        Tag {
            id: tag.id.into(),
            name: tag.name,
            created_at: tag.created_at.to_string(),
            updated_at: tag.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct TagForCreate {
    pub name: String,
}

impl Into<crate::model::tag::TagForCreate> for TagForCreate {
    fn into(self: Self) -> crate::model::tag::TagForCreate {
        crate::model::tag::TagForCreate {
            id: Uuid::new_v4(),
            name: self.name,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct TagForUpdate {
    pub name: Option<String>,
    pub updated_at: String,
}

impl Into<crate::model::tag::TagForUpdate> for TagForUpdate {
    fn into(self: Self) -> crate::model::tag::TagForUpdate {
        crate::model::tag::TagForUpdate {
            name: self.name,
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}
