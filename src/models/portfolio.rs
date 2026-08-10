use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreatePortfolioRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct AddHoldingRequest {
    pub symbol: String,
    pub amount: f64,
}
