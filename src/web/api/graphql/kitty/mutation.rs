use async_graphql::{Context, Object, Result};

use crate::model::kitty::KittyBmc;
use crate::web::api::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::{
    KittyForCreate, KittyForUpdate, KittyImageForCreate, KittyScoreForCreate, KittyTagForCreate,
};

#[derive(Default)]
pub struct KittyMutation;

#[Object]
impl KittyMutation {
    pub async fn create_kitty(
        &self,
        ctx: &Context<'_>,
        input: KittyForCreate,
    ) -> Result<crate::web::api::graphql::kitty::model::Kitty> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let kitty =
            KittyBmc::create(mm, input.into()).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(kitty.into())
    }

    pub async fn update_kitty(
        &self,
        ctx: &Context<'_>,
        id: Id,
        input: KittyForUpdate,
    ) -> Result<crate::web::api::graphql::kitty::model::Kitty> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let kitty = KittyBmc::update(mm, id.into(), input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(kitty.into())
    }

    pub async fn delete_kitty(&self, ctx: &Context<'_>, id: Id) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        KittyBmc::delete(mm, id.into()).map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Kitty deleted".to_string())
    }

    pub async fn add_kitty_tag(
        &self,
        ctx: &Context<'_>,
        input: KittyTagForCreate,
    ) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        KittyBmc::create_kitty_tag(mm, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Kitty tag created".to_string())
    }

    pub async fn add_kitty_image(
        &self,
        ctx: &Context<'_>,
        input: KittyImageForCreate,
    ) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        KittyBmc::create_kitty_image(mm, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Kitty image created".to_string())
    }

    pub async fn add_kitty_score(
        &self,
        ctx: &Context<'_>,
        input: KittyScoreForCreate,
    ) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        KittyBmc::create_kitty_rating_score(mm, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Kitty score created".to_string())
    }

    pub async fn add_kitty_album(
        &self,
        ctx: &Context<'_>,
        input: crate::web::api::graphql::kitty::model::KittyAlbumForCreate,
    ) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        KittyBmc::create_kitty_album(mm, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Kitty album created".to_string())
    }
}
