use wasm_bindgen::prelude::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[wasm_bindgen]
pub struct TimeSeriesData {
    data: HashMap<String, Vec<(DateTime<Utc>, f64)>>, // Maps the data to timestamps, values
}

#[wasm_bindgen]
impl TimeSeriesData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TimeSeriesData {
        TimeSeriesData {
            data: HashMap::new(),
        }
    }

    pub fn add_data(&mut self, measurement: String, timestamp: String, value: f64) {
        let time: DateTime<Utc> = timestamp.parse().unwrap();
        self.data.entry(measurement).or_insert_with(Vec::new).push((time, value));
    }

    pub fn get_data(&self, measurement: String) -> JsValue {
        let binding = Vec::new();
        let entries = self.data.get(&measurement).unwrap_or(&binding);
        let json = serde_json::to_string(entries).unwrap(); // i think this serializes to JSON string
        let js_value = JsValue::from_str(&json); // Create JsValue from the JSON string
        js_value // returns it
    }
    
    
}
