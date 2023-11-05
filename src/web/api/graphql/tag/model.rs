use async_graphql::{ComplexObject, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{graphql::scalars::Id, model::tag::TagBmc, web::api::graphql::kitty::model::Kitty};

#[derive(Debug, Clone, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct Tag {
    pub id: Id,
    pub name: String,
    pub tag_category_id: Id,
    pub created_at: String,
    pub updated_at: String,
}

#[ComplexObject]
impl Tag {
    async fn images(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<crate::web::api::graphql::image::model::Image>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let images = TagBmc::get_images(mm, self.id.0)
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(images.into_iter().map(|r| r.into()).collect())
    }

    async fn kitties(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<Kitty>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let kitties = TagBmc::get_kitties(mm, self.id.into())
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(kitties.into_iter().map(|r| r.into()).collect())
    }

    async fn albums(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<crate::web::api::graphql::album::model::Album>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(crate::web::api::graphql::error::Error::ModalManagerNotInContext.into())
            }
        };
        let albums = TagBmc::get_albums(mm, self.id.into())
            .map_err(crate::web::api::graphql::error::Error::from_model_to_graphql)?;
        Ok(albums.into_iter().map(|r| r.into()).collect())
    }
}

impl From<crate::model::tag::Tag> for Tag {
    fn from(tag: crate::model::tag::Tag) -> Self {
        Self {
            id: Id(tag.id),
            name: tag.name,
            tag_category_id: Id(tag.tag_category_id),
            created_at: tag.created_at.to_string(),
            updated_at: tag.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct TagForCreate {
    pub name: String,
    pub tag_category_id: Id,
}

impl Into<crate::model::tag::TagForCreate> for TagForCreate {
    fn into(self: Self) -> crate::model::tag::TagForCreate {
        crate::model::tag::TagForCreate {
            id: uuid::Uuid::new_v4(),
            name: self.name,
            tag_category_id: self.tag_category_id.0,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct TagForUpdate {
    pub name: Option<String>,
    pub tag_category_id: Option<Id>,
}

impl Into<crate::model::tag::TagForUpdate> for TagForUpdate {
    fn into(self: Self) -> crate::model::tag::TagForUpdate {
        crate::model::tag::TagForUpdate {
            name: self.name,
            tag_category_id: self.tag_category_id.map(|id| id.0),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}
