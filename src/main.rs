mod binance;
mod data;
mod strategy;
mod utils;

use anyhow::Result;
use chrono::{Duration ,Utc};
use std::time::Instant;
use tokio::time;

use crate::binance::client::BinanceClient;
use crate::data::storage::TimeSeriesStorage;
use crate::strategy::indicators::MovingAverages;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise logger
    env_logger::init();

    // Create binance client
    let binance_clinet = BinanceClient::new();

    // Create storage with max history of 1000 candles per symbol
    let mut storage = TimeSeriesStorage::new(1000);

    // Define symbol and interval
    let symbol = "XRPUSDT";
    let interval = "1m";

    // Initial data load
    println!("Fetching historical data...");
    let historical_candles = binance_clinet.get_klines(symbol, interval, 500).await?;

    for candle in historical_candles {
        storage.add_candlestick(candle);
    }

    println!("Historical data loaded. Starting main loop...");

    // Main loop - fetch new data every minute
    let mut interval = time::interval(std::time::Duration::from_secs(60));

    loop {
        interval.tick().await;
        println!("Fetching new candlesticks...");

        // Get latest candle
        let candles = binance_clinet.get_klines(symbol, "1m", 1).await?;

        if let Some(candle) = candles.first() {
            storage.add_candlestick(candle.clone());

            // Get all candlesticks
            let all_candles = storage.get_candlestick(symbol);

            // Calculate indicators
            let sma_period = 20;
            let ema_period = 20;

            let sma = MovingAverages::calculate_sma(&all_candles, sma_period)?;
            let ema = MovingAverages::calculate_ema(&candles, ema_period)?;

            println!("SMA period is {}m", sma.last().unwrap_or(&0.0));
            println!("EMA period is {}m", ema.last().unwrap_or(&0.0));
        }
    }
}