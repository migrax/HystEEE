use std::num::ParseIntError;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};
use std::ops::{Add, Sub, Div};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Time(pub u64);

impl Time {
    pub fn as_secs(&self) -> f64 {
        self.0 as f64 / 1e9
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

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}
