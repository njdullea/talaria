use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::record;

pub struct Talaria {
    exchange_fees: HashMap<String, f64>,
    exchange_prices: HashMap<String, f64>,
}

// Max exchange and value, then min exchange and value.
type ExchangePair = ((String, f64), (String, f64));

impl Talaria {
    pub fn new() -> Self {
        // Why won't Hashmap::from work?
        let mut exchange_fees: HashMap<String, f64> = HashMap::new();
        exchange_fees.insert("coinbase".to_owned(), 0.1 / 100_f64);
        exchange_fees.insert("binance".to_owned(), 0.1 / 100_f64);
        exchange_fees.insert("kraken".to_owned(), 0.26 / 100_f64);

        Self {
            exchange_fees,
            exchange_prices: HashMap::new(),
        }
    }

    pub fn update_exchange_price(&mut self, exchange: String, price: f64) {
        self.exchange_prices.insert(exchange, price);
    }

    pub fn find_best_trade_pair(&mut self) -> Option<ExchangePair> {
        let mut max_exchange: Option<(String, f64)> = None;
        let mut min_exchange: Option<(String, f64)> = None;

        for (exchange, price) in self.exchange_prices.iter() {
            let value = price.to_owned();

            // If there is no max exchange name, none of the values have been set yet.
            if max_exchange.is_none() {
                max_exchange = Some((exchange.to_owned(), value));
                min_exchange = Some((exchange.to_owned(), value));
                continue;
            }

            if let Some((_, v)) = &max_exchange {
                if value > v.to_owned() {
                    max_exchange = Some((exchange.to_owned(), value));
                    continue;
                }
            }

            if let Some((_, v)) = &min_exchange {
                if value < v.to_owned() {
                    min_exchange = Some((exchange.to_owned(), value));
                    continue;
                }
            }
        }

        if max_exchange.is_some() && min_exchange.is_some() {
            let max_exchange_info = max_exchange.unwrap();
            let min_exchange_info = min_exchange.unwrap();

            // Confirm they are not the same exchange.
            if max_exchange_info.0 != min_exchange_info.0 {
                return Some((max_exchange_info, min_exchange_info));
            }
        }

        None
    }

    pub fn check_for_trade_and_value(&mut self) -> Option<(ExchangePair, f64)> {
        match self.find_best_trade_pair() {
            Some((max_exchange, min_exchange)) => {
                let price_diff = max_exchange.1 - min_exchange.1;
                let max_exchange_fee = self.exchange_fees.get(&max_exchange.0).unwrap();
                let min_exchange_fee = self.exchange_fees.get(&min_exchange.0).unwrap();

                let fee_1unit =
                    (max_exchange.1 * max_exchange_fee) + (min_exchange.1 * min_exchange_fee);
                if price_diff > fee_1unit {
                    return Some(((max_exchange, min_exchange), price_diff));
                }

                return None;
            }
            None => return None,
        }
    }

    pub fn backtest_v2(&mut self) {}

    pub fn backtest(&mut self) {
        let coinbase_records = record::read_records_from_file("data/ATOM-USD-KuCoin.txt");
        let binance_records = record::read_records_from_file("data/ATOM-USD-Binance.txt");

        let mut coinbase_funds = 500_f64;
        let coinbase_coins = 1000_f64;
        let mut binance_funds = 500_f64;
        let binance_coins = 1000_f64;

        let mut trade_qty: f64;
        let mut tlr = Talaria::new();

        let mut line_num = 0;

        // Make BTreeMap (sorted hashamp) and go through each data set to sort by datetime. Use struct with each exchange and Option?

        for (coinbase_record, binance_record) in coinbase_records.iter().zip(binance_records.iter())
        {
            if coinbase_record.date != binance_record.date {
                println!(
                    "Not same time {:?}, {:?}, {:?}",
                    line_num, coinbase_record.date, binance_record.date
                )
            }
            line_num = line_num + 1;
            assert_eq!(coinbase_record.date, binance_record.date);

            tlr.update_exchange_price("coinbase".to_owned(), coinbase_record.close);
            tlr.update_exchange_price("binance".to_owned(), binance_record.close);

            match tlr.check_for_trade_and_value() {
                Some(((max_exchange, min_exchange), _price_diff)) => {
                    // println!("Price diff: {:?}", price_diff);
                    let max_exchange_fee =
                        tlr.exchange_fees.get(&max_exchange.0).unwrap().to_owned();
                    let min_exchange_fee =
                        tlr.exchange_fees.get(&min_exchange.0).unwrap().to_owned();

                    if max_exchange.0 == "coinbase".to_owned() {
                        trade_qty =
                            f64::min((binance_funds * 0.8) / min_exchange.1, coinbase_coins);

                        let coinbase_cost = max_exchange.1 * trade_qty;
                        let coinbase_fee = coinbase_cost * max_exchange_fee;
                        let coinbase_total = coinbase_cost + coinbase_fee;

                        let binance_cost = min_exchange.1 * trade_qty;
                        let binance_fee = binance_cost * min_exchange_fee;
                        let binance_total = binance_cost + binance_fee;

                        if (coinbase_coins >= trade_qty)
                            && (binance_funds > binance_total)
                            && (trade_qty > 1_f64)
                        {
                            // sell on coinbase
                            coinbase_funds = coinbase_funds + coinbase_total;

                            // sell on binance
                            binance_funds = binance_funds - binance_total;

                            // pretend we send coins from one wallet to the other to balance out.
                            println!(
                                "DT {:?}, Sell CB {:?}, buy on BN {:?}, and Total: {:?}",
                                coinbase_record.date,
                                coinbase_funds,
                                binance_funds,
                                coinbase_funds + binance_funds
                            );
                        }
                    } else if max_exchange.0 == "binance".to_owned() {
                        trade_qty =
                            f64::min((coinbase_funds * 0.8) / min_exchange.1, binance_coins);
                        let binance_cost = max_exchange.1 * trade_qty;
                        let binance_fee = binance_cost * max_exchange_fee;
                        let binance_total = binance_cost + binance_fee;

                        let coinbase_cost = min_exchange.1 * trade_qty;
                        let coinbase_fee = coinbase_cost * min_exchange_fee;
                        let coinbase_total = coinbase_cost + coinbase_fee;

                        if (binance_coins >= trade_qty)
                            && (coinbase_funds > coinbase_total)
                            && (trade_qty > 1_f64)
                        {
                            // buy on coinbase
                            coinbase_funds = coinbase_funds - coinbase_total;

                            // sell on binance
                            binance_funds = binance_funds + binance_total;

                            // pretend we send coins from one wallet to the other to balance coin totals.
                            println!(
                                "DT {:?}, Sell on BN {:?}, buy on CB {:?}, and Total: {:?}",
                                coinbase_record.date,
                                binance_funds,
                                coinbase_funds,
                                coinbase_funds + binance_funds
                            );
                        }
                    }
                }
                None => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test_talaria() {
        let mut tlr = Talaria::new();
        tlr.update_exchange_price("exchange1".to_owned(), 1.0);
        tlr.update_exchange_price("exchange2".to_owned(), 1.1);
        tlr.update_exchange_price("exchange3".to_owned(), 1.2);
        tlr.update_exchange_price("exchange1".to_owned(), 1.3);

        let (max_exchange, min_exchange) = tlr.find_best_trade_pair().unwrap();
        assert_eq!(max_exchange, ("exchange1".to_owned(), 1.3));
        assert_eq!(min_exchange, ("exchange2".to_owned(), 1.1));
    }

    #[test]
    fn backtest_talaria() {
        let mut tlr = Talaria::new();
        tlr.backtest();
        // let coinbase_records = record::read_records_from_file("data/XLM-USD-Coinbase.txt");
        // let binance_records = record::read_records_from_file("data/XLM-USD-Binance.txt");

        // let mut coinbase_funds = 5000_f64;
        // let coinbase_coins = 10000_f64;
        // let mut binance_funds = 5000_f64;
        // let binance_coins = 10000_f64;

        // let mut trade_qty = 10000_f64;
        // let mut tlr = Talaria::new();

        // for (coinbase_record, binance_record) in coinbase_records.iter().zip(binance_records.iter()) {
        // 	assert_eq!(coinbase_record.date, binance_record.date);

        // 	 tlr.update_exchange_price("coinbase".to_owned(), coinbase_record.close);
        // 	 tlr.update_exchange_price("binance".to_owned(), binance_record.close);

        // 	match tlr.check_for_trade_and_value() {
        // 		Some(((max_exchange, min_exchange), price_diff)) => {
        // 			// println!("Price diff: {:?}", price_diff);
        // 			let max_exchange_fee = tlr.exchange_fees.get(&max_exchange.0).unwrap().to_owned();
        // 			let min_exchange_fee = tlr.exchange_fees.get(&min_exchange.0).unwrap().to_owned();

        // 			if max_exchange.0 == "coinbase".to_owned() {
        // 				trade_qty = f64::min((binance_funds * 0.8) / min_exchange.1, coinbase_coins);

        // 				let coinbase_cost = max_exchange.1 * trade_qty;
        // 				let coinbase_fee = coinbase_cost * max_exchange_fee;
        // 				let coinbase_total = coinbase_cost + coinbase_fee;

        // 				let binance_cost = min_exchange.1 * trade_qty;
        // 				let binance_fee = binance_cost * min_exchange_fee;
        // 				let binance_total = binance_cost + binance_fee;

        // 				if (coinbase_coins >= trade_qty) && (binance_funds > binance_total) && (trade_qty > 1_f64) {
        // 					// sell on coinbase
        // 					coinbase_funds = coinbase_funds + coinbase_total;

        // 					// sell on binance
        // 					binance_funds = binance_funds - binance_total;

        // 					// pretend we send coins from one wallet to the other to balance out.
        // 					println!("CB, BN, and Total: {:?}, {:?}, {:?}", coinbase_funds, binance_funds, coinbase_funds + binance_funds);
        // 				}
        // 			} else if max_exchange.0 == "binance".to_owned() {
        // 				trade_qty = f64::min((coinbase_funds * 0.8) / min_exchange.1, binance_coins);
        // 				let binance_cost = max_exchange.1 * trade_qty;
        // 				let binance_fee = binance_cost * max_exchange_fee;
        // 				let binance_total = binance_cost + binance_fee;

        // 				let coinbase_cost = min_exchange.1 * trade_qty;
        // 				let coinbase_fee = coinbase_cost * min_exchange_fee;
        // 				let coinbase_total = coinbase_cost + coinbase_fee;

        // 				if (binance_coins >= trade_qty) && (coinbase_funds > coinbase_total) && (trade_qty > 1_f64) {
        // 					// buy on coinbase
        // 					coinbase_funds = coinbase_funds - coinbase_total;

        // 					// sell on binance
        // 					binance_funds = binance_funds + binance_total;

        // 					// pretend we send coins from one wallet to the other to balance coin totals.
        // 					println!("BN, CB, and Total: {:?}, {:?}, {:?}", binance_funds, coinbase_funds, coinbase_funds + binance_funds);
        // 				}
        // 			}
        // 		},
        // 		None => {},
        // 	}
        // }
    }
}
