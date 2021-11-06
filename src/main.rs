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
    let mut rsi_trader = RSITrader::new(14).unwrap();
    match backtest(&mut rsi_trader, "data/AMZN.csv") {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Err"),
    }

    let mut fso_trader = FSOTrader::new(14).unwrap();
    match backtest(&mut fso_trader, "data/AMZN.csv") {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Err"),
    }

    let mut sso_trader = SSOTrader::new(14).unwrap();
    match backtest(&mut sso_trader, "data/AMZN.csv") {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Err"),
    }

    // let mut rsi_trader2 = RSITrader::new(14).unwrap();
    rsi_trader.reset();
    match backtest(&mut rsi_trader, "data/XLM-USD.csv") {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Err"),
    }

    // let mut fso_trader2 = FSOTrader::new(14).unwrap();
    fso_trader.reset();
    match backtest(&mut fso_trader, "data/XLM-USD.csv") {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Err"),
    }

    // let mut sso_trader2 = SSOTrader::new(14).unwrap();
    sso_trader.reset();
    match backtest(&mut sso_trader, "data/XLM-USD.csv") {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Err"),
    }
}

fn backtest(mut trader: impl Trade + Description, filename: &str) -> Result<(), Box<dyn Error>> {
    println!("Executing {:?}", trader.description());
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
