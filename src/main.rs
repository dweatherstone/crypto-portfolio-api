use std::{env, net::SocketAddr, time::Duration};

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::AppState;

pub mod db;
pub mod handlers;
pub mod models;
pub mod state;

#[tokio::main]
async fn main() {
    // Initialize tracing logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres pool");

    // Run pending migrations automatically on startup
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let state = AppState::new(pool);

    // Define routes and attach shared state
    let app = Router::new()
        .route("/api/health", get(handlers::health::health_check))
        .route("/api/prices", get(handlers::price::get_prices))
        .route(
            "/api/portfolios",
            get(handlers::portfolio::list_portfolios).post(handlers::portfolio::create_portfolio),
        )
        .route(
            "/api/portfolios/{id}",
            get(handlers::portfolio::get_details),
        )
        .route(
            "/api/portfolios/{id}/holdings",
            post(handlers::portfolio::add_holdings),
        )
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
