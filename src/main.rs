use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::io;
use tsdb::FinancialTimeSeries;

fn main() {
    let mut tsdb = FinancialTimeSeries::new();

    if let Ok(loaded_data) = FinancialTimeSeries::load_from_file("financial_data.json") {
        tsdb = loaded_data; 
        println!("Loaded existing data from financial_data.json.");
    } else {
        eprintln!("Error loading data. Starting fresh.");
    }

    println!("Welcome to the Financial Time Series Database!");

    loop {
        println!("\nEnter the asset symbol (or type 'retrieve <symbol>' to retrieve data / or type 'exit' to finish):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        // Check for retrieve command
        if input.starts_with("retrieve ") {
            let symbol = input[9..].trim();
            let data = tsdb.get_data(symbol);
            if let Some(data) = data {
                println!("Retrieved data for symbol '{}':", symbol);
                for point in data {
                    println!(
                        "Timestamp: {}\nOpening Price: {:.2}\nClosing Price: {:.2}\nHighest Price: {:.2}\nLowest Price: {:.2}\nVolume: {}\n",
                        point.timestamp.to_rfc3339(),
                        point.open,
                        point.close,
                        point.high,
                        point.low,
                        point.volume
                    );
                }
            } else {
                println!("No data found for symbol '{}'.", symbol);
            }
            continue; 
        }

        if input.eq_ignore_ascii_case("exit") {
            break; 
        }

        println!("Enter the asset symbol:");
        let mut symbol = String::new();
        io::stdin().read_line(&mut symbol).expect("Failed to read line");
        let symbol = symbol.trim();

        println!("Enter the timestamp (YYYY-MM-DD HH:MM:SS):");
        let mut timestamp_input = String::new();
        io::stdin().read_line(&mut timestamp_input).expect("Failed to read line");

        let naive_datetime = NaiveDateTime::parse_from_str(timestamp_input.trim(), "%Y-%m-%d %H:%M:%S")
            .expect("Invalid timestamp format");
        let timestamp: DateTime<Utc> = Utc.from_utc_datetime(&naive_datetime);

        println!("Enter the opening price:");
        let mut open_input = String::new();
        io::stdin().read_line(&mut open_input).expect("Failed to read line");
        let open: f64 = open_input.trim().parse().expect("Invalid number format");

        println!("Enter the closing price:");
        let mut close_input = String::new();
        io::stdin().read_line(&mut close_input).expect("Failed to read line");
        let close: f64 = close_input.trim().parse().expect("Invalid number format");

        println!("Enter the highest price:");
        let mut high_input = String::new();
        io::stdin().read_line(&mut high_input).expect("Failed to read line");
        let high: f64 = high_input.trim().parse().expect("Invalid number format");

        println!("Enter the lowest price:");
        let mut low_input = String::new();
        io::stdin().read_line(&mut low_input).expect("Failed to read line");
        let low: f64 = low_input.trim().parse().expect("Invalid number format");

        println!("Enter the volume:");
        let mut volume_input = String::new();
        io::stdin().read_line(&mut volume_input).expect("Failed to read line");
        let volume: u64 = volume_input.trim().parse().expect("Invalid number format");

        tsdb.add_data(symbol.to_string(), timestamp, open, close, high, low, volume);
        println!("Data for {} added successfully.", symbol);
        
        println!("Current data: {:?}", tsdb);
    }

    // Save the data to a file at the end of the session
    if let Err(e) = tsdb.save_to_file("financial_data.json") {
        eprintln!("Error saving data: {}", e);
    } else {
        println!("Data saved successfully to financial_data.json.");
    }
}
