use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::response::{Html, IntoResponse};

use crate::ctx::Ctx;

use crate::web::api::GraphQlState;
use crate::web::Result;

pub async fn graphql_playground() -> impl IntoResponse {
    Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/api/graphql")
            .subscription_endpoint("/api/graphql/ws"),
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
