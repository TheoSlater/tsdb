// main.rs

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::io::{self};
use tsdb::FinancialTimeSeries;

fn main() {
    let mut tsdb = FinancialTimeSeries::new();

    println!("Welcome to the Financial Time Series Database!");

    loop {
        println!("\nEnter the asset symbol (or type 'exit' to finish):");
        let mut symbol = String::new();
        io::stdin().read_line(&mut symbol).expect("Failed to read line");
        let symbol = symbol.trim();

        if symbol.eq_ignore_ascii_case("exit") {
            break; 
        }

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

        // Add the collected data to the time series
        tsdb.add_data(symbol.to_string(), timestamp, open, close, high, low, volume);
        println!("Data for {} added successfully.", symbol);
    }

    // Example of saving to a file
    if let Err(e) = tsdb.save_to_file("financial_data.json") {
        eprintln!("Error saving data: {}", e);
    } else {
        println!("Data saved successfully to financial_data.json.");
    }
}
