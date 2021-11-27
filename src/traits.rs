use std::sync::mpsc;
use crate::record;

// pub trait Description {
//     fn description(&self) -> &str;
// }

// pub trait Reset {
//     fn reset(&self);
// }

// pub trait Default {
//     fn default() -> Self;
// }

pub trait Exchange {
    fn save_testing_data(days: i64) -> Result<(), Box<dyn std::error::Error>>;

    fn subscribe_to_data(tx: mpsc::Sender<record::Record>);
}
