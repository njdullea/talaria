use crate::local_env;
use crate::record;
use binance::api::*;
use binance::config;
use binance::market::*;
use binance::websockets::*;
use chrono::Duration;
use coinbase_pro_rs::structs::DateTime;
use coinbase_pro_rs::{Public, Sync, MAIN_URL};
use reqwest;
use serde::{de::Error, Deserialize, Deserializer};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::time::SystemTime;
use std::vec;

// time, open, high, low, close, vwap, volume, count
#[derive(Deserialize, Debug)]
struct KrakenKLine(
    u64,
    #[serde(deserialize_with = "f64_from_string")] f64,
    #[serde(deserialize_with = "f64_from_string")] f64,
    #[serde(deserialize_with = "f64_from_string")] f64,
    #[serde(deserialize_with = "f64_from_string")] f64,
    #[serde(deserialize_with = "f64_from_string")] f64,
    #[serde(deserialize_with = "f64_from_string")] f64,
    u64,
);

fn f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(D::Error::custom)
}

// #[serde(deny_unknown_fields)]
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

pub fn save_exchange_data() -> Result<(), Box<dyn std::error::Error>> {
    local_env::setup_local_env();
    let binance_api_key = local_env::get_env_var("BINANCE_US_API_KEY");
    let binance_secret_key = local_env::get_env_var("BINANCE_US_SECRET_KEY");
    let api_endpoint = "https://api.binance.us";
    let config = config::Config::default().set_rest_api_endpoint(api_endpoint);
    let binance_market: Market =
        Binance::new_with_config(binance_api_key, binance_secret_key, &config);
    let coinbase_client: Public<Sync> = Public::new(MAIN_URL);

    let system_time = SystemTime::now();
    let now = DateTime::from(system_time);
    let mut start = DateTime::from(system_time)
        .checked_sub_signed(Duration::weeks(4))
        .unwrap();

    let mut end = start
        .clone()
        .checked_add_signed(Duration::minutes(300))
        .unwrap();

    // Kraken does not have any limits on how much OHLC data in one request.
    // Additionally, Kraken records go right up
    let kraken_records = get_kraken_data(start).unwrap();
    record::save_records_to_file("data/ATOM-USD-Kraken.txt", kraken_records);

    // Coinbase and Binance do have limits on how much OHLC data in one request.
    let mut coinbase_records: Vec<record::Record> = vec![];
    let mut binance_records: Vec<record::Record> = vec![];

    // TODO: this goes close to now, not right up to now leaving a little bit of data behind.
    while end.timestamp_nanos() < now.timestamp_nanos() {
        let coinbase_klines = coinbase_client.get_candles(
            "ATOM-USD",
            Some(start.clone()),
            Some(end.clone()),
            coinbase_pro_rs::structs::public::Granularity::M1,
        )?;

        // TODO: there is something wack going on with the order of coinbase data!
        coinbase_klines.into_iter().rev().for_each(|f| {
            coinbase_records.push(record::Record {
                date: f.0.to_string(),
                open: f.3,
                close: f.4,
                high: f.2,
                low: f.1,
                volume: f.5,
            });
        });

        let binance_klines = binance_market.get_klines(
            "ATOMUSD",
            "1m",
            None,
            start.timestamp_millis() as u64,
            end.timestamp_millis() as u64,
        )?;

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

        start = start.checked_add_signed(Duration::minutes(300)).unwrap();
        end = end.checked_add_signed(Duration::minutes(300)).unwrap()
    }

    record::save_records_to_file("data/ATOM-USD-Coinbase.txt", coinbase_records);
    record::save_records_to_file("data/ATOM-USD-Binance.txt", binance_records);

    Ok(())
}

fn get_kraken_data(start: DateTime) -> Result<Vec<record::Record>, Box<dyn std::error::Error>> {
    let seconds = start.timestamp().to_string();

    let mut url = String::from("https://api.kraken.com/0/public/OHLC?pair=ATOMUSD&since=");
    url.push_str(seconds.as_str());
    let json: KrakenResponse = reqwest::blocking::get(url)?.json()?;

    let mut records: Vec<record::Record> = vec![];

    for kline in json.result.xatomzusd.iter() {
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

pub fn subscribe_to_binance_klines(tx: mpsc::Sender<record::Record>) {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let kline: String = format!("{}", "bnbbtc@kline_1m");

    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Kline(kline_event) => {
                let is_final_bar = kline_event.kline.is_final_bar.clone();
                if is_final_bar {
                    let dp = record::Record {
                        date: (kline_event.event_time / 1000_u64).to_string(),
                        high: kline_event.kline.high.parse::<f64>().unwrap(),
                        low: kline_event.kline.low.parse::<f64>().unwrap(),
                        open: kline_event.kline.open.parse::<f64>().unwrap(),
                        close: kline_event.kline.close.parse::<f64>().unwrap(),
                        volume: kline_event.kline.volume.parse::<f64>().unwrap(),
                    };
                    tx.send(dp).unwrap();
                }
            }
            _ => (),
        };
        Ok(())
    });

    web_socket.connect(&kline).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        match e {
            err => {
                println!("Error: {:?}", err);
            }
        }
    }
    web_socket.disconnect().unwrap();
}

// pub fn subscribe_to_kraken_klines(tx: mpsc::Sender<record::Record>) {}
