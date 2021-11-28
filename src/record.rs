use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub date: u64,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

// Will overwrite an existing file.
pub fn save_records_to_file(file_path: &str, records: Vec<Record>) {
    let path = Path::new(file_path);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match fs::File::create(file_path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let content = serde_json::to_string(&records).unwrap();

    // Write the string to `file`, returns `io::Result<()>`
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

pub fn read_records_from_file(file_path: &str) -> Vec<Record> {
    let content = fs::read_to_string(file_path).unwrap();

    let records: Vec<Record> = serde_json::from_str(content.as_str()).unwrap();

    records
}
