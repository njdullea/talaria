use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    date: String,
    pub open: f64,
    pub close: f64,
	high: f64,
    low: f64,
	volume: f64,
	adj_close: f64,
}