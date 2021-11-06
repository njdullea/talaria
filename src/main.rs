
mod record;
mod traders;
mod market;

use std::error::Error;
use std::fs;
use ta::DataItem;

use crate::record::Record;
use crate::traders::relative_strength_index::RSITrader;
use crate::market::{Trade, MarketAction};

fn main() {
    let mut rsi_trader = RSITrader::new(14).unwrap();
    match backtest(&mut rsi_trader) {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Err"),
    }
}

fn backtest(mut trader: impl Trade) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string("data/AMZN.csv").expect("Something went wrong reading the file.");
    let mut rdr = csv::Reader::from_reader(contents.as_bytes());
    let mut stock_qty = 0.0;
    let trade_qty = 2.0;
    let mut fiat_total = 10000.0;
    println!("Starting stock qty: {:?}", stock_qty);
    println!("Starting fiat_total: {:?}", fiat_total);

    for record in rdr.deserialize() {
        let record: Record = record?;

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
                println!("Time to {:?}", action);
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

    println!("Ending stock qty: {:?}", stock_qty);
    println!("Ending fiat_total: {:?}", fiat_total);

    Ok(())
}
