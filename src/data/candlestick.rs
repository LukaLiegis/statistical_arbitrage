// src/data/candlestick.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize ,Serialize};

// Raw candle data from API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candlestick{
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
        open_time: i64,
        open: String,
        high: String,
        low: String,
        close: String,
        volume: String,
        close_time: i64,
        quote_asset_volume: String,
        number_of_trades: i64,
        taker_buy_base_asset_volume: String,
        taker_buy_quote_asset_volume: String,
        ignore: String,
    ) -> Self {
        Self {
            open_time,
            open,
            high,
            low,
            close,
            volume,
            close_time,
            quote_asset_volume,
            number_of_trades,
            taker_buy_base_asset_volume,
            taker_buy_quote_asset_volume,
            ignore,
        }
    }
}