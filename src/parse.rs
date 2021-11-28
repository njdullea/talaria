use serde::{de::Error, Deserialize, Deserializer};
use std::{fmt::Display, str::FromStr};

pub fn num_from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<T>().map_err(D::Error::custom)
}

pub fn u64_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(D::Error::custom)
}

fn _f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(D::Error::custom)
}
