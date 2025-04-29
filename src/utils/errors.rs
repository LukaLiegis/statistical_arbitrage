use thiserror::Error;

#[derive(Debug, Error)]
pub enum TradingError {
    #[error("API Error: {0}")]
    ApiError(String),

    #[error("Data Error: {0}")]
    DataError(String),

    #[error("Calculation Error: {0}")]
    StrategyError(String),
}