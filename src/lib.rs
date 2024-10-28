use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancialDataPoint {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FinancialTimeSeries {
    pub data: HashMap<String, Vec<FinancialDataPoint>>, // Keyed by asset symbol (e.g., "AAPL", "GOOGL")
}

impl FinancialTimeSeries {
    pub fn new() -> Self {
        FinancialTimeSeries {
            data: HashMap::new(),
        }
    }

    pub fn add_data(&mut self, symbol: String, timestamp: DateTime<Utc>, open: f64, close: f64, high: f64, low: f64, volume: u64) {
        let entry = self.data.entry(symbol).or_insert_with(Vec::new);
        entry.push(FinancialDataPoint { timestamp, open, close, high, low, volume });
    }

    pub fn get_data(&self, symbol: &str) -> Option<&Vec<FinancialDataPoint>> {
        self.data.get(symbol)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let json_data = self.to_json().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(filename, json_data)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let json_data = std::fs::read_to_string(filename)?;
        Self::from_json(&json_data).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}
