use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPoint {
    timestamp: DateTime<Utc>,
    value: f64,
}

#[derive(Default)]
pub struct TimeSeries {
    data: HashMap<String, Vec<DataPoint>>,
}

impl TimeSeries {
    pub fn new() -> Self {
        TimeSeries {
            data: HashMap::new(),
        }
    }

    pub fn add_data(&mut self, measurement: String, timestamp: DateTime<Utc>, value: f64) {
        let entry = self.data.entry(measurement).or_insert_with(Vec::new);
        entry.push(DataPoint { timestamp, value });
    }

    pub fn get_data(&self, measurement: &str) -> Option<&Vec<DataPoint>> {
        self.data.get(measurement)
    }
}
