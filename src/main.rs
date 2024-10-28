use tsdb::TimeSeries;
use chrono::Utc;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut time_series = TimeSeries::new();
    time_series.add_data("temperature".to_string(), Utc::now(), 20.5);
    time_series.add_data("temperature".to_string(), Utc::now(), 21.0);

    if let Some(data) = time_series.get_data("temperature") {
        println!("Retrieved data for temperature: {:?}", data);
    } else {
        println!("No data found for the specified measurement.");
    }

    match time_series.to_json() {
        Ok(json) => {
            println!("Serialized JSON: {}", json);
            match File::create("output.json") {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(json.as_bytes()) {
                        eprintln!("Error writing to file: {}", e);
                    } else {
                        println!("JSON data written to output.json");
                    }
                },
                Err(e) => eprintln!("Error creating file: {}", e),
            }
        },
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }

    let json_data = r#"{"data":{"temperature":[{"timestamp":"2024-10-28T12:34:56Z","value":20.5},{"timestamp":"2024-10-28T12:34:56Z","value":21.0}]}}"#;
    match TimeSeries::from_json(json_data) {
        Ok(ts) => println!("Deserialized TimeSeries: {:?}", ts),
        Err(e) => eprintln!("Error deserializing from JSON: {}", e),
    }
}
