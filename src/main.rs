mod record;
mod traders;
mod market;
mod traits;

use std::error::Error;
use std::fs;
use ta::DataItem;

use crate::traits::Description;
use crate::record::Record;
use crate::traders::relative_strength_index::RSITrader;
use crate::traders::fast_stochastic_oscillator::FSOTrader;
use crate::traders::slow_stochastic_oscillator::SSOTrader;
use crate::market::{Trade, MarketAction};

fn main() {
    let datasets = vec!["data/AMZN.csv", "data/SHIB-USD.csv", "data/XLM-USD.csv", "data/ADA-USD.csv"];
    let mut rsi_trader = RSITrader::new(14).unwrap();
    let mut fso_trader = FSOTrader::new(14).unwrap();
    let mut sso_trader = SSOTrader::new(14).unwrap();

    for dataset in datasets {
        match backtest(&mut rsi_trader, dataset) {
            Ok(_) => println!("Ok"),
            Err(_) => println!("Err"),
        }

        match backtest(&mut fso_trader, dataset) {
            Ok(_) => println!("Ok"),
            Err(_) => println!("Err"),
        }

        match backtest(&mut sso_trader, dataset) {
            Ok(_) => println!("Ok"),
            Err(_) => println!("Err"),
        }

        rsi_trader.reset();
        fso_trader.reset();
        sso_trader.reset();
    }
}

fn backtest(mut trader: impl Trade + Description, filename: &str) -> Result<(), Box<dyn Error>> {
    println!("Executing {:?} on dataset {:?}", trader.description(), filename);
    let contents =
        fs::read_to_string(filename).expect("Something went wrong reading the file.");
    let mut rdr = csv::Reader::from_reader(contents.as_bytes());
    let mut stock_qty = 0.0;
    let trade_qty = 2.0;
    let mut fiat_total = 10000.0;
    let mut last_price = 0.0;

    for record in rdr.deserialize() {
        let record: Record = record?;
        last_price = record.close;

        let data_item = DataItem::builder()
            .close(record.close)
            .open(record.open)
            .volume(record.volume)
            .high(record.high)
            .low(record.low)
            .build()
            .unwrap();

        match trader.trade(data_item) {
            Some(action) => {
                match action {
                    MarketAction::Buy => {
                        let cost = trade_qty * record.close;
                        fiat_total = fiat_total - cost;
                        stock_qty = stock_qty + trade_qty;
                    }
                    MarketAction::Sell => {
                        let cost = trade_qty * record.close;
                        fiat_total = fiat_total + cost;
                        stock_qty = stock_qty - trade_qty;
                    }
                }
            }
            None => (),
        }
    }

    let total_value = fiat_total + (stock_qty * last_price);
    println!("Final value: {:?}", total_value);

    Ok(())
}
