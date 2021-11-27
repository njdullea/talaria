use crate::local_env;
use crate::record;
use crate::traits::Exchange;
use std::time::SystemTime;
use std::vec;
use binance::api::*;
use binance::config;
use binance::market::*;
use binance::websockets::*;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use chrono::Duration;

pub struct BinanceExchange;

impl Exchange for BinanceExchange {
	fn save_testing_data(days: i64) -> Result<(), Box<dyn std::error::Error>> {
		local_env::setup_local_env();
		let binance_api_key = local_env::get_env_var("BINANCE_US_API_KEY");
		let binance_secret_key = local_env::get_env_var("BINANCE_US_SECRET_KEY");
		
		let api_endpoint = "https://api.binance.us";
		let config = config::Config::default().set_rest_api_endpoint(api_endpoint);
		let binance_market: Market = Binance::new_with_config(binance_api_key, binance_secret_key, &config);
		
		let system_time = SystemTime::now();
		let now = chrono::DateTime::<chrono::Utc>::from(system_time);
		let mut start = chrono::DateTime::<chrono::Utc>::from(system_time)
			.checked_sub_signed(Duration::weeks(4))
			.unwrap();
		let mut end = start.clone()
			.checked_add_signed(Duration::minutes(300))
			.unwrap();
		
		let mut binance_records: Vec<record::Record> = vec![];

		// TODO: this goes close to now, not right up to now leaving a little bit of data behind.
		while end.timestamp_nanos() < now.timestamp_nanos() {
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
	
			start = start
				.checked_add_signed(Duration::minutes(300))
				.unwrap();
			end = end
				.checked_add_signed(Duration::minutes(300))
				.unwrap()
		};

		record::save_records_to_file("data/ATOM-USD-Binance.txt", binance_records);

		Ok(())
	}

	fn subscribe_to_data(tx: mpsc::Sender<record::Record>) {
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
}
