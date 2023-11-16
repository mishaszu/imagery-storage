use crate::crypt::token::Token;
use crate::ctx::Ctx;
use crate::model::user::UserBmc;
use crate::model::ModelManager;
use crate::web::{set_token_cookie, AUTH_TOKEN};
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

#[allow(dead_code)] // For now, until we have the rpc.
pub async fn mw_ctx_require<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve<B>(
    mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = _ctx_resolve(mm, &cookies).await;

    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    // Store the ctx_ext_result in the request extension
    // (for Ctx extractor).
    req.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(req).await)
}

async fn _ctx_resolve(mm: State<ModelManager>, cookies: &Cookies) -> CtxExtResult {
    // -- Get Token String
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    // -- Parse Token
    let token: Token = token.parse().map_err(|_| CtxExtError::TokenWrongFormat)?;
    let token_claims = token
        .parse_claims()
        .map_err(|_| CtxExtError::FailValidate)?;

    // -- Get UserForAuth
    let (user, account) = UserBmc::get(&mm, &token_claims.user_id).map_err(|_| {
        debug!("error!");
        CtxExtError::UserNotFound
    })?;

    // -- Validate Token
    token.validate().map_err(|_| CtxExtError::FailValidate)?;

    // -- Update Token
    set_token_cookie(cookies, token);

    // -- Create CtxExtResult
    Ok(Ctx::new(user.id, account.id, &user.nick, &account.kind))
}

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,
    TokenWrongFormat,

    UserNotFound,
    // ModelAccessError(String),
    FailValidate,
    // CannotSetTokenCookie,
    CtxNotInRequestExt,
    // CtxCreateFail(String),
}
// endregion: --- Ctx Extractor Result/Error
