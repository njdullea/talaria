use ta::indicators::SlowStochastic;
use ta::DataItem;
use ta::{Close, Next};
use crate::market::{Trade, MarketAction};
use crate::description::Description;

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