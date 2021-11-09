use crate::market::{MarketAction, Trade};
use crate::traders::rsi::rsi_indicator::RelativeStrengthIndex;
use crate::traits::Description;
use ta::DataItem;
use ta::Next;

pub struct RSITrader {
    rsi: RelativeStrengthIndex,
    count: usize,
    period: usize,
    overbought: usize,
    oversold: usize,
    in_position: bool,
    description: &'static str,
}

impl RSITrader {
    pub fn new(period: usize) -> Result<Self, &'static str> {
        match period {
            0 => Err("Invalid parameter: period for RSITrader must be greater than 0."),
            _ => Ok(Self {
                rsi: RelativeStrengthIndex::new(period),
                period,
                count: 0,
                overbought: 70,
                oversold: 30,
                in_position: false,
                description: "RSI Trader",
            }),
        }
    }

    pub fn reset(&mut self) {
        self.rsi = RelativeStrengthIndex::new(self.period);
        self.count = 0;
        self.in_position = false;
    }
}

impl Trade for &mut RSITrader {
    fn trade(&mut self, data_item: DataItem) -> Option<MarketAction> {
        let rsi = self.rsi.next(data_item);
        self.count = self.count + 1;

        if self.count > self.period && rsi.is_some() {
            if (rsi.unwrap() < self.oversold as f64) && !self.in_position {
                self.in_position = true;
                return Some(MarketAction::Buy);
            } else if (rsi.unwrap() > self.overbought as f64) && self.in_position {
                self.in_position = false;
                return Some(MarketAction::Sell);
            }
        }

        None
    }
}

impl Description for &mut RSITrader {
    fn description(&self) -> &str {
        self.description
    }
}

impl Default for RSITrader {
    fn default() -> Self {
        RSITrader::new(14).unwrap()
    }
}
