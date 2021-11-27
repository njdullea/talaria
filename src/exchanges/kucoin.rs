use crate::traits::Exchange;
use std::{fmt::Display, str::FromStr, sync::mpsc};
use crate::record;
use chrono::{DateTime, Duration, Utc, naive::serde::ts_nanoseconds::deserialize};
use std::time::SystemTime;
use serde::{de::Error, Deserialize, Deserializer};

pub struct KuCoinExchange;

impl Exchange for KuCoinExchange {
	fn save_testing_data(days: i64) -> Result<(), Box<dyn std::error::Error>> {
		let base_url = "https://api.kucoin.com";
		Ok(())
	}

    fn subscribe_to_data(tx: mpsc::Sender<record::Record>) {

	}
}

fn get_kline_data(start: DateTime<chrono::Utc>, end: DateTime<chrono::Utc>) -> Result<Vec<record::Record>, Box<dyn std::error::Error>> {
    let start_seconds = start.timestamp().to_string();
	let end_seconds = end.timestamp().to_string();

	let mut url = String::from("https://api.kucoin.com/api/v1/market/candles?symbol=ATOM-USDT&startAt=");

    url.push_str(start_seconds.as_str());
	url.push_str("&endAt=");
	url.push_str(end_seconds.as_str());
	url.push_str("&type=1min");

	let json: KuCoinResponse = reqwest::blocking::get(url)?.json()?;
	println!("KuCoin json: {:?}", json);

    let mut records: Vec<record::Record> = vec![];

    // for kline in json.result.xatomzusd.iter() {
    //     let record = record::Record {
    //         date: kline.0.to_string(),
    //         open: kline.1,
    //         high: kline.2,
    //         low: kline.3,
    //         close: kline.4,
    //         volume: kline.6,
    //     };
    //     records.push(record);
    // }

    Ok(records)
}

// start time, open, close, high, low, transaction volume, transaction amount
#[derive(Deserialize, Debug)]
struct KuCoinKLine<T> (
	#[serde(deserialize_with = "num_from_string")]
	#[serde(bound(deserialize = "T: FromStr, T::Err: Display"))]
	T,
	// #[serde(deserialize_with = "u64_from_string")]
	// u64,
	#[serde(deserialize_with = "f64_from_string")]
	f64,
	#[serde(deserialize_with = "f64_from_string")]
	f64,
	#[serde(deserialize_with = "f64_from_string")]
	f64,
	#[serde(deserialize_with = "f64_from_string")]
	f64,
	#[serde(deserialize_with = "f64_from_string")]
	f64,
	#[serde(deserialize_with = "f64_from_string")]
	f64,
);

fn num_from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: FromStr,
	T::Err: Display,
{
	let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<T>().map_err(D::Error::custom)
}

fn u64_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
	D: Deserializer<'de>
{
	let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(D::Error::custom)
}

fn f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(D::Error::custom)
}

#[derive(Deserialize, Debug)]
struct KuCoinResponse {
	#[serde(deserialize_with = "u64_from_string")]
	code: u64,
	data: Vec<KuCoinKLine<u64>>
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_kucoin_klines() {
		let system_time = SystemTime::now();
		// let now = DateTime::<Utc>::from(system_time);
		let start = DateTime::<Utc>::from(system_time)
			.checked_sub_signed(Duration::minutes(5))
			.unwrap();
	
		let end = start.clone()
			.checked_add_signed(Duration::minutes(4))
			.unwrap();
		
		let r = get_kline_data(start, end);
		match r {
			Ok(_) => println!("Okay!"),
			Err(e) => println!("Err! {:?}", e), 
		}
	}
}

