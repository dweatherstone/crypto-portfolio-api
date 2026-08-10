use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holding {
    pub symbol: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: String,
    pub name: String,
    pub holdings: Vec<Holding>,
}

#[derive(Clone, Default)]
pub struct AppState {
    // In-memory store: Portfolio ID -> Portfolio
    pub portfolios: Arc<Mutex<HashMap<String, Portfolio>>>,
    // In-memory store: Symbol (e.g. "BTC") -> Price in USD
    pub prices: Arc<Mutex<HashMap<String, f64>>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut initial_prices = HashMap::new();
        initial_prices.insert("BTC".to_string(), 65000.00);
        initial_prices.insert("ETH".to_string(), 3400.00);
        initial_prices.insert("SOL".to_string(), 145.50);

        Self {
            portfolios: Arc::new(Mutex::new(HashMap::new())),
            prices: Arc::new(Mutex::new(initial_prices)),
        }
    }
}
