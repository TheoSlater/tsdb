use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPoint {
    timestamp: DateTime<Utc>,
    value: f64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
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

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
