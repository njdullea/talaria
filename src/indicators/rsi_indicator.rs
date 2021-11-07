use ta::indicators::ExponentialMovingAverage as Ema;
use ta::Close;
use ta::DataItem;
use ta::Next;

pub struct RelativeStrengthIndex {
    prev_close: f64,
    period: usize,
    count: usize,
    average_gain: Ema,
    average_loss: Ema,
    pub rsi: Option<f64>,
}

impl RelativeStrengthIndex {
    pub fn new(period: usize) -> RelativeStrengthIndex {
        RelativeStrengthIndex {
            period,
            count: 0,
            average_gain: Ema::new(period).unwrap(),
            average_loss: Ema::new(period).unwrap(),
            rsi: None,
            prev_close: 0.0,
        }
    }
}

impl Next<DataItem> for RelativeStrengthIndex {
    type Output = Option<f64>;

    fn next(&mut self, input: DataItem) -> Self::Output {
        self.count = self.count + 1;

        if self.count == 0 {
            self.prev_close = input.close();
            return None;
        }

        let mut up = 0.0;
        let mut down = 0.0;

        if input.close() > self.prev_close {
            up = input.close() - self.prev_close;
        } else {
            down = self.prev_close - input.close();
        }

        let gain = self.average_gain.next(up);
        let loss = self.average_loss.next(down);
        self.prev_close = input.close();

        if self.count > self.period {
            if loss == 0_f64 {
                self.rsi = Some(100_f64);
                return Some(100_f64);
            }

            let rsi = 100_f64 - (100_f64 / (1_f64 + (gain / loss)));
            self.rsi = Some(rsi);
            return Some(rsi);
        }

        None
    }
}
