use crate::market::{MarketAction, Trade};
use crate::traits::Description;
use ta::indicators::SlowStochastic;
use ta::DataItem;
use ta::{Close, Next};

pub struct SSOTrader {
    sso: SlowStochastic,
    count: usize,
    period: usize,
    overbought: usize,
    oversold: usize,
    in_position: bool,
    description: &'static str,
}

impl SSOTrader {
    pub fn new(period: usize) -> Result<Self, &'static str> {
        match period {
            0 => Err("Invalid parameter: period for RSITrader must be greater than 0."),
            _ => Ok(Self {
                sso: SlowStochastic::new(period, 3).unwrap(),
                period,
                count: 0,
                overbought: 80,
                oversold: 20,
                in_position: false,
                description: "SSO Trader",
            }),
        }
    }

    pub fn reset(&mut self) {
        self.sso = SlowStochastic::new(self.period, 3).unwrap();
        self.count = 0;
        self.in_position = false;
    }
}

impl Trade for &mut SSOTrader {
    fn trade(&mut self, data_item: DataItem) -> Option<MarketAction> {
        let sso = self.sso.next(data_item.close());
        self.count = self.count + 1;

        if self.count > self.period {
            if (sso < self.oversold as f64) && !self.in_position {
                self.in_position = true;
                return Some(MarketAction::Buy);
            } else if (sso > self.overbought as f64) && self.in_position {
                self.in_position = false;
                return Some(MarketAction::Sell);
            }
        }

        None
    }
}

impl Description for &mut SSOTrader {
    fn description(&self) -> &str {
        self.description
    }
}

impl Default for SSOTrader {
    fn default() -> Self {
        SSOTrader::new(14).unwrap()
    }
}
