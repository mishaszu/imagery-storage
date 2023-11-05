use std::net::SocketAddr;

use axum::middleware;
use axum::{routing::get, Router};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub use self::config::config;
pub use self::error::{Error, Result};
use self::web::middleware::mw_auth::mw_ctx_resolve;
use self::web::routes_login;

mod config;
mod crypt;
mod ctx;
mod error;
pub mod graphql;
mod model;
pub mod schema;
pub mod services;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mm = model::ModelManager::new().await?;
    mm.run_migration();

    let routes_all = Router::new()
        .merge(routes_login::routes(mm.clone()))
        .nest("/api", web::api::routes(mm.clone()))
        .route("/hello", get(hello_world))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("{:<12} - {addr}", "LISTENING");

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn hello_world() -> &'static str {
    info!("{:<12} - test route", "GET");
    "Hello world from WooBoo V3!"
}