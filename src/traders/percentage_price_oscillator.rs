use crate::market::{MarketAction, Trade};
use crate::traits::Description;
use ta::indicators::PercentagePriceOscillator;
use ta::DataItem;
use ta::{Close, Next};

pub struct PPOTrader {
    ppo: PercentagePriceOscillator,
    count: usize,
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    in_position: bool,
    description: &'static str,
}

impl PPOTrader {
    pub fn new(
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> Result<Self, &'static str> {
        if fast_period < 1 {
            return Err("Fast period must be greater than 0");
        } else if slow_period < 1 {
            return Err("Slow period must be greater than 0");
        } else if signal_period < 1 {
            return Err("Signal period must be greater than 0");
        }

        Ok(Self {
            ppo: PercentagePriceOscillator::new(fast_period, slow_period, signal_period).unwrap(),
            fast_period,
            slow_period,
            signal_period,
            count: 0,
            in_position: false,
            description: "PPO Trader",
        })
    }

    pub fn reset(&mut self) {
        self.ppo =
            PercentagePriceOscillator::new(self.fast_period, self.slow_period, self.signal_period)
                .unwrap();
        self.count = 0;
        self.in_position = false;
    }
}

impl Trade for &mut PPOTrader {
    fn trade(&mut self, data_item: DataItem) -> Option<MarketAction> {
        let ppo_output = self.ppo.next(data_item.close());
        self.count = self.count + 1;

        if self.count > self.slow_period {
            if ppo_output.ppo > ppo_output.signal && !self.in_position {
                self.in_position = true;
                return Some(MarketAction::Buy);
            } else if ppo_output.ppo < ppo_output.signal && self.in_position {
                self.in_position = false;
                return Some(MarketAction::Sell);
            }
        }

        None
    }
}

impl Description for &mut PPOTrader {
    fn description(&self) -> &str {
        self.description
    }
}
