use crate::record;
use crate::time_range::TimeRange;

pub trait Next {
    fn next();
}

pub trait Exchange {
    fn save_testing_data(time_range: TimeRange) -> Result<(), Box<dyn std::error::Error>>;

    fn subscribe_to_data(
        tx: flume::Sender<Result<record::Record, &'static str>>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
