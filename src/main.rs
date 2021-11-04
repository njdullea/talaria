use std::error::Error;
use std::fs;
use ta::indicators::{RelativeStrengthIndex};
use ta::Next;

mod record;
use crate::record::Record;

#[derive(Debug)]
enum MarketActionType {
    Buy,
    Sell
}

fn main() {
    match backtest_rsi() {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Err"),
    }
}

fn backtest_rsi() -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string("data/AMZN.csv").expect("Something went wrong reading the file.");
    let mut rdr = csv::Reader::from_reader(contents.as_bytes());
    let mut rsi_trader = RSITrader::new(14).unwrap();
    
    // Total amount of stock currently owned.
    let mut stock_qty = 0.0;
    // Amount to stock to trade during each exchange.
    let trade_qty = 2.0;
    // Total amount of fiat money that can be spent during each exchange.
    let mut fiat_total = 10000.0;
    println!("Starting stock qty: {:?}", stock_qty);
    println!("Starting fiat_total: {:?}", fiat_total);

    for record in rdr.deserialize() {
        let record: Record = record?;

        match rsi_trader.next(record.close) {
            Some(action) => {
                println!("Time to {:?}", action);
                match action {
                    MarketActionType::Buy => {
                        let cost = trade_qty * record.close;
                        fiat_total = fiat_total - cost;
                        stock_qty = stock_qty + trade_qty;
                    },
                    MarketActionType::Sell => {
                        let cost = trade_qty * record.close;
                        fiat_total = fiat_total + cost;
                        stock_qty = stock_qty - trade_qty;
                    },
                }
            println!("New stock qty: {:?}", stock_qty);
            println!("New fiat_total: {:?}", fiat_total);
            },
            None => (),
        }
    };

    println!("Ending stock qty: {:?}", stock_qty);
    println!("Ending fiat_total: {:?}", fiat_total);

	Ok(())
}

struct RSITrader {
    rsi: RelativeStrengthIndex,
    count: usize,
    period: usize,
    overbought: usize,
    oversold: usize,
    in_position: bool,
}

impl RSITrader {
    pub fn new(period: usize) -> Result<Self, &'static str> {
        match period {
            0 => Err("Invalid parameter: period for RSITrader must be greater than 0."),
            _ => Ok(Self {
                rsi: RelativeStrengthIndex::new(period).unwrap(),
                period,
                count: 0,
                overbought: 70,
                oversold: 30,
                in_position: false,
            })
        }
    }
}

impl Next<f64> for RSITrader {
    type Output = Option<MarketActionType>;

    fn next(&mut self, close: f64) -> Self::Output {
        let rsi = self.rsi.next(close);
        self.count = self.count + 1;

        if self.count > self.period {
            if (rsi < self.oversold as f64) && !self.in_position {
                self.in_position = true;
                return Some(MarketActionType::Buy)
            } else if (rsi > self.overbought as f64) && self.in_position {
                self.in_position = false;
                return Some(MarketActionType::Sell)
            }
        }

        None
    }
}
