use crate::record;
use crate::time_range::TimeRange;
use std::sync::mpsc;

// pub trait Description {
//     fn description(&self) -> &str;
// }

// pub trait Reset {
//     fn reset(&self);
// }

// pub trait Default {
//     fn default() -> Self;
// }

pub trait Next {
    fn next();
}

pub trait Exchange {
    fn save_testing_data(time_range: TimeRange) -> Result<(), Box<dyn std::error::Error>>;

    fn subscribe_to_data(tx: mpsc::Sender<record::Record>);
}
