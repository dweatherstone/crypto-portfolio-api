use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreatePortfolioRequest {
    pub name: String,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct AddHoldingRequest {
    pub symbol: String,
    pub amount: Decimal,
}
