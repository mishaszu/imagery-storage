use async_graphql::{Context, Object, Result};

use crate::model::album::AlbumBmc;
use crate::web::api::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::{
    AlbumForCreate, AlbumForUpdate, AlbumImageForCreate, AlbumScoreForCreate, AlbumTagForCreate,
};

#[derive(Default)]
pub struct AlbumMutation;

#[Object]
impl AlbumMutation {
    pub async fn create_album(
        &self,
        ctx: &Context<'_>,
        input: AlbumForCreate,
    ) -> Result<crate::web::api::graphql::album::model::Album> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let album =
            AlbumBmc::create(mm, input.into()).map_err(GraphQLError::from_model_to_graphql)?;
        Ok(album.into())
    }

    pub async fn update_album(
        &self,
        ctx: &Context<'_>,
        id: Id,
        input: AlbumForUpdate,
    ) -> Result<crate::web::api::graphql::album::model::Album> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let album = AlbumBmc::update(mm, id.into(), input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(album.into())
    }

    pub async fn delete_album(&self, ctx: &Context<'_>, id: Id) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        AlbumBmc::delete(mm, id.into()).map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Album deleted".to_string())
    }

    pub async fn create_album_with_images(
        &self,
        ctx: &Context<'_>,
        album: AlbumForCreate,
        images: Vec<String>,
    ) -> Result<crate::web::api::graphql::album::model::Album> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        let images: Vec<crate::model::image::ImageForCreate> = images
            .into_iter()
            .map(|i| {
                let path = format!("{}/{}", album.title, i);
                path.into()
            })
            .collect();

        let album = AlbumBmc::create_with_images(mm, album.into(), images)
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok(album.into())
    }

    pub async fn delete_album_with_images(&self, ctx: &Context<'_>, id: Id) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };
        AlbumBmc::delete_with_images(mm, id.into()).map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Album deleted".to_string())
    }

    pub async fn add_album_tag(
        &self,
        ctx: &Context<'_>,
        input: AlbumTagForCreate,
    ) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        AlbumBmc::create_album_tag(mm, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Album tag added".to_string())
    }

    pub async fn add_album_image(
        &self,
        ctx: &Context<'_>,
        input: AlbumImageForCreate,
    ) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        AlbumBmc::create_album_image(mm, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Album image added".to_string())
    }

    pub async fn add_album_score(
        &self,
        ctx: &Context<'_>,
        input: AlbumScoreForCreate,
    ) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        AlbumBmc::create_album_rating_score(mm, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Album score added".to_string())
    }
}
