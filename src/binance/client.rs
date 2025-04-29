use anyhow::Result;
use chrono::{DateTime ,Utc};
use reqwest::Client;
use serde::{Serialize,Deserialize};

use crate::data::candlestick::Candlestick;

pub struct BinanceClient {
    client: Client,
    base_url: String,
}

impl BinanceClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.binance.com".to_string(),
        }
    }

    pub async fn get_klines(&self, symbol: &str, interval: &str, limit: u32) -> Result<Vec<Candlestick>> {
        let url = format!("{}/api/v3/klines", self.base_url);

        let response = self.client
            .get(url)
            .query(&[
                ("symbol", symbol),
                ("interval", interval),
                ("limit", &limit.to_string()),
            ])
            .send()
            .await?
            .json::<Vec<Vec<serde_json::Value>>>()
            .await?;

        let mut candlesticks = Vec::new();

        for kline in response {
            let timestamp = DateTime::<Utc>::from_timestamp(
                kline[0].as_i64().unwrap_or(0) / 1000,
                0
            ).unwrap();

            let candlestick = Candlestick::new(
                symbol.to_string(),
                timestamp,
                kline[1].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0),
                kline[2].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0),
                kline[3].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0),
                kline[4].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0),
                kline[5].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0),
            );

            candlesticks.push(candlestick);
        }

        Ok(candlesticks)

    }
}