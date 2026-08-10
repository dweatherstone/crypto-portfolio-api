use axum::http::StatusCode;

// GET /api/health
pub async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}
