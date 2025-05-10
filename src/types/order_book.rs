use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderLevel {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: Vec<OrderLevel>,
    pub asks: Vec<OrderLevel>,
    pub timestamp: i64,
}