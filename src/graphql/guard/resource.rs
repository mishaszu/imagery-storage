use std::marker::PhantomData;

use async_graphql::Guard;
use uuid::Uuid;

use crate::graphql::scalars::Id;
use crate::model::image::ImageBmc;
use crate::web::graphql::error::Error as GraphQLError;
use crate::{ctx::Ctx, model::ModelManager};

pub trait HasAccess {
    fn is_allowed(
        mm: &ModelManager,
        allow_admin: bool,
        resource_id: &Uuid,
        user: &Uuid,
    ) -> crate::model::error::Result<bool>;
}

pub struct ResourceGuard<T> {
    // if set true it will pass account id to has_access instead of user id
    check_account: bool,
    allow_admin: bool,
    resource_id: Uuid,
    _phantom: PhantomData<T>,
}

impl ResourceGuard<ImageBmc> {
    pub fn new_image(resource_id: Id, allow_admin: bool) -> Self {
        Self {
            check_account: false,
            allow_admin,
            resource_id: resource_id.into(),
            _phantom: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T> Guard for ResourceGuard<T>
where
    T: HasAccess + Send + Sync,
{
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
        let app_ctx = ctx.data_opt::<Ctx>();
        let mm = ctx.data_opt::<ModelManager>();
        let mm = match mm {
            Some(mm) => mm,
            None => return Err(GraphQLError::ModalManagerNotInContext.into()),
        };

        let user_id = match (app_ctx, self.check_account) {
            (Some(ctx), true) => ctx.account_id,
            (Some(ctx), false) => ctx.user_id,
            (None, _) => {
                return Err(GraphQLError::AccessError("No user logged in".to_string()).into())
            }
        };

        let result = T::is_allowed(mm, self.allow_admin, &self.resource_id, &user_id)
            .map_err(GraphQLError::ModelError)?;

        if result {
            Ok(())
        } else {
            Err(async_graphql::Error::from("Access denied"))
        }
    }
}
