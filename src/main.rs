mod exchanges;
mod record;
mod talaria;
mod traits;
mod utilities;

use core::panic;
use exchanges::kucoin;
use flume::{Receiver, Sender};
use std::env;
use time_range::TimeRange;
use traits::Exchange;
use utilities::{local_env, parse, time_range};

use std::thread;

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
    // TODO: convert channel to result with record so we can match on ok or err and if err cancel the program
    let (record_tx, record_rx): (Sender<Result<record::Record, &'static str>>, Receiver<Result<record::Record, &'static str>>) =
        flume::bounded(32);

    let record_tx1 = record_tx.clone();
    // TODO: something is ending the kucoin thread. I'm guessing it is ping messages not being turned into json properly.
    // 1. Need to figure out why something in thread failing doesnt stop main.
    // 2. Need to find whats breaking in the thread and fix it.
    thread::spawn(move || {
        let err = exchanges::kucoin::KuCoinExchange::subscribe_to_data(record_tx1);
        match err {
            Ok(_) => {}
            // TODO: include e in error
            // TODO: having a panic doesn't seem to be ending the program properly?
            Err(_e) => {
                println!("Error in the kucoin thread we should end!!!!!!!!!!!!!!!!!");
                std::process::exit(1);
            },
        }
    });

    thread::spawn(move || {
        let err = exchanges::binance::BinanceExchange::subscribe_to_data(record_tx);
        match err {
            Ok(_) => {}
            // TODO: include e in error
            Err(_e) => {
                println!("Error in the binance thread we should end!!!!!!!!!!!!!!!!");
                std::process::exit(1);
            },
        }
    });

    let mut tlr = talaria::Talaria::new();
    for received in record_rx {
        match received {
            Ok(record) => {
                println!("Datetime: {:?}", record.date);
                tlr.update_exchange_price(record.exchange.to_string(), record.close);
                let trade_option = tlr.check_for_trade_and_value();

                match trade_option {
                    Some(((max_exchange, min_exchange), price_diff)) => {
                        let max_exchange_fee = tlr.exchange_fees.get(&max_exchange.0).unwrap().to_owned();
                        let min_exchange_fee = tlr.exchange_fees.get(&min_exchange.0).unwrap().to_owned();

                        // buy on the low sell on the high
                        let fees =
                            (max_exchange.1 * max_exchange_fee) + (min_exchange.1 * min_exchange_fee);
                        if price_diff > fees {
                            let profit = price_diff - fees;
                            println!(
                                "Buy on: {:?}, sell on: {:?}, price diff: {:?}, fees: {:}, and profit {:?}",
                                max_exchange, min_exchange, price_diff, fees, profit
                            );
                        }
                    }
                    None => {}
                }
            },
            Err(e) => {
                println!("Error while executing talaria. Exiting. Error: {:?}", e);
                std::process::exit(1);
            }
        }
    }
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
