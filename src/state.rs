use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holding {
    pub id: Uuid,
    pub symbol: String,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: Uuid,
    pub name: String,
    pub user: User,
    pub holdings: Vec<Holding>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    // In-memory cache for ultra-fast, zero-DB-cost price lookups
    pub prices: Arc<Mutex<HashMap<String, f64>>>,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self {
            db,
            prices: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
