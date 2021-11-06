pub trait Description {
    fn description(&self) -> &str;
}

pub trait Reset {
    fn reset(&self);
}
