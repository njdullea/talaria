use std::usize;

use crate::market::{MarketAction, Trade};
use crate::traits::Description;
use ta::indicators::MoneyFlowIndex;
use ta::DataItem;
use ta::{Close, Next};

pub struct MFITrader {
    mfi: MoneyFlowIndex,
    count: usize,
    period: usize,
	overbought: usize,
	oversold: usize,
    in_position: bool,
    description: &'static str,
}

impl MFITrader {
    pub fn new(period: usize) -> Result<Self, &'static str> {
        match period {
            0 => Err("Invalid parameter: period for MFITrader must be greater than 0."),
            _ => Ok(Self {
                mfi: MoneyFlowIndex::new(period).unwrap(),
                period,
                count: 0,
				overbought: 80,
                oversold: 20,
                in_position: false,
                description: "MFI Trader",
            }),
        }
    }

    pub fn reset(&mut self) {
        self.mfi = MoneyFlowIndex::new(self.period).unwrap();
        self.count = 0;
        self.in_position = false;
    }
}

impl Trade for &mut MFITrader {
    fn trade(&mut self, data_item: DataItem) -> Option<MarketAction> {
        let mfi = self.mfi.next(data_item);
        self.count = self.count + 1;

        if self.count > self.period {
			if mfi > 0 {

			} else if mfi < 0 {}
            // if (mfi < self.oversold as f64) && !self.in_position {
            //     self.in_position = true;
            //     return Some(MarketAction::Buy);
            // } else if (mfi > self.overbought as f64) && self.in_position {
            //     self.in_position = false;
            //     return Some(MarketAction::Sell);
            // }
        }

        None
    }
}

impl Description for &mut MFITrader {
    fn description(&self) -> &str {
        self.description
    }
}