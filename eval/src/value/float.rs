use std::{fmt, str::FromStr};

#[derive(Debug, Copy, Clone)]

pub struct JmlFloat(pub(crate) f64);

impl PartialEq for JmlFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for JmlFloat {}

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
