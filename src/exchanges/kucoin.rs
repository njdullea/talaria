use crate::parse;
use crate::record;
use crate::{time_range::TimeRange, traits::Exchange};
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use std::time::SystemTime;
use std::{fmt::Display, str::FromStr, sync::mpsc};

pub struct KuCoinExchange;

impl Exchange for KuCoinExchange {
    // KuCoin has limit of 1500 data points per request.
    fn save_testing_data(time_range: TimeRange) -> Result<(), Box<dyn std::error::Error>> {
        let mut records: Vec<record::Record> = vec![];

        time_range.for_each(|sr: TimeRange| {
            let mut new_records = get_kline_data(sr.start, sr.end).unwrap();
            new_records.reverse();
            records.append(&mut new_records);
        });

        //records.reverse();
        record::save_records_to_file("data/ATOM-USD-KuCoin.txt", records);

        Ok(())
    }

    fn subscribe_to_data(_tx: mpsc::Sender<record::Record>) {}
}

fn get_kline_data(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<record::Record>, Box<dyn std::error::Error>> {
    let start_seconds = start.timestamp().to_string();
    let end_seconds = end.timestamp().to_string();

    let mut url =
        String::from("https://api.kucoin.com/api/v1/market/candles?symbol=ATOM-USDT&startAt=");
    url.push_str(start_seconds.as_str());
    url.push_str("&endAt=");
    url.push_str(end_seconds.as_str());
    url.push_str("&type=1min");

    let json: KuCoinResponse = reqwest::blocking::get(url)?.json()?;
    let mut records: Vec<record::Record> = vec![];

    for kline in json.data {
        records.push(record::Record {
            date: kline.0 as u64,
            open: kline.1,
            close: kline.2,
            high: kline.3,
            low: kline.4,
            volume: kline.5,
        });
    }

    Ok(records)
}

// start time, open, close, high, low, transaction volume, transaction amount
#[derive(Deserialize, Debug)]
struct KuCoinKLine<T, U>(
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "T: FromStr, T::Err: Display"))]
    T,
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "U: FromStr, U::Err: Display"))]
    U,
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "U: FromStr, U::Err: Display"))]
    U,
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "U: FromStr, U::Err: Display"))]
    U,
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "U: FromStr, U::Err: Display"))]
    U,
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "U: FromStr, U::Err: Display"))]
    U,
    #[serde(deserialize_with = "parse::num_from_string")]
    #[serde(bound(deserialize = "U: FromStr, U::Err: Display"))]
    U,
);

#[derive(Deserialize, Debug)]
struct KuCoinResponse {
    #[serde(deserialize_with = "parse::u64_from_string")]
    code: u64,
    data: Vec<KuCoinKLine<u64, f64>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_kucoin_klines() {
        let system_time = SystemTime::now();
        let start = DateTime::<Utc>::from(system_time)
            .checked_sub_signed(Duration::minutes(5))
            .unwrap();

        let end = start
            .clone()
            .checked_add_signed(Duration::minutes(4))
            .unwrap();

        let r = get_kline_data(start, end);
        match r {
            Ok(_) => println!("Okay!"),
            Err(e) => println!("Err! {:?}", e),
        }
    }

    #[test]
    fn confirm_kucoin_lines_ordered() {
        let records = record::read_records_from_file("data/ATOM-USD-KuCoin.txt");
        let mut previous_dt = 0;

        for record in records {
            println!(
                "Previous Dt and current DT: {:?} {:?} ",
                previous_dt, record.date
            );
            assert!(record.date.gt(&previous_dt));
            previous_dt = record.date;
        }
    }
}
