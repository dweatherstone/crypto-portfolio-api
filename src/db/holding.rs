use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct HoldingRow {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub symbol: String,
    pub amount: Decimal,
}
