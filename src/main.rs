use anyhow::Result;
use tracing_subscriber;
use statistical_arbitrage::{TradingApp};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create trading application
    let app = TradingApp::new()?;

    let symbols = vec!["XRPUSDC", "XRP/USDC"];

    app.start(symbols).await?;

    tokio::signal::ctrl_c().await?;

    Ok(())
}