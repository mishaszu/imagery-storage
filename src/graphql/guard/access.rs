use async_graphql::{Context, Guard, Result};
use uuid::Uuid;

use crate::access::Accesship;
use crate::model::image::ImageBmc;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{ctx::Ctx, graphql::scalars::Id, model::ModelManager};

pub struct CreatorGuard {
    post_id: Uuid,
    admin_allowed: bool,
}

impl CreatorGuard {
    pub fn new(id: Id, admin_allowed: bool) -> Self {
        Self {
            post_id: id.into(),
            admin_allowed,
        }
    }
}

#[async_trait::async_trait]
impl Guard for CreatorGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let app_ctx = ctx.data_opt::<Ctx>();
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(GraphQLError::ModalManagerNotInContext.into());
            }
        };

        let user_account_id = match app_ctx {
            Some(ctx) => ctx.account_id,
            None => return Err(GraphQLError::AccessError("No user logged in".to_string()).into()),
        };

        let post = crate::model::post::PostBmc::get(mm, &self.post_id)
            .map_err(GraphQLError::ModelError)?;

        let has_access = post
            .user_access(mm, user_account_id)
            .map_err(GraphQLError::ModelError)?;

        match (has_access, self.admin_allowed) {
            (Accesship::Admin, true) | (Accesship::Owner, _) => Ok(()),
            _ => Err(GraphQLError::AccessError(user_account_id.to_string()).into()),
        }
    }
}

pub struct ImageCreatorGuard {
    image_id: Uuid,
    admin_allowed: bool,
}

impl ImageCreatorGuard {
    pub fn new(id: Id, admin_allowed: bool) -> Self {
        Self {
            image_id: id.into(),
            admin_allowed,
        }
    }
}

#[async_trait::async_trait]
impl Guard for ImageCreatorGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let app_ctx = ctx.data_opt::<Ctx>();
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => {
                return Err(GraphQLError::ModalManagerNotInContext.into());
            }
        };

        let (user_id, user_account_id) = match app_ctx {
            Some(ctx) => (ctx.user_id, ctx.account_id),
            None => return Err(GraphQLError::AccessError("No user logged in".to_string()).into()),
        };

        let image = ImageBmc::get(mm, &self.image_id).map_err(GraphQLError::ModelError)?;

        match (user_id == image.user_id, self.admin_allowed) {
            (_, true) | (true, _) => Ok(()),
            _ => Err(GraphQLError::AccessError(user_account_id.to_string()).into()),
        }
    }
}
