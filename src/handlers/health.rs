use axum::{extract::State, http::StatusCode};

use crate::{db, state::AppState};

// GET /api/health
pub async fn health_check(State(state): State<AppState>) -> (StatusCode, &'static str) {
    match db::health_check(&state.db).await {
        Ok(_) => (StatusCode::OK, "OK"),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "Database unavailable"),
    }
}
