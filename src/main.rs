use serde::Deserialize;
use std::error::Error;
use std::fs;
use ta::indicators::ExponentialMovingAverage;
use ta::Next;

#[derive(Debug, Deserialize, Clone)]
struct Record {
    date: String,
    open: f64,
    close: f64,
	high: f64,
    low: f64,
	volume: f64,
	adj_close: f64,
}

fn main() {
    let mut ema = ExponentialMovingAverage::new(3).unwrap();
    ema.next(2.0);
    ema.next(5.0);
    ema.next(1.0);
    let avg = ema.next(6.25);
    println!("Avg: {:?}", avg);
    match print_csv_data("data/AMZN.csv") {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Err"),
    }
}

fn print_csv_data(filename: &str) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");
    let mut rdr = csv::Reader::from_reader(contents.as_bytes());

	// let mut data_points = Vec::new();

    for record in rdr.deserialize() {
        let record: Record = record?;
        println!("Record: {:?}", record);
        // let data_point = DataPoint {
        //     open: record.open * 1000_f64,
        //     close: record.close * 1000_f64,
        //     volume: record.volume as f64,
        // };

		// data_points.insert(0, data_point);
    };

	Ok(())
}
