use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::graphql::scalars::Id;
use crate::model::rating_score::RatingScoreBmc;
use crate::model::ModelManager;
use crate::web::api::graphql::error::Error as GraphQLError;
use crate::web::api::graphql::rating_score::model::RatingScore;

#[derive(Debug, Clone, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct Rating {
    pub id: Id,
    pub name: String,
    pub scale: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

#[ComplexObject]
impl Rating {
    async fn scores(&self, ctx: &Context<'_>) -> Result<Vec<RatingScore>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let scores = RatingScoreBmc::get_by_rating_id(mm, self.id.0)
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(scores.into_iter().map(|r| r.into()).collect())
    }
}

impl From<crate::model::rating::Rating> for Rating {
    fn from(rating: crate::model::rating::Rating) -> Self {
        Self {
            id: Id(rating.id),
            name: rating.name,
            scale: rating.scale,
            description: rating.description,
            created_at: rating.created_at.to_string(),
            updated_at: rating.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct RatingForCreate {
    pub name: String,
    pub scale: String,
    pub description: String,
}

impl Into<crate::model::rating::RatingForCreate> for RatingForCreate {
    fn into(self) -> crate::model::rating::RatingForCreate {
        crate::model::rating::RatingForCreate {
            id: uuid::Uuid::new_v4(),
            name: self.name,
            scale: self.scale,
            description: self.description,
        }
    }
}

#[derive(Debug, Clone, Deserialize, InputObject)]
pub struct RatingForUpdate {
    pub name: Option<String>,
    pub scale: Option<String>,
    pub description: Option<String>,
}

impl Into<crate::model::rating::RatingForUpdate> for RatingForUpdate {
    fn into(self) -> crate::model::rating::RatingForUpdate {
        crate::model::rating::RatingForUpdate {
            name: self.name,
            scale: self.scale,
            description: self.description,
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}
