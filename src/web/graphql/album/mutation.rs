use async_graphql::{Context, Object, Result};

use crate::model::album::AlbumBmc;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::{Album, AlbumForCreate, AlbumForUpdate, AlbumPostForCreate};

#[derive(Default)]
pub struct AlbumMutation;

#[Object]
impl AlbumMutation {
    pub async fn create_album(&self, ctx: &Context<'_>, input: AlbumForCreate) -> Result<Album> {
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
    ) -> Result<Album> {
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

    // pub async fn create_album_with_images(
    //     &self,
    //     ctx: &Context<'_>,
    //     album: AlbumForCreate,
    //     images: Vec<String>,
    // ) -> Result<Album> {
    //     let mm = ctx.data_opt::<ModelManager>();
    //     let mm = match mm {
    //         Some(mm) => mm,
    //         None => return Err(GraphQLError::ModalManagerNotInContext.into()),
    //     };
    //     let images: Vec<crate::model::image::ImageForCreate> = images
    //         .into_iter()
    //         .map(|i| {
    //             let path = format!("{}/{}", album.name, i);
    //             path.into()
    //         })
    //         .collect();

    //     let album = AlbumBmc::create_with_images(mm, album.into(), images)
    //         .map_err(GraphQLError::from_model_to_graphql)?;
    //     Ok(album.into())
    // }

    // pub async fn delete_album_with_images(&self, ctx: &Context<'_>, id: Id) -> Result<String> {
    //     let mm = ctx.data_opt::<ModelManager>();
    //     let mm = match mm {
    //         Some(mm) => mm,
    //         None => return Err(GraphQLError::ModalManagerNotInContext.into()),
    //     };
    //     AlbumBmc::delete_with_images(mm, id.into()).map_err(GraphQLError::from_model_to_graphql)?;
    //     Ok("Album deleted".to_string())
    // }

    pub async fn add_album_image(
        &self,
        ctx: &Context<'_>,
        input: AlbumPostForCreate,
    ) -> Result<String> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        AlbumBmc::create_album_post(mm, input.into())
            .map_err(GraphQLError::from_model_to_graphql)?;
        Ok("Album image added".to_string())
    }
}
