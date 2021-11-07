use crate::market::{MarketAction, Trade};
use crate::traits::Description;
use ta::indicators::FastStochastic;
use ta::DataItem;
use ta::{Close, Next};

pub struct FSOTrader {
    fso: FastStochastic,
    count: usize,
    period: usize,
    overbought: usize,
    oversold: usize,
    in_position: bool,
    description: &'static str,
}

impl FSOTrader {
    pub fn new(period: usize) -> Result<Self, &'static str> {
        match period {
            0 => Err("Invalid parameter: period for RSITrader must be greater than 0."),
            _ => Ok(Self {
                fso: FastStochastic::new(period).unwrap(),
                period,
                count: 0,
                overbought: 80,
                oversold: 20,
                in_position: false,
                description: "FSO Trader",
            }),
        }
    }

    pub fn reset(&mut self) {
        self.fso = FastStochastic::new(self.period).unwrap();
        self.count = 0;
        self.in_position = false;
    }
}

impl Trade for &mut FSOTrader {
    fn trade(&mut self, data_item: DataItem) -> Option<MarketAction> {
        let fso = self.fso.next(data_item.close());
        self.count = self.count + 1;

        if self.count > self.period {
            if (fso < self.oversold as f64) && !self.in_position {
                self.in_position = true;
                return Some(MarketAction::Buy);
            } else if (fso > self.overbought as f64) && self.in_position {
                self.in_position = false;
                return Some(MarketAction::Sell);
            }
        }

        None
    }
}

impl Description for &mut FSOTrader {
    fn description(&self) -> &str {
        self.description
    }
}

impl Default for FSOTrader {
    fn default() -> Self {
        FSOTrader::new(14).unwrap()
    }
}