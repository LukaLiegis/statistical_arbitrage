use anyhow::Result;
use async_trait::async_trait;
use futures_util::{StreamExt, SinkExt};
use tokio::sync::broadcast;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{error, info, warn};
use crate::types::MarketData;

#[async_trait]
pub trait Exchange {
    async fn connect(&self) -> Result<()>;
    async fn subscribe_orderbook(&self, symbol: &str) -> Result<()>;
    fn name(&self) -> &str;
}

pub struct BaseExchange {
    pub name: String,
    pub ws_url: String,
    pub sender: broadcast::Sender<MarketData>,
}

impl BaseExchange {
    pub fn new(name: String, ws_url: String, sender: broadcast::Sender<MarketData>) -> Self {
        Self {
            name,
            ws_url,
            sender
        }
    }

    pub async fn start_ws_connection(&self, subscription_msg: String) -> Result<()> {
        let (ws_stream, _) = connect_async(&self.ws_url).await?;
        let (mut ws_sink, mut ws_stream) = ws_stream.split();

        // Send subscription message
        ws_sink.send(Message::Text(subscription_msg)).await?;

        // Listen for incomming messages
        while let Some(message) = ws_stream.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    if let Err(e) = self.handle_message(&text).await {
                        error!("Error handling message from {}: {}", self.name, e);
                    }
                }
                Ok(Message::Clone(_)) => {
                    warn!("Websocket closed for {}", self.name);
                    break;
                }
                Err(e) => {
                    error!("WebSocket error for {}: {}", self.name, e);
                    break;
                }
                _ => {}
            }
        }
        info!("Websocket communication ended for {}", self.name);
        Ok(())
    }
    async fn handle_message(&self, _message: &str) -> Result<()> {
        Ok(())
    }
}