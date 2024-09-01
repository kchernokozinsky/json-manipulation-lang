use std::{fmt, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct JmlBool(bool);

impl JmlBool {
    pub fn is_truthy(&self) -> bool {
        self.0
    }
}

impl FromStr for JmlBool {
    type Err = <bool as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FromStr::from_str(s).map(JmlBool)
    }
}

impl From<bool> for JmlBool {
    fn from(v: bool) -> Self {
        JmlBool(v)
    }
}

impl fmt::Display for JmlBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
