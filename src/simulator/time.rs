use std::num::ParseIntError;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};
use std::ops::Add;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Time(pub u64);

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

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}
