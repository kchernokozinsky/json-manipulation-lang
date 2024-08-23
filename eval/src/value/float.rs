use std::{cmp::Ordering, fmt, ops, str::FromStr};

use super::integer::JmlInt;

#[derive(Debug, Copy, Clone)]

pub struct JmlFloat(pub(crate) f64);

impl PartialEq for JmlFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for JmlFloat {}

impl PartialOrd for JmlFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for JmlFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        // This will panic if either value is NaN
        self.partial_cmp(other)
            .expect("Cannot compare JmlFloat values with NaN")
    }
}

impl FromStr for JmlFloat {
    type Err = <f64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FromStr::from_str(s).map(JmlFloat)
    }
}

impl From<f64> for JmlFloat {
    fn from(v: f64) -> Self {
        JmlFloat(v)
    }
}

impl From<f32> for JmlFloat {
    fn from(v: f32) -> Self {
        JmlFloat(v as f64)
    }
}

impl fmt::Display for JmlFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Add<JmlFloat> for JmlFloat {
    type Output = JmlFloat;

    fn add(self, rhs: JmlFloat) -> JmlFloat {
        JmlFloat(self.0 + rhs.0)
    }
}

impl ops::Add<JmlInt> for JmlFloat {
    type Output = JmlFloat;

    fn add(self, rhs: JmlInt) -> JmlFloat {
        JmlFloat(self.0 + rhs.0 as f64)
    }
}
