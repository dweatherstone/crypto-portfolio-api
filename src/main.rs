use std::net::SocketAddr;

use axum::{
    Router,
    routing::{get, post},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::AppState;

pub mod handlers;
pub mod models;
pub mod state;

#[tokio::main]
async fn main() {
    // Initialize tracing logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::new();

    // Define routes and attach shared state
    let app = Router::new()
        .route("/api/health", get(handlers::health::health_check))
        .route("/api/prices", get(handlers::price::get_prices))
        .route(
            "/api/portfolios",
            get(handlers::portfolio::list_portfolios).post(handlers::portfolio::create_portfolio),
        )
        .route(
            "/api/portfolios/{id}/holdings",
            post(handlers::portfolio::add_holding),
        )
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
