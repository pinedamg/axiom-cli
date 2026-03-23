use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenSavings {
    pub raw_bytes: usize,
    pub processed_bytes: usize,
    pub command: String,
    pub timestamp: i64,
}

impl TokenSavings {
    pub fn new(command: &str, raw_bytes: usize, processed_bytes: usize) -> Self {
        Self {
            raw_bytes,
            processed_bytes,
            command: command.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn savings_percentage(&self) -> f64 {
        if self.raw_bytes == 0 {
            return 0.0;
        }
        100.0 * (1.0 - (self.processed_bytes as f64 / self.raw_bytes as f64))
    }
}

pub trait AnalyticsProvider {
    fn record_savings(&self, savings: TokenSavings) -> anyhow::Result<()>;
    fn get_total_savings(&self) -> anyhow::Result<(usize, usize)>;
}
