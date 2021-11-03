use ta::indicators::ExponentialMovingAverage;
use ta::Next;

fn main() {
    let mut ema = ExponentialMovingAverage::new(3).unwrap();
    ema.next(2.0);
    ema.next(5.0);
    ema.next(1.0);
    let avg = ema.next(6.25);
    println!("Avg: {:?}", avg);
}
