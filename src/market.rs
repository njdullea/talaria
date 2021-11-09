use crate::local_env;
use std::borrow::Borrow;
use std::time::SystemTime;
use ta::DataItem;

use binance::api::*;
use binance::config;
use binance::market::*;
use chrono::Duration;
use coinbase_pro_rs::structs::public::Candle;
use coinbase_pro_rs::structs::DateTime;
use coinbase_pro_rs::{Public, Sync, MAIN_URL};

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
    let now = DateTime::from(system_time);
    let mut actual_start = DateTime::from(system_time)
        .checked_sub_signed(Duration::weeks(1))
        .unwrap();
    let mut actual_end = DateTime::from(system_time)
        .checked_sub_signed(Duration::weeks(1))
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

fn get_testing_data(
    coinbase_client: &Public<Sync>,
    binance_market: &Market,
    start: DateTime,
    end: DateTime,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut coinbase_klines = coinbase_client.get_candles(
        "XLM-USD",
        Some(start),
        Some(end),
        coinbase_pro_rs::structs::public::Granularity::M1,
    )?;

    let binance_klines = binance_market.get_klines(
        "XLMUSD",
        "1m",
        None,
        start.timestamp_millis() as u64,
        end.timestamp_millis() as u64,
    )?;

    // TODO: time to add kraken, see if there are any ops there.

    match binance_klines {
        binance::model::KlineSummaries::AllKlineSummaries(klines) => {
            for b_kline in klines {
                // The time here is there same as the open on the binance kline
                let cb_kline: Option<Candle> = coinbase_klines.pop();

                let binance_close = b_kline.close;
                let coinbase_close = cb_kline.unwrap().4;

                let diff = (binance_close - coinbase_close).abs();


                if diff > 0.001 {
                    println!("WE FOUND AN OP woth num of cents {:?}", diff * 100.0);
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
