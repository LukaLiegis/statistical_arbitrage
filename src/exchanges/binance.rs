use std::fmt::format;

pub struct Binance {
    base: BaseExchange
}

impl Binance {
    pub fn new(sender: broadcast::Sender<MarketData>) -> Self {
        Self {
            base: BaseExchange::new(
                "binance".to_string(),
                "wss://stream.binance.com".to_string(),
                sender,
            ),
        }
    }

    async fn handle_binance_message(&self, message: &str) -> Result<()> {
        // Parse Binance format
        #[derive(Deserialize)]
        struct BinanceOrderbook {
            s: String,
            b: Vec<OrderLevel>,
            a: Vec<OrderLevel>,
            #[serde(rename = "E")]
            event_time: i64,
        }

        let data: BinanceOrderbook = serde_json::from_str(message)?;

        let order_book = OrderBook {
            symbol: data.s,
            bids: data.b.into_iter()
                .filter_map(|level| {
                    if level.len() >= 2 {
                        Some(OrderLevel {
                            price: level[0].parse().ok()?,
                            quantity: level[1].parse().ok()?,
                        })
                    } else {
                        None
                    }
                })
            .collect(),
            asks: data.a.into_iter()
                .filter_map(|level| {
                    if level.len() >= 2 {
                        Some(OrderLevel {
                            price: level[0].parse().ok()?,
                            quantity: level[1].parse().ok()?,
                        })
                    } else {
                        None
                    }
                })
            .collect(),
            timestamp: data.event_time,
        };

        // Send market data update
        let market_data = MarketData {
            exchange: "binance".to_string(),
            order_book,
        };

        if let Err(e) = self.base.sender.send(market_data) {
            warn!("Failed to send market data: {}", e);
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl Exchange for Binance {
    async fn connect(&self) -> Result<()> {
        self.base.start_ws_connection("".to_string()).await?;
    }

    async fn subscribe_orderbook(&self, symbol: &str) -> Result<()> {
        let subscription = format!(
            r#"{{"method": "SUBSCRIBE", "params": ["{}@depth@100ms"], "id": 1}}"#,
            symbol.to_lowercase()
        );

        self.base.start_ws_connection(subscription).await
    }

    fn name(&self) -> &str {
        &self.base.name
    }
}