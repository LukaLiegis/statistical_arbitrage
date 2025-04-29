use std::collections::HashMap;
use anyhow::Results;
use crate::data::candlestick::CandleStick;

pub struct TimeSeriesStorage {
    // Use HashMap where the key is the symbol and the value is a vector of candles
    data: HashMap<String, Vec<CandleStick>>,
    // Max number of candlesticks to store per symbol
    max_history: usize,
}

impl TimeSeriesStorage {
    pub fn new(max_history: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_history,
        }
    }

    pub fn add_candlestick(&mut self, candlestick: CandleStick) {
        let symbol = candlestick.symbol.clone();
        let entry = self.data.entry(symbol).or_insert_with(Vec::new());

        entry.push(candlestick);

        // Ensure we don't exceed max history by removing oldest entries
        if entry.len() > self.max_history {
            entry.remove(0)
        }
    }

    pub fn get_candlestick(&self, symbol: &str) -> Vec<Candlestick> {
        match self.data.get(symbol) {
            Some(candlesticks) => candlesticks.clone(),
            None => Vec::new(),
        }
    }

    pub fn get_latest_candlestick(&self, symbol: &str, n: usize) -> Vec<Candlestick> {
        match self.data.get(symbol) {
            Some(candlesticks) => {
                if candlesticks.len() <= n {
                    candlesticks.clone()
                } else {
                    candlesticks[candlesticks.len() - n..].to_vec()
                }
            },
            None => Vec::new(),
        }
    }

    pub fn save_to_csv(&self, symbol: &str, path: &str) -> Result<()> {
        let candlesticks = self.get_candlestick(symbol);
        let mut wtr = csv::Writer::from_path(path)?;

        for candlestick in candlesticks {
            writer.serialize(candlestick)?;
        }

        writer.flush()?;
        Ok(())
    }

    pub fn load_from_csv(&mut self, symbol: &str, path: &str) -> Result<()> {
        let mut reader = csv::Reader::from_path(path)?;

        for result in reader.deserialize() {
            let candlestick: CandleStick = result?;
            self.add_candlestick(candlestick);
        }

        Ok(())
    }
}