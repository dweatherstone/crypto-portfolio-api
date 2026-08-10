use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    db,
    models::portfolio::{AddHoldingRequest, CreatePortfolioRequest},
    state::{AppState, Portfolio},
};

// GET /api/portfolios
pub async fn list_portfolios(
    State(state): State<AppState>,
) -> Result<Json<Vec<Portfolio>>, StatusCode> {
    let portfolios = db::portfolio::list(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(portfolios))
}

pub async fn get_details(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Portfolio>, StatusCode> {
    let portfolio = db::portfolio::get(&state.db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(portfolio))
}

// POST /api/portfolios
pub async fn create_portfolio(
    State(state): State<AppState>,
    Json(payload): Json<CreatePortfolioRequest>,
) -> Result<Json<Portfolio>, StatusCode> {
    let portfolio_id = db::portfolio::create(&state.db, payload.user_id, &payload.name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let portfolio = db::portfolio::get(&state.db, portfolio_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(portfolio))
}

// POST /api/portfolios/:id/holdings
pub async fn add_holdings(
    Path(portfolio_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<Vec<AddHoldingRequest>>,
) -> Result<Json<Portfolio>, StatusCode> {
    let holdings: Vec<(String, Decimal)> = payload
        .into_iter()
        .map(|holding_request| (holding_request.symbol, holding_request.amount))
        .collect();

    db::portfolio::add_holdings(&state.db, portfolio_id, &holdings)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let portfolio = db::portfolio::get(&state.db, portfolio_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(portfolio))
}
