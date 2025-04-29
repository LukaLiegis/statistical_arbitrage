pub struct Config {
    pub symbol: String,
    pub interval: String,
    pub sma_period: usize,
    pub ema_period: usize,
}

impl Config {
    pub fn default() -> Self {
        Self {
            symbol: "XRPUSDT".to_string(),
            interval: "1m".to_string(),
            sma_period: 20,
            ema_period: 20,
        }
    }
}