use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    adj_close: f64,
}
