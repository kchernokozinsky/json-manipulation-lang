use std::fmt;

use super::JmlValue;
use derive_more::From;

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct JmlList(pub(crate) Vec<JmlValue>);

impl JmlList {
    pub fn access_by_index(&self, index: usize) -> JmlValue {
        self.0.get(index).map_or(JmlValue::null(), |v| v.clone())
    }
}

impl fmt::Display for JmlList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let values: Vec<String> = self.0.iter().map(|v| format!("{}", v)).collect();
        write!(f, "[{}]", values.join(", "))
    }
}
