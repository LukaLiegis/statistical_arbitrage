// Make modules available within this module
pub mod client;
pub mod models;

// Re-export important items for convenience
pub use client::BinanceClient;
pub use models::*;