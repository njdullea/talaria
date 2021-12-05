use chrono::{DateTime, Duration, Utc};
use std::iter::Iterator;
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    interval_size: Duration,
}

impl TimeRange {
    pub fn new(
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        interval_size: i64,
    ) -> Result<Self, &'static str> {
        // Binance has max limit of 500 items, Coinbase 300, Kraken 1500, KuCoin 1500

        // Maybe I need to just implement a way to update step size?
        if interval_size > 500_i64 {
            return Err("Max interval size is 500.");
        }

        Ok(Self {
            start: start,
            end,
            interval_size: Duration::minutes(interval_size),
        })
    }

    pub fn default() -> Self {
        let system_time = SystemTime::now();
        let start = DateTime::from(system_time)
            .checked_sub_signed(Duration::days(7))
            .unwrap();

        let end = DateTime::from(system_time);
        TimeRange::new(start, end, 300_i64).unwrap()
    }
}

/*
Splits time range into subsections of step size.
*/
impl Iterator for TimeRange {
    type Item = TimeRange;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start.ge(&self.end) {
            return None;
        }

        let current_start = self.start.clone();
        let next_start = self
            .start
            .clone()
            .checked_add_signed(self.interval_size)
            .unwrap();

        if next_start.le(&self.end) {
            self.start = next_start;
        } else {
            self.start = self.end;
        }

        Some(TimeRange {
            start: current_start,
            end: self.start,
            interval_size: self.interval_size,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_iter() {
//         let tr = super::TimeRange::default();
//         // for sr in tr {
//         // 	println!("SR: {:?}", sr);
//         // }
//         tr.for_each(|f| println!("sr: {:?}", f));
//     }
// }
