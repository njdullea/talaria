use std::error::Error;
use std::fs;
use ta::indicators::RelativeStrengthIndex;
use ta::DataItem;
use ta::{Close, Next};

mod record;
use crate::record::Record;

trait Trade {
    fn trade(&mut self, data_item: DataItem) -> Option<MarketAction>;
}

#[derive(Debug)]
enum MarketAction {
    Buy,
    Sell,
}

fn main() {
    // match backtest_rsi() {
    //     Ok(_) => println!("Ok"),
    //     Err(_) => println!("Err"),
    // }

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

// fn backtest_rsi() -> Result<(), Box<dyn Error>> {
// 	let contents = fs::read_to_string("data/AMZN.csv").expect("Something went wrong reading the file.");
//     let mut rdr = csv::Reader::from_reader(contents.as_bytes());
//     let mut rsi_trader = RSITrader::new(14).unwrap();

//     // Total amount of stock currently owned.
//     let mut stock_qty = 0.0;
//     // Amount to stock to trade during each exchange.
//     let trade_qty = 2.0;
//     // Total amount of fiat money that can be spent during each exchange.
//     let mut fiat_total = 10000.0;
//     println!("Starting stock qty: {:?}", stock_qty);
//     println!("Starting fiat_total: {:?}", fiat_total);

//     for record in rdr.deserialize() {
//         let record: Record = record?;

//         match rsi_trader.next(record.close) {
//             Some(action) => {
//                 println!("Time to {:?}", action);
//                 match action {
//                     MarketAction::Buy => {
//                         let cost = trade_qty * record.close;
//                         fiat_total = fiat_total - cost;
//                         stock_qty = stock_qty + trade_qty;
//                     },
//                     MarketAction::Sell => {
//                         let cost = trade_qty * record.close;
//                         fiat_total = fiat_total + cost;
//                         stock_qty = stock_qty - trade_qty;
//                     },
//                 }
//             println!("New stock qty: {:?}", stock_qty);
//             println!("New fiat_total: {:?}", fiat_total);
//             },
//             None => (),
//         }
//     };

//     println!("Ending stock qty: {:?}", stock_qty);
//     println!("Ending fiat_total: {:?}", fiat_total);

// 	Ok(())
// }

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
            }),
        }
    }
}

impl Trade for &mut RSITrader {
    fn trade(&mut self, data_item: DataItem) -> Option<MarketAction> {
        let rsi = self.rsi.next(data_item.close());
        self.count = self.count + 1;

        if self.count > self.period {
            if (rsi < self.oversold as f64) && !self.in_position {
                self.in_position = true;
                return Some(MarketAction::Buy);
            } else if (rsi > self.overbought as f64) && self.in_position {
                self.in_position = false;
                return Some(MarketAction::Sell);
            }
        }

        None
    }
}

// impl Next<f64> for RSITrader {
//     type Output = Option<MarketAction>;

//     fn next(&mut self, close: f64) -> Self::Output {
//         let rsi = self.rsi.next(close);
//         self.count = self.count + 1;

//         if self.count > self.period {
//             if (rsi < self.oversold as f64) && !self.in_position {
//                 self.in_position = true;
//                 return Some(MarketAction::Buy)
//             } else if (rsi > self.overbought as f64) && self.in_position {
//                 self.in_position = false;
//                 return Some(MarketAction::Sell)
//             }
//         }

//         None
//     }
// }
