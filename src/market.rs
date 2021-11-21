use crate::local_env;
use crate::record;
use std::time::SystemTime;
use std::vec;
use serde::{de::Error, Deserialize, Deserializer};
use ta::DataItem;
use reqwest;
use binance::api::*;
use binance::config;
use binance::market::*;
use chrono::Duration;
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
        .checked_sub_signed(Duration::weeks(4))
        .unwrap();

    let mut end = start.clone()
        .checked_add_signed(Duration::minutes(300))
        .unwrap();

    // Kraken does not have any limits on how much OHLC data in one request.
    // Additionally, Kraken records go right up 
    let kraken_records = get_kraken_data(start).unwrap();
    record::save_records_to_file("data/XLM-USD-Kraken.txt", kraken_records);

    // Coinbase and Binance do have limits on how much OHLC data in one request.
    let mut coinbase_records: Vec<record::Record> = vec![];
    let mut binance_records: Vec<record::Record> = vec![];

    // TODO: this goes close to now, not right up to now leaving a little bit of data behind.
    while end.timestamp_nanos() < now.timestamp_nanos() {
        let coinbase_klines = coinbase_client.get_candles(
            "XLM-USD",
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
                volume: f.5
            });
        });

        let binance_klines = binance_market.get_klines(
            "XLMUSD",
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

        start = start
            .checked_add_signed(Duration::minutes(300))
            .unwrap();
        end = end
            .checked_add_signed(Duration::minutes(300))
            .unwrap()
    };

    record::save_records_to_file("data/XLM-USD-Coinbase.txt", coinbase_records);
    record::save_records_to_file("data/XLM-USD-Binance.txt", binance_records);

    Ok(())
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
