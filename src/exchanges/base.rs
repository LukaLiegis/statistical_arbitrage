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

impl BaseExchange {}