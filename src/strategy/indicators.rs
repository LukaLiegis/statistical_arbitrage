use anyhow::{ Result, anyhow};
use crate::data::candlestick::Candlestick;

pub struct MovingAverages;

impl MovingAverages {
    // Simple moving averages
    pub fn calculate_sma(data: &[Candlestick], period: usize) -> Result<Vec<f64>> {
        if data.len() < period {
            return Err(anyhow!("Not enough data points for SMA calculation"));
        }

        let mut result = Vec::new();

        for i in period..=data.len() {
            let window = &data[i - period..i];
            let sum: f64 = window.iter().map(|c| c.close).sum();
            let sma = sum / period as f64;
            result.push(sma);
        }

        Ok(result)
    }

    pub fn calculate_ema(data: &[Candlestick], period: usize) -> Result<Vec<f64>> {
        if data.len() < period {
            return Err(anyhow!("Not enough data points for EMA calculation"));
        }

        let multiplier = 2.0 / (period as f64 + 1.0);
        let mut ema_values = Vec::new();

        // Calculate initial SMA
        let initial_sma = {
            let sum: f64 = data[0..period].iter().map(|c| c.close).sum();
            sum / period as f64
        };

        ema_values.push(initial_sma);

        // Calculate EMA values
        for i in period..data.len() {
            let previous_ema = ema_values.last().unwrap();
            let current_price = data[i].close;
            let ema = (current_price - previous_ema) * multiplier + previous_ema;
            ema_values.push(ema);
        }

        Ok(ema_values)
    }
}