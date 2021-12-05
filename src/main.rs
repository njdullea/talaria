mod exchanges;
mod record;
mod talaria;
mod traits;
mod utilities;

use std::env;
use exchanges::kucoin;
use time_range::TimeRange;
use traits::Exchange;
use utilities::{local_env, parse, time_range};

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_config(&args).to_owned() {
        Ok(arg) => {
            if arg == "reset-data" {
                reset_data();
            } else if arg == "backtest" {
                execute_backtest();
            } else if arg == "talaria" {
                execute_talaria();
            } else if arg == "__dev__" {
                execute_dev();
            }
        }
        Err(err) => {
            println!("Error in parse config: {:?}", err);
        }
    }
}

fn parse_config(args: &[String]) -> Result<&str, &'static str> {
    match args.get(1) {
        None => Err("Error in main: Please provide an execution argument."),
        Some(e) => Ok(e),
    }
}

fn execute_dev() {
    let res = kucoin::KuCoinExchange::test_ws();
    match res {
        Ok(_) => println!("WS RES - OK!"),
        Err(e) => println!("WS RES - Error: {:?}", e.to_string()),
    }
}

fn execute_talaria() {
    // TODO: when we execute for real, we need to make sure that if either of the websockets fails
    // that way immediately stop everything else.
    println!("Would execute talaria now!");
}

fn execute_backtest() {
    let mut tlr = talaria::Talaria::new();
    tlr.backtest();
}

fn reset_data() {
    let tr = TimeRange::default();

    match exchanges::binance::BinanceExchange::save_testing_data(tr.clone()) {
        Ok(_) => println!("Binance - OK!"),
        Err(e) => println!("Binance - Error: {:?}", e),
    }

    // match exchanges::coinbase::CoinbaseExchange::save_testing_data(tr.clone()) {
    //     Ok(_) => println!("Coinbase - OK!"),
    //     Err(e) => println!("Coinbase - Error: {:?}", e),
    // }

    match exchanges::kucoin::KuCoinExchange::save_testing_data(tr.clone()) {
        Ok(_) => println!("KuCoin - OK!"),
        Err(e) => println!("KuCoin - Error: {:?}", e),
    }

    // TODO: 2. Update backtest to sort data and then go through only use datetimes.
    // where there is info from multiple exchanges.
}
