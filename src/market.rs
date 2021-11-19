use crate::local_env;
use crate::record;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::time::SystemTime;
use std::vec;
use binance::model::KlineEvent;
use serde::Serialize;
// use chrono::serde::ts_nanoseconds::deserialize;
// use serde::Deserialize;
use serde::{de::Error, Deserialize, Deserializer};
use ta::DataItem;
use reqwest;

use binance::api::*;
use binance::config;
use binance::market::*;
use chrono::Duration;
use coinbase_pro_rs::structs::public::Candle;
use coinbase_pro_rs::structs::DateTime;
use coinbase_pro_rs::{Public, Sync, MAIN_URL};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub trait Trade {
    fn trade(&mut self, data_item: DataItem) -> Option<MarketAction>;
}

#[derive(Debug)]
pub enum MarketAction {
    Buy,
    Sell,
}
// time, open, high, low, close, vwap, volume, count
#[derive(Deserialize, Debug)]
struct KrakenKLine (
    u64, 
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
    u64
);

impl KrakenKLine {
    fn new(time: u64, open: f64, high: f64, low: f64, close: f64, vwap: f64, volume: f64, count: u64) -> Self {
        KrakenKLine (time, open, high, low, close, vwap, volume, count)
    }
}

fn f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(D::Error::custom)
}

#[derive(Deserialize, Debug)]
struct KrakenResultStruct {
    XXLMZUSD: Vec<KrakenKLine>,
    last: u64,
}

#[derive(Deserialize, Debug)]
struct KrakenResponse {
    error: Vec<String>,
    result: KrakenResultStruct,
}

pub fn setup_testing_data() -> Result<(), Box<dyn std::error::Error>> {
    // let system_time = SystemTime::now();
    // let now = DateTime::from(system_time);
    // let minutes_ago = now.checked_sub_signed(Duration::minutes(5)).unwrap();
    // let seconds = minutes_ago.timestamp().to_string();

    // let mut url = String::from("https://api.kraken.com/0/public/OHLC?pair=XLMUSD&since=");
    // url.push_str(seconds.as_str());
    // let resp = reqwest::blocking::get(url)?;
    // // let text = resp.text()?;
    // // println!("{:#?}", text);

    // // TODO: combine kracken in with the other exchanges!
    // let json: KrakenResponse = resp.json()?;
    // println!("{:#?}", json);

    local_env::setup_local_env();
    let binance_api_key = local_env::get_env_var("BINANCE_US_API_KEY");
    let binance_secret_key = local_env::get_env_var("BINANCE_US_SECRET_KEY");

    let system_time = SystemTime::now();
    let now = DateTime::from(system_time);
    let mut actual_start = DateTime::from(system_time)
        .checked_sub_signed(Duration::weeks(5))
        .unwrap();
    let mut actual_end = DateTime::from(system_time)
        .checked_sub_signed(Duration::weeks(5))
        .unwrap()
        .checked_add_signed(Duration::minutes(300))
        .unwrap();

    let client: Public<Sync> = Public::new(MAIN_URL);

    let api_endpoint = "https://api.binance.us";
    let config = config::Config::default().set_rest_api_endpoint(api_endpoint);
    // .set_ws_endpoint(ws_endpoint);
    let market: Market = Binance::new_with_config(binance_api_key, binance_secret_key, &config);

    while actual_end.timestamp_nanos() < now.timestamp_nanos() {
        let _ = get_testing_data(client.borrow(), market.borrow(), actual_start, actual_end);
        actual_start = actual_start
            .checked_add_signed(Duration::minutes(300))
            .unwrap();
        actual_end = actual_end
            .checked_add_signed(Duration::minutes(300))
            .unwrap()
    }

    Ok(())
}

pub fn save_exchange_data() -> Result<(), Box<dyn std::error::Error>> {
    local_env::setup_local_env();
    let binance_api_key = local_env::get_env_var("BINANCE_US_API_KEY");
    let binance_secret_key = local_env::get_env_var("BINANCE_US_SECRET_KEY");
    let api_endpoint = "https://api.binance.us";
    let config = config::Config::default().set_rest_api_endpoint(api_endpoint);
    let binance_market: Market = Binance::new_with_config(binance_api_key, binance_secret_key, &config);
    let coinbase_client: Public<Sync> = Public::new(MAIN_URL);

    let system_time = SystemTime::now();
    let now = DateTime::from(system_time);
    let mut start = DateTime::from(system_time)
        .checked_sub_signed(Duration::hours(6))
        .unwrap();
    let mut end = DateTime::from(system_time)
        .checked_sub_signed(Duration::hours(6))
        .unwrap()
        .checked_add_signed(Duration::minutes(300))
        .unwrap();

    while end.timestamp_nanos() < now.timestamp_nanos() {
        let coinbase_klines = coinbase_client.get_candles(
            "XLM-USD",
            Some(start.clone()),
            Some(end.clone()),
            coinbase_pro_rs::structs::public::Granularity::M1,
        )?;
        
        let coinbase_path = Path::new("data/XLM-USD-Coinbase.txt");
        let coinbase_display = coinbase_path.display();
        
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut coinbase_file = match File::create(&coinbase_path) {
            Err(why) => panic!("couldn't create {}: {}", coinbase_display, why),
            Ok(file) => file,
        };

        let mut coinbase_records: Vec<record::Record> = vec![];
        coinbase_klines.into_iter().for_each(|f| {
            coinbase_records.insert(0, record::Record {
                date: f.0.to_string(),
                open: f.3,
                close: f.4,
                high: f.2,
                low: f.1,
                volume: f.5
            });
        });

        let coinbase_st = serde_json::to_string(&coinbase_records).unwrap();

        // Write the string to `file`, returns `io::Result<()>`
        match coinbase_file.write_all(coinbase_st.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", coinbase_display, why),
            Ok(_) => println!("successfully wrote to {}", coinbase_display),
        }

        // let mut binance_records: Vec<record::Record> = vec![];
        let binance_klines = binance_market.get_klines(
            "ADAUSD",
            "1m",
            None,
            start.timestamp_millis() as u64,
            end.timestamp_millis() as u64,
        )?;

        let binance_path = Path::new("data/XLM-USD-Binance.txt");
        let binance_display = binance_path.display();
        
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut binance_file = match File::create(&binance_path) {
            Err(why) => panic!("couldn't create {}: {}", binance_display, why),
            Ok(file) => file,
        };

        let mut binance_records: Vec<record::Record> = vec![];
        match binance_klines {
            binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                for kline in klines {
                    let record = record::Record {
                        // Convert milliseconds into seconds.
                        date: (kline.open_time / 1000).to_string(),
                        open: kline.open,
                        high: kline.high,
                        low: kline.low,
                        close: kline.close,
                        volume: kline.volume,
                    };
    
                    binance_records.push(record);
                }
            }
        }

        let binance_st = serde_json::to_string(&binance_records).unwrap();

        // Write the string to `file`, returns `io::Result<()>`
        match binance_file.write_all(binance_st.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", binance_display, why),
            Ok(_) => println!("successfully wrote to {}", binance_display),
        }

        start = start
            .checked_add_signed(Duration::minutes(300))
            .unwrap();
        end = end
            .checked_add_signed(Duration::minutes(300))
            .unwrap()
    };

    Ok(())
}

/*
    1. Saving exchange data sets to csv.
    2. Backtesting atalanta on each data set.
    3. Websocket connection passing updating atalanta.
*/

pub fn compare_exchange_prices() {
    let system_time = SystemTime::now();
    let now = DateTime::from(system_time);
    // When this is set over like 500 the times are getting out of sync.
    let minutes_ago = now.checked_sub_signed(Duration::minutes(500)).unwrap();

    local_env::setup_local_env();
    let binance_api_key = local_env::get_env_var("BINANCE_US_API_KEY");
    let binance_secret_key = local_env::get_env_var("BINANCE_US_SECRET_KEY");
    let api_endpoint = "https://api.binance.us";
    let config = config::Config::default().set_rest_api_endpoint(api_endpoint);
    let market: Market = Binance::new_with_config(binance_api_key, binance_secret_key, &config);

    let mut binance_data = get_binace_data(minutes_ago, &market).unwrap();
    // println!("Binance data: {:?}", binance_data);
    let mut kraken_data = get_kraken_data(minutes_ago).unwrap();
    // println!("Kraken data: {:?}", kraken_data);

    let binance_fee = 0.1 / 100_f64;
    let kraken_fee = 0.26 / 100_f64;
    // let average_fee = (binance_fee + kraken_fee) / 2.0;

    let trade_qty = 500_f64;

    for data in binance_data.iter_mut().zip(kraken_data.iter_mut()) {
        let (binance_record, kraken_record) = data;
        // println!("Dates: {:?} {:?}", binance_record.date, kraken_record.date);
        
        let diff = (binance_record.close - kraken_record.close).abs();
        // println!("Diff: {:?}", diff);
        
        // tenth of a cent;
        if diff > 0.001 {
            println!("Date time: {:?} {:?}", binance_record.date, kraken_record.date);
            let trades_cost = (binance_record.close * binance_fee * trade_qty) + (kraken_record.close * kraken_fee * trade_qty);
            println!("Trade cost: {:?}", trades_cost);
            let trades_gains = diff * trade_qty;

            let trade_profit = trades_gains - trades_cost;
            println!("Profit: {:?}", trade_profit);
        }
    }
}

fn get_binace_data(start: DateTime, binance_market: &Market) -> Result<Vec<record::Record>, Box<dyn std::error::Error>> {
    let binance_klines = binance_market.get_klines(
        "XLMUSD",
        "1m",
        None,
        start.timestamp_millis() as u64,
        None
    )?;

    let mut records: Vec<record::Record> = vec![];

    match binance_klines {
        binance::model::KlineSummaries::AllKlineSummaries(klines) => {
            for kline in klines {
                let record = record::Record {
                    date: kline.open_time.to_string(),
                    open: kline.open,
                    high: kline.high,
                    low: kline.low,
                    close: kline.close,
                    volume: kline.volume,
                };

                records.push(record);
            }
        }
    }

    Ok(records)
}

fn get_kraken_data(start: DateTime) -> Result<Vec<record::Record>, Box<dyn std::error::Error>> {
    let seconds = start.timestamp().to_string();

    let mut url = String::from("https://api.kraken.com/0/public/OHLC?pair=XLMUSD&since=");
    url.push_str(seconds.as_str());
    let json: KrakenResponse = reqwest::blocking::get(url)?.json()?;

    let mut records: Vec<record::Record> = vec![];

    for kline in json.result.XXLMZUSD.iter() {
        let record = record::Record {
            date: kline.0.to_string(),
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

fn get_testing_data(
    coinbase_client: &Public<Sync>,
    binance_market: &Market,
    start: DateTime,
    end: DateTime,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut coinbase_klines = coinbase_client.get_candles(
        "ADA-USD",
        Some(start),
        Some(end),
        coinbase_pro_rs::structs::public::Granularity::M1,
    )?;

    let binance_klines = binance_market.get_klines(
        "ADAUSD",
        "1m",
        None,
        start.timestamp_millis() as u64,
        end.timestamp_millis() as u64,
    )?;

    // TODO: time to add kraken, see if there are any ops there.
    
    // 0.25% of trade
    let coinbase_fee = 0.0025;

    // 0.1% of trade
    let binance_fee = 0.001;

    let mut total = 0.0;

    match binance_klines {
        binance::model::KlineSummaries::AllKlineSummaries(klines) => {
            for b_kline in klines {
                // The time here is there same as the open on the binance kline
                let cb_kline: Option<Candle> = coinbase_klines.pop();

                let binance_close = b_kline.close;
                let coinbase_close = cb_kline.unwrap().4;

                let trade_qty = 1000_f64;

                let diff = (binance_close - coinbase_close).abs() * trade_qty;
                let fee = (binance_close * binance_fee * trade_qty) + (coinbase_close * coinbase_fee * trade_qty);
                let profit = diff - fee;
                if profit > 0_f64 {
                    println!("Profit: {:}", profit);
                    total = total + profit;
                }
                // if diff > 0.01 {
                //     let fee_made = diff - fee;
                //     let mony_on_500 = fee_made * 5000_f64;
                //     total = total + mony_on_500;

                //     println!("WE FOUND AN OP woth num of cents {:?}, {:?}. {:?}", diff * 100.0, fee_made, mony_on_500);
                // }
            }
        }
    }

    println!("Total over this time: {:?} {:?} {:?}", total, start.timestamp_millis(), end.timestamp_millis());
    Ok(())
}

// pub fn get_coinbase_candlesticks() -> Result<(), Box<dyn std::error::Error>> {
//     let start = SystemTime::now();
//     let since_the_epoch = start
//         .duration_since(UNIX_EPOCH)
//         .expect("Time went backwards");
//     println!("{:?}", since_the_epoch);

//     println!("Executing get coinbase candlesticks");
//     let symbol = "XLM-USD";
//     let one_minute = 60;
//     let _one_day = 86400;
//     let mut url = String::from("https://api.pro.coinbase.com/products/");
//     url.push_str(symbol);
//     url.push_str("/candles?granularity=");
//     url.push_str(one_minute.to_string().as_str());
//     // TODO: add start and end timestamps

//     let resp = reqwest::blocking::get(url)?.text()?;
//     println!("{:#?}", resp);
//     Ok(())
// }

// pub fn get_binance_candlesticks() -> Result<(), Box<dyn std::error::Error>> {
//     let start = SystemTime::now();
//     let since_the_epoch = start
//         .duration_since(UNIX_EPOCH)
//         .expect("Time went backwards");
//     println!("{:?}", since_the_epoch);

//     println!("Executing get coinbase candlesticks");
//     let symbol = "XLM-USD";
//     let one_minute = 60;
//     let _one_day = 86400;
//     // let mut url = String::from("https://api.pro.coinbase.com/products/");
//     let url = String::from("https://api.binance.us/api/v3/klines?interval=5m&symbol=XLMUSD");

//     // let url = String::from("https://api.binance.us/api/v3/ticker/price");

//     // url.push_str(symbol);
//     // url.push_str("/candles?granularity=");
//     // url.push_str(one_minute.to_string().as_str());
//     // TODO: add start and end timestamps

//     let resp = reqwest::blocking::get(url)?.text()?;
//     println!("{:#?}", resp);
//     Ok(())
// }

// pub fn getBinanceCandleSticks() {}
