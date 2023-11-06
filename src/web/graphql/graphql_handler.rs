use async_graphql_axum::GraphQLSubscription;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use reqwest::Client;

use crate::ctx::Ctx;

use crate::model::ModelManager;
use crate::web::Result;

use super::{create_schema, ImagerySchema};

#[derive(Clone)]
pub struct GraphQlState {
    schema: ImagerySchema,
    reqwest_client: Client,
}

pub fn routes(mm: ModelManager) -> Router {
    let reqwest_client = Client::new();
    let schema = create_schema(mm.clone(), reqwest_client.clone());
    Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .with_state(GraphQlState {
            schema,
            reqwest_client,
        })
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql")
            .subscription_endpoint("/graphql/ws"),
    ))
}

pub async fn graphql_handler(
    State(graph_ql_state): State<GraphQlState>,
    ctx: Result<Ctx>,
    request: GraphQLRequest,
) -> impl IntoResponse {
    let state = graph_ql_state;
    let builer_schema = match ctx {
        Ok(ctx) => request.0.data(ctx),
        Err(_) => request.0.data(ctx),
    };
    let builder = state.schema.execute(builer_schema).await;
    let response = GraphQLResponse(builder.into());
    response.into_response()
}
