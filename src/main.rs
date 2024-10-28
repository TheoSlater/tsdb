use tsdb::FinancialTimeSeries;
use chrono::Utc;

fn main() {
    let filename = "financial_timeseries_data.json";

    // Initialize a new FinancialTimeSeries or load from file
    let mut financial_time_series = match FinancialTimeSeries::load_from_file(filename) {
        Ok(ts) => ts,
        Err(_) => FinancialTimeSeries::new(), // Create a new one if loading fails
    };

    let now = Utc::now();
    
    // Add some financial data
    financial_time_series.add_data("AAPL".to_string(), now, 150.0, 152.0, 153.0, 149.5, 1000000);
    financial_time_series.add_data("AAPL".to_string(), now + chrono::Duration::seconds(60), 152.0, 151.0, 155.0, 150.0, 1200000);
    financial_time_series.add_data("AAPL".to_string(), now + chrono::Duration::seconds(120), 151.0, 154.0, 155.5, 150.5, 900000);

    // Save to JSON file
    if let Err(e) = financial_time_series.save_to_file(filename) {
        eprintln!("Failed to save to file: {}", e);
    } else {
        println!("Data successfully saved to {}", filename);
    }

    // Print the current state
    println!("Current Financial Time Series: {:?}", financial_time_series);

    // Query and print the data for AAPL
    if let Some(data) = financial_time_series.get_data("AAPL") {
        println!("Retrieved financial data for AAPL: {:?}", data);
    } else {
        println!("No data found for the specified asset.");
    }
}
