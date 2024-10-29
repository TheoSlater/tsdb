// lib.rs

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration as StdDuration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinancialDataPoint {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u64,
}

#[derive(Debug, Default)]
pub struct FinancialTimeSeries {
    data: HashMap<String, Vec<FinancialDataPoint>>, // Data for each asset symbol
    cache: Arc<Mutex<HashMap<String, Vec<FinancialDataPoint>>>>, // Thread-safe cache
}

impl FinancialTimeSeries {
    pub fn new() -> Self {
        FinancialTimeSeries {
            data: HashMap::new(),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_data(
        &mut self,
        symbol: String,
        timestamp: DateTime<Utc>,
        open: f64,
        close: f64,
        high: f64,
        low: f64,
        volume: u64,
    ) {
        let data_point = FinancialDataPoint { timestamp, open, close, high, low, volume };
        self.data.entry(symbol.clone()).or_insert_with(Vec::new).push(data_point.clone());

        let mut cache = self.cache.lock().expect("Failed to lock cache");
        cache.entry(symbol.clone()).or_insert_with(Vec::new).push(data_point.clone());

        let filename = format!("{}_timeseries_data.json", symbol);
        let data_clone = self.data.clone();
        thread::spawn(move || {
            thread::sleep(StdDuration::from_secs(60));
            if let Ok(json_data) = serde_json::to_string(&data_clone) {
                let _ = std::fs::write(&filename, json_data);
            }
        });
    }

    pub fn get_data(&self, symbol: &str) -> Option<&Vec<FinancialDataPoint>> {
        self.data.get(symbol)
    }

    pub fn get_data_by_time_range(
        &self,
        symbol: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Option<Vec<&FinancialDataPoint>> {
        self.data.get(symbol).map(|points| {
            points.iter()
                .filter(|p| p.timestamp >= start && p.timestamp <= end)
                .collect()
        })
    }

    pub fn moving_average(&self, symbol: &str, period: usize) -> Option<Vec<f64>> {
        self.data.get(symbol).map(|points| {
            points.windows(period).map(|window| {
                window.iter().map(|p| p.close).sum::<f64>() / period as f64
            }).collect()
        })
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let json_data = serde_json::to_string(&self.data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(filename, json_data)
    }

    pub fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let json_data = std::fs::read_to_string(filename)?;
        let data: HashMap<String, Vec<FinancialDataPoint>> = serde_json::from_str(&json_data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(FinancialTimeSeries {
            data,
            cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }
}
