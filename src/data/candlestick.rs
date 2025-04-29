use chrono::{DateTime, UTC};
use serde::{Deserialize ,Serialize};

// Raw candle data from API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleData{
    pub open_time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub close_time: i64,
    pub quote_asset_volume: String,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: String,
    pub taker_buy_quote_asset_volume: String,
    pub ignore: String,
}

impl Candlestick {
    pub fn new(
        symbol: String,
        timestamp: DateTime<UTC>,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> Self{
        Self {
            symbol,
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}