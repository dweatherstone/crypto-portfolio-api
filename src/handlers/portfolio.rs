use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    models::portfolio::{AddHoldingRequest, CreatePortfolioRequest},
    state::{AppState, Holding, Portfolio},
};

// GET /api/portfolios
pub async fn list_portfolios(
    State(state): State<AppState>,
) -> Result<Json<Vec<Portfolio>>, StatusCode> {
    let store = state
        .portfolios
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let list = store.values().cloned().collect();
    Ok(Json(list))
}

// POST /api/portfolios
pub async fn create_portfolio(
    State(state): State<AppState>,
    Json(payload): Json<CreatePortfolioRequest>,
) -> Result<(StatusCode, Json<Portfolio>), StatusCode> {
    let mut store = state
        .portfolios
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = format!("port_{}", store.len() + 1);
    let new_portfolio = Portfolio {
        id: id.clone(),
        name: payload.name,
        holdings: vec![],
    };

    store.insert(id, new_portfolio.clone());
    Ok((StatusCode::CREATED, Json(new_portfolio)))
}

// POST /api/portfolios/:id/holdings
pub async fn add_holding(
    Path(portfolio_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<AddHoldingRequest>,
) -> Result<Json<Portfolio>, StatusCode> {
    let mut store = state
        .portfolios
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let portfolio = store.get_mut(&portfolio_id).ok_or(StatusCode::NOT_FOUND)?;

    // Update amount if holding already exists, otherwise add it
    if let Some(existing) = portfolio
        .holdings
        .iter_mut()
        .find(|h| h.symbol == payload.symbol)
    {
        existing.amount += payload.amount;
    } else {
        portfolio.holdings.push(Holding {
            symbol: payload.symbol,
            amount: payload.amount,
        });
    }

    Ok(Json(portfolio.clone()))
}
