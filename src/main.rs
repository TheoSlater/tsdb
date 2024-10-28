use tsdb::TimeSeries;
use chrono::Utc;

fn main() {
    let mut time_series = TimeSeries::new();
    let now = Utc::now();

    time_series.add_data("temperature".to_string(), now, 20.5);
    time_series.add_data("temperature".to_string(), now + chrono::Duration::seconds(60), 21.0);
    time_series.add_data("temperature".to_string(), now + chrono::Duration::seconds(120), 19.5);

    let start_time = now + chrono::Duration::seconds(30);
    let end_time = now + chrono::Duration::seconds(90);

    if let Some(data) = time_series.query_data("temperature", start_time, end_time) {
        println!("Retrieved data for temperature in range: {:?}", data);
    } else {
        println!("No data found for the specified measurement.");
    }
}
