use crate::local_env;
use crate::record;
use crate::time_range;
use crate::traits::Exchange;
use binance::api::*;
use binance::config;
use binance::market::*;
use binance::websockets::*;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::vec;

pub struct BinanceExchange;

impl Exchange for BinanceExchange {
    fn save_testing_data(
        time_range: time_range::TimeRange,
    ) -> Result<(), Box<dyn std::error::Error>> {
        local_env::setup_local_env();
        let binance_api_key = local_env::get_env_var("BINANCE_US_API_KEY");
        let binance_secret_key = local_env::get_env_var("BINANCE_US_SECRET_KEY");

        let api_endpoint = "https://api.binance.us";
        let config = config::Config::default().set_rest_api_endpoint(api_endpoint);
        let binance_market: Market =
            Binance::new_with_config(binance_api_key, binance_secret_key, &config);

        let mut binance_records: Vec<record::Record> = vec![];

        for sub_range in time_range {
            let binance_klines = binance_market.get_klines(
                "ATOMUSD",
                "1m",
                None,
                sub_range.start.timestamp_millis() as u64,
                sub_range.end.timestamp_millis() as u64,
            )?;

            match binance_klines {
                binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                    for kline in klines {
                        let record = record::Record {
                            exchange: record::Exchange::Binance,
                            // Convert milliseconds into seconds.
                            date: (kline.open_time / 1000) as u64,
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
        }

        record::save_records_to_file("data/ATOM-USD-Binance.txt", binance_records);

        Ok(())
    }

    fn subscribe_to_data(
        tx: flume::Sender<Result<record::Record, &'static str>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let keep_running = AtomicBool::new(true); // Used to control the event loop
        let kline: String = format!("{}", "atomusdt@kline_1m");

        let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
            match event {
                WebsocketEvent::Kline(kline_event) => {
                    let dp = record::Record {
                        exchange: record::Exchange::Binance,
                        date: (kline_event.event_time) as u64,
                        high: kline_event.kline.high.parse::<f64>().unwrap(),
                        low: kline_event.kline.low.parse::<f64>().unwrap(),
                        open: kline_event.kline.open.parse::<f64>().unwrap(),
                        close: kline_event.kline.close.parse::<f64>().unwrap(),
                        volume: kline_event.kline.volume.parse::<f64>().unwrap(),
                    };
                    tx.send(Ok(dp)).unwrap();
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
                    tx.send(Err("Error in binance connection.")).unwrap();
                }
            }
        }
        web_socket.disconnect().unwrap();

        Ok(())
    }
}
