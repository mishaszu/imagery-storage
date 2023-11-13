use async_graphql::{Context, Object, Result};

use crate::model::album::AlbumBmc;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{graphql::scalars::Id, model::ModelManager};

use super::model::Album;

#[derive(Default)]
pub struct AlbumQuery;

#[Object]
impl AlbumQuery {
    async fn album(&self, ctx: &Context<'_>, id: Id) -> Result<Album> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let album = AlbumBmc::get(mm, id.into()).map_err(GraphQLError::ModelError)?;
        Ok(album.into())
    }

    async fn albums(&self, ctx: &Context<'_>) -> Result<Vec<Album>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let albums = AlbumBmc::list(mm).map_err(GraphQLError::ModelError)?;
        Ok(albums.into_iter().map(|a| a.into()).collect())
    }
}
