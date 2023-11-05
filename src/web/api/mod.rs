use async_graphql_axum::GraphQLSubscription;
use axum::{response::IntoResponse, routing::get, Router};
use reqwest::Client;

mod graphql;
mod routes_image;

use crate::model::ModelManager;

use self::graphql::{create_schema, graphql_playground, WooBooSchema};
use self::routes_image::{get_image, get_raw_image};

#[derive(Clone)]
pub struct GraphQlState {
    schema: WooBooSchema,
    reqwest_client: Client,
}

pub fn routes(mm: ModelManager) -> Router {
    let reqwest_client = Client::new();
    let schema = create_schema(mm.clone(), reqwest_client.clone());
    Router::new()
        .route("/test", get(test))
        .route("/get_image", get(get_image))
        .route("/get_raw_image/:album_id/:image_id", get(get_raw_image))
        .route(
            "/graphql",
            get(graphql_playground).post(graphql::graphql_handler),
        )
        .route_service("/graphql/ws", GraphQLSubscription::new(schema.clone()))
        // .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_require))
        .with_state(GraphQlState {
            schema,
            reqwest_client,
        })
}

async fn test() -> impl IntoResponse {
    "User logged in".to_string()
}
