mod atalanta;
mod local_env;
mod market;
mod record;
mod traders;
mod traits;

use std::error::Error;
use std::fs;
use ta::DataItem;

use crate::market::{MarketAction, Trade};
use crate::record::Record;
use crate::traders::fast_stochastic_oscillator::FSOTrader;
use crate::traders::percentage_price_oscillator::PPOTrader;
use crate::traders::rsi::rsi_trader::RSITrader;
use crate::traders::slow_stochastic_oscillator::SSOTrader;
use crate::traits::Description;

fn main() {
    // match market::setup_testing_data() {
    //     Ok(_) => {}
    //     Err(e) => {
    //         println!("E: {:?}", e);
    //     }
    // }

    match market::save_exchange_data() {
        Ok(_) => {}
        Err(e) => {
            println!("Error saving exchange data: {:?}", e);
        }
    }

    // market::compage_exchange_prices();

    // backtest_indicator_traders();
}

fn backtest_indicator_traders() {
    let datasets = vec![
        // "data/AMZN.csv",
        // "data/PFE.csv",
        // "data/SHIB-USD.csv",
        // "data/XLM-USD.csv",
        // "data/ADA-USD.csv",
        "data/Binance_XLMUSDT_minute.csv",
        "data/Binance_ADAUSDT_minute.csv",
    ];
    let mut rsi_trader = RSITrader::default();
    let mut fso_trader = FSOTrader::default();
    let mut sso_trader = SSOTrader::default();
    let mut ppo_trader = PPOTrader::default();

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

        match backtest(&mut ppo_trader, dataset) {
            Ok(_) => println!("Ok"),
            Err(_) => println!("Err"),
        }

        rsi_trader.reset();
        fso_trader.reset();
        sso_trader.reset();
        ppo_trader.reset();
    }
}

fn backtest(
    mut trader: impl Trade + Description,
    filename: &str,
) -> Result<((f64, f64)), Box<dyn Error>> {
    println!(
        "Executing {:?} on dataset {:?}",
        trader.description(),
        filename
    );
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");
    let mut rdr = csv::Reader::from_reader(contents.as_bytes());
    let mut stock_qty = 0.0;
    let mut trade_qty = 20.0;
    let starting_fiat_total = 10000.0;
    let mut fiat_total = 10000.0;
    let mut last_price = 0.0;
    let mut first_item = true;
    let mut first_price = 0.0;

    for record in rdr.deserialize() {
        let record: Record = record?;
        last_price = record.close;

        if first_item {
            first_price = record.open;
            first_item = false;

            trade_qty = ((starting_fiat_total / 2.0) / first_price).floor();
        }

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
                // println!("Time to {:?}", action);
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

    let hodl_qty = (starting_fiat_total / first_price).floor();
    let hodl_fiat_rem = starting_fiat_total - (hodl_qty * first_price);
    let hodl_total_value = hodl_fiat_rem + (hodl_qty * last_price);
    println!("Final hodl value: {:?}", hodl_total_value);

    let trader_total_value = fiat_total + (stock_qty * last_price);
    println!("Final trading value: {:?}", trader_total_value);
    println!(
        "Gains from trading over hodling: {:?}",
        trader_total_value - hodl_total_value
    );

    Ok((trader_total_value, hodl_total_value))
}
