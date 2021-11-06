use ta::indicators::RelativeStrengthIndex;
use ta::DataItem;
use ta::{Close, Next};
use crate::market::{Trade, MarketAction};

pub struct RSITrader {
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