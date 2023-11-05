use axum::{middleware, response::IntoResponse, routing::get, Router};
use reqwest::Client;

mod routes_image;

use crate::model::ModelManager;

use self::routes_image::{get_image, get_raw_image};

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
        .route("/get_image", get(get_image))
        .route("/get_raw_image/:album_id/:image_id", get(get_raw_image))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_require))
        .with_state(ApiState { reqwest_client, mm })
}

async fn test() -> impl IntoResponse {
    "User logged in".to_string()
}
