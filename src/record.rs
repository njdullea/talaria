use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Records {
    pub records: Vec<Record>
}
