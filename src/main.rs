use chrono::Utc;
use tsdb::TimeSeries;

fn main() {
    let mut tsdb = TimeSeries::new();

    tsdb.add_data("temperature".to_string(), Utc::now(), 22.5);
    tsdb.add_data("temperature".to_string(), Utc::now(), 23.0);

    if let Some(data) = tsdb.get_data("temperature") {
        println!("Temperature Data: {:?}", data);
    } else {
        println!("No data found for temperature.");
    }
}
