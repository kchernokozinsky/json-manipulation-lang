use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]

pub struct JmlString(String);

impl FromStr for JmlString {
    type Err = <String as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FromStr::from_str(s).map(JmlString)
    }
}

impl From<String> for JmlString {
    fn from(v: String) -> Self {
        JmlString(v)
    }
}

impl From<&str> for JmlString {
    fn from(v: &str) -> Self {
        JmlString(v.to_owned())
    }
}

impl From<&String> for JmlString {
    fn from(v: &String) -> Self {
        v.as_str().into()
    }
}

impl fmt::Display for JmlString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}
