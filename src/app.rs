use std::collections::HashMap;
use anyhow::Result;
use tokio::sync::broadcast;
use tracing::{error, info};
use crate::exchanges::{Exchange, Binance, };
use crate::types::MarketData;

pub struct TradingApp {
    exchanges: HashMap<String, Box<dyn Exchange>>,
    market_data_sender: broadcast::Sender<MarketData>,
    market_data_receiver: broadcast::Receiver<MarketData>,
}

impl TradingApp {
    pub fn new() -> Self {

    }

    pub async fn start(&self, symbols: Vec<&str>) -> Result<()> {

    }

    async fn start_market_data_listener(&self) {

    }
}