use crate::local_env;
// use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use binance::model::Kline;
use ta::DataItem;

use binance::api::*;
use binance::config;
use binance::market::*;
use binance::model::KlineSummary;
use coinbase_pro_rs::{Public, MAIN_URL, Sync};
use coinbase_pro_rs::structs::{DateTime};
use coinbase_pro_rs::structs::public::{Candle};
use chrono::{Duration};

pub trait Trade {
    fn trade(&mut self, data_item: DataItem) -> Option<MarketAction>;
}

#[derive(Debug)]
pub enum MarketAction {
    Buy,
    Sell,
}

pub fn setup_testing_data() -> Result<(), Box<dyn std::error::Error>> {
    local_env::setup_local_env();
    let binance_api_key = local_env::get_env_var("BINANCE_US_API_KEY");
    let binance_secret_key = local_env::get_env_var("BINANCE_US_SECRET_KEY");

    let system_time = SystemTime::now();
    let since_the_epoch = system_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // coinbase won't let you request more than 300 items at once
    let milli_in_period = 60000 * 300; // minute
    // let milli_in_period = 604800000; // week

    let now = DateTime::from(system_time);
    let end = DateTime::from(system_time);
    let start = DateTime::from(system_time).checked_sub_signed(Duration::milliseconds(milli_in_period)).unwrap();

    // let coinbase_end = Some(DateTime::from(system_time));
    // let coinbase_start = DateTime::from(system_time).checked_sub_signed(Duration::milliseconds(milli_in_period));
    // println!("Coinbase start and end: {:?} {:?}", coinbase_start, coinbase_end);

    // let binance_end: u64 = since_the_epoch.as_millis() as u64;
    // let binance_start: u64 = binance_end - milli_in_period as u64;
    // println!("Binance start and end: {:?} {:?}", binance_start, binance_end);

    let client: Public<Sync> = Public::new(MAIN_URL);
    // let mut coinbase_klines = client.get_candles("XLM-USD", coinbase_start, coinbase_end, coinbase_pro_rs::structs::public::Granularity::M1)?;

    let api_endpoint = "https://api.binance.us";
    let config = config::Config::default().set_rest_api_endpoint(api_endpoint);
        // .set_ws_endpoint(ws_endpoint);
    let market: Market = Binance::new_with_config(binance_api_key, binance_secret_key, &config);

    get_testing_data(client, market, start, end);

    // let api_endpoint = "https://api.binance.us";
    // let config = config::Config::default().set_rest_api_endpoint(api_endpoint);
    //     // .set_ws_endpoint(ws_endpoint);
    // let market: Market = Binance::new_with_config(binance_api_key, binance_secret_key, &config);

    // let binance_klines = market.get_klines("XLMUSD", "1m", None, Some(binance_start), Some(binance_end))?;

    // match binance_klines {
    //     binance::model::KlineSummaries::AllKlineSummaries(klines) => {
    //         for b_kline in klines {
    //             // The time here is there same as the open on the binance kline
    //             let cb_kline: Option<Candle> = coinbase_klines.pop();

    //             let time = b_kline.close_time;
    //             let binance_close = b_kline.close;
    //             let coinbase_close = cb_kline.unwrap().4;

    //             println!("Close at this time: {:?}", time);
    //             let diff = (binance_close - coinbase_close).abs();
    //             // println!("Closes: {:?} {:?}", binance_close, coinbase_close);
                
    //             if diff > 0.001 {
    //                 println!("WE FOUND AN OP WE FOUND AN OP WE FOUND AN OP: {:?}", diff);
    //             }
    //         }
    //     }
    // }

    Ok(())
}

fn get_testing_data(coinbase_client: Public<Sync>, binance_market: Market, start: DateTime, end: DateTime) -> Result<(), Box<dyn std::error::Error>> {
    let mut coinbase_klines = coinbase_client.get_candles("XLM-USD", Some(start), Some(end), coinbase_pro_rs::structs::public::Granularity::M1)?;

    let binance_klines = binance_market.get_klines("XLMUSD", "1m", None, start.timestamp_millis() as u64, end.timestamp_millis() as u64)?;

    match binance_klines {
        binance::model::KlineSummaries::AllKlineSummaries(klines) => {
            for b_kline in klines {
                // The time here is there same as the open on the binance kline
                let cb_kline: Option<Candle> = coinbase_klines.pop();

                let time = b_kline.close_time;
                let binance_close = b_kline.close;
                let coinbase_close = cb_kline.unwrap().4;

                println!("Close at this time: {:?}", time);
                let diff = (binance_close - coinbase_close).abs();
                // println!("Closes: {:?} {:?}", binance_close, coinbase_close);
                
                if diff > 0.001 {
                    println!("WE FOUND AN OP WE FOUND AN OP WE FOUND AN OP: {:?}", diff);
                }
            }
        }
    }
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
