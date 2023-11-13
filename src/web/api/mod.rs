use axum::{
    extract::DefaultBodyLimit,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use reqwest::Client;

mod routes_image;

use crate::model::ModelManager;

use self::routes_image::{get_image, post_image, post_images};

use super::middleware::mw_auth::mw_ctx_require;

#[derive(Clone)]
pub struct ApiState {
    pub reqwest_client: Client,
    pub mm: ModelManager,
}

pub fn routes(mm: ModelManager) -> Router {
    let reqwest_client = Client::new();
    Router::new()
        .route("/test", get(test))
        .route("/image/:image_id", get(get_image))
        .route("/image", post(post_image))
        // .route("/images", post(post_images))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_require))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 10))
        .with_state(ApiState { reqwest_client, mm })
}

async fn test() -> impl IntoResponse {
    "User logged in".to_string()
}
