use std::{fmt, ops, str::FromStr};

use super::float::JmlFloat;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]

pub struct JmlInt(pub(crate) i64);

impl FromStr for JmlInt {
    type Err = <i64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FromStr::from_str(s).map(JmlInt)
    }
}

impl From<i64> for JmlInt {
    fn from(v: i64) -> Self {
        JmlInt(v)
    }
}

impl From<i32> for JmlInt {
    fn from(v: i32) -> Self {
        JmlInt(v as i64)
    }
}

impl From<i16> for JmlInt {
    fn from(v: i16) -> Self {
        JmlInt(v as i64)
    }
}

impl From<i8> for JmlInt {
    fn from(v: i8) -> Self {
        JmlInt(v as i64)
    }
}

impl From<u32> for JmlInt {
    fn from(v: u32) -> Self {
        JmlInt(v as i64)
    }
}

impl From<u16> for JmlInt {
    fn from(v: u16) -> Self {
        JmlInt(v as i64)
    }
}

impl From<u8> for JmlInt {
    fn from(v: u8) -> Self {
        JmlInt(v as i64)
    }
}

impl From<usize> for JmlInt {
    fn from(v: usize) -> Self {
        JmlInt(v as i64)
    }
}

impl From<isize> for JmlInt {
    fn from(v: isize) -> Self {
        JmlInt(v as i64)
    }
}

impl fmt::Display for JmlInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Add<JmlInt> for JmlInt {
    type Output = JmlInt;

    fn add(self, rhs: JmlInt) -> JmlInt {
        JmlInt(self.0 + rhs.0)
    }
}

impl ops::Add<JmlFloat> for JmlInt {
    type Output = JmlFloat;

    fn add(self, rhs: JmlFloat) -> JmlFloat {
        JmlFloat(self.0 as f64 + rhs.0)
    }
}
