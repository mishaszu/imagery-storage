use async_graphql::{Context, Object, Result};

use crate::ctx::Ctx;
use crate::model::account::{Account, AccountBmc};
use crate::model::ModelManager;
use crate::web::graphql::error::Error as GraphQLError;

use super::model::Image;

#[derive(Default)]
pub struct ImageQuery;

#[Object]
impl ImageQuery {
    /// return logged user images
    async fn images(&self, ctx: &Context<'_>) -> Result<Vec<Image>> {
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let app_ctx = ctx.data_opt::<Ctx>();
        let user_id = match app_ctx {
            Some(ctx) => ctx.user_id,
            None => return Err(GraphQLError::AccessError("No user logged in".to_string()).into()),
        };

        let Account { is_banned, .. } =
            AccountBmc::get_by_user_id(mm, &user_id).map_err(GraphQLError::ModelError)?;

        if is_banned {
            return Err(GraphQLError::AccessError("User is banned".to_string()).into());
        }

        let images = crate::model::image::ImageBmc::list_user(mm, &user_id)
            .map_err(GraphQLError::ModelError)?;
        Ok(images.into_iter().map(|r| r.into()).collect())
    }
}
