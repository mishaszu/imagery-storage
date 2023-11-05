use async_graphql::{ComplexObject, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::graphql::scalars::Id;
use crate::model::tag::TagBmc;
use crate::web::api::graphql::error::Error as GraphQLError;
use crate::web::api::graphql::tag::model::Tag as TagGql;

#[derive(Debug, Clone, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct TagCategory {
    pub id: Id,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[ComplexObject]
impl TagCategory {
    async fn tags(&self, ctx: &async_graphql::Context<'_>) -> Result<Vec<TagGql>> {
        let mm = ctx.data_opt::<crate::model::ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let tags = TagBmc::get_by_tag_category_id(mm, self.id.into(), None)
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(tags.into_iter().map(|tag| tag.into()).collect())
    }
}

impl From<crate::model::tag_category::TagCategory> for TagCategory {
    fn from(tag_category: crate::model::tag_category::TagCategory) -> Self {
        Self {
            id: Id(tag_category.id),
            name: tag_category.name,
            created_at: tag_category.created_at.to_string(),
            updated_at: tag_category.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct TagCategoryForCreate {
    pub name: String,
}

impl Into<crate::model::tag_category::TagCategoryForCreate> for TagCategoryForCreate {
    fn into(self: Self) -> crate::model::tag_category::TagCategoryForCreate {
        crate::model::tag_category::TagCategoryForCreate {
            id: uuid::Uuid::new_v4(),
            name: self.name,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct TagCategoryForUpdate {
    pub name: Option<String>,
}

impl Into<crate::model::tag_category::TagCategoryForUpdate> for TagCategoryForUpdate {
    fn into(self: Self) -> crate::model::tag_category::TagCategoryForUpdate {
        crate::model::tag_category::TagCategoryForUpdate {
            name: self.name,
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}
