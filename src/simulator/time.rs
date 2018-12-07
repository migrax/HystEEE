use std::num::ParseIntError;
use std::ops::{Add, Div, Sub};
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Time(pub u64);

impl Time {
    pub fn as_secs(self) -> f64 {
        self.0 as f64 / 1e9
    }

    pub fn from_secs(t: f64) -> Time {
        Time((1e9 * t).round() as u64)
    }
}

impl FromStr for Time {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = u64::from_str(s)?;

        Ok(Time(val))
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        Time(self.0 + other.0)
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        Time(self.0 - other.0)
    }
}

impl Div for Time {
    type Output = f64;

    fn div(self, other: Time) -> f64 {
        self.0 as f64 / other.0 as f64
    }
}
