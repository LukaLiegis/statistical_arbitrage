pub struct Bybit {
    base: BaseExchange,
}

impl Bybit {
    pub fn new(sender: broadcast::Sender<MarketData>) -> Self {
        Self {
            base: BaseExchange::new(
                "bybit".to_string(),
                "wss://stream.bybit.com/v5/public/spot".to_string(),
                sender,
            ),
        }
    }
    async fn handle_bybit_message(&self, message: &str) -> Result<()> {
        // Parse Bybit format
        #[derive(Deserialize)]
        struct BybitData {
            s: String,
            b: Vec<Vec<String>>,
            a: Vec<Vec<String>>,
            ts: i64,
        }

        #[derive(Deserialize)]
        struct BybitResponse {
            data: BybitData,
        }

        let response: BybitResponse = serde_json::from_str(message)?;

        let order_book = OrderBook {
            symbol: response.data.s,
            bids: response.data.b.into_iter()
                .filter_map(|level|{
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
            asks: response.data.a.into_iter()
                .filter_map(|level|{
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
            timestamp: response.data.ts,
        };

        let market_data = MarketData {
            exchange: "bybit".to_string(),
            order_book,
        };

        if let Err(e) = self.base.sender.send(market_data) {
            warn!("Failed to send market data: {}", e);
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl Exchange for Bybit {
    async fn connect(&self) -> Result<()> {
        self.base.start_ws_connection("".to_string()).await
    }

    async fn subscribe_orderbook(&self, symbol: &str) -> Result<()> {
        let subscription = format!(
            // TODO
            symbol,
        );

        self.base.start_ws_subscription(subscription).await
    }

    fn name(&self) -> &str {
        &self.base.name
    }
}