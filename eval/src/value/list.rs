use std::fmt;

use super::JmlValue;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]

pub struct JmlList(Vec<JmlValue>);

impl fmt::Display for JmlList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let values: Vec<String> = self.0.iter().map(|v| format!("{}", v)).collect();
        write!(f, "[{}]", values.join(", "))
    }
}
