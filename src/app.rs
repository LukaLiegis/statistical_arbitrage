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
        let (sender, receiver) = broadcast::channel(1000);

        // Initialize exchanges
        exchanges.insert("binance".to_string(), Box::new(Binance::new(sender.clone())));
        exchanges.insert("bybit".to_string(), Box::new(Bybit::new()));

        Self {
            exchanges,
            market_data_sender: sender,
            market_data_receiver: receiver,
        }
    }

    /// Start all exchange connections
    pub async fn start(&self, symbols: Vec<&str>) -> Result<()> {
        // Start a task to listen to market data
        self.start_market_data_listener().await;

        // Connect to all exchanges
        for (name, exchange) in &self.exchanges {
            let exchanage_clone = name.clone();
            let symbols_clone = symbols.clone();

            tokio::spawn(async move {
                info!("Connecting to {}", exchanage_clone);

                if let Err(e) = exchange.connect().await {
                    error!("Error connecting to exchange {}: {}", exchanage_clone, e);
                    return;
                }

                for symbol in symbols_clone {
                    if let Err(e) = exchange.subscribe_orderbook(symbol).await {
                        error!("Error subscribing to symbol {} on {}: {}", symbol, exchanage_clone, e);
                    }
                }
            });
        }

        Ok(())
    }
    /// Listen for market data updates
    async fn start_market_data_listener(&self) {
        let mut receiver = self.market_data_sender.subscribe();

        tokio::spawn(async move {
            while let Ok(market_data) = receiver.recv().await {
                // Process market data
                info!(
                    "Received market data from {}: {} has {} bids and {} asks",
                    market_data.exchange,
                    market_data.order_book.symbol,
                    market_data.bids.len(),
                    market_data.asks.len()
                );

                // Here logic will be added to spot arb opportunities
            }
        });
    }
}