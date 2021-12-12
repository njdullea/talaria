use crate::parse;
use crate::record;
use crate::{time_range::TimeRange, traits::Exchange};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::sync::mpsc;

pub struct KrakenExchange;

impl Exchange for KrakenExchange {
    // KuCoin has limit of 1500 data points per request.
    fn save_testing_data(time_range: TimeRange) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Kraken doesn't have end and just returns up to like 1500 records.
        // Need to handle iteration for timerange with this constraint.
        let records = get_kline_data(time_range.start).unwrap();
        record::save_records_to_file("data/ATOM-USD-Kraken.txt", records);

        Ok(())
    }

    fn subscribe_to_data(
        _tx: flume::Sender<Result<record::Record, &'static str>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

fn get_kline_data(start: DateTime<Utc>) -> Result<Vec<record::Record>, Box<dyn std::error::Error>> {
    let seconds = start.timestamp().to_string();

    let mut url = String::from("https://api.kraken.com/0/public/OHLC?pair=ATOMUSD&since=");
    url.push_str(seconds.as_str());
    let json: KrakenResponse = reqwest::blocking::get(url)?.json()?;

    let mut records: Vec<record::Record> = vec![];

    for kline in json.result.xatomzusd.iter() {
        let record = record::Record {
            exchange: record::Exchange::Kraken,
            date: kline.0 as u64,
            open: kline.1,
            high: kline.2,
            low: kline.3,
            close: kline.4,
            volume: kline.6,
        };
        records.push(record);
    }

    Ok(records)
}

// time, open, high, low, close, vwap, volume, count
#[derive(Deserialize, Debug)]
struct KrakenKLine(
    u64,
    #[serde(deserialize_with = "parse::f64_from_string")] f64,
    #[serde(deserialize_with = "parse::f64_from_string")] f64,
    #[serde(deserialize_with = "parse::f64_from_string")] f64,
    #[serde(deserialize_with = "parse::f64_from_string")] f64,
    #[serde(deserialize_with = "parse::f64_from_string")] f64,
    #[serde(deserialize_with = "parse::f64_from_string")] f64,
    u64,
);

#[derive(Deserialize, Debug)]
struct KrakenResultStruct {
    #[serde(rename = "XATOMZUSD")]
    xatomzusd: Vec<KrakenKLine>,
    last: u64,
}

#[derive(Deserialize, Debug)]
struct KrakenResponse {
    error: Vec<String>,
    result: KrakenResultStruct,
}
