use std::collections::HashMap;

use axum::{Json, extract::State, http::StatusCode};

use crate::state::AppState;

// GET /api/prices
pub async fn get_prices(
    State(state): State<AppState>,
) -> Result<Json<HashMap<String, f64>>, StatusCode> {
    let prices = state
        .prices
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(prices.clone()))
}
