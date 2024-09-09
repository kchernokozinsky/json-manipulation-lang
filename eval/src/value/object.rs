use derive_more::From;
use std::collections::HashMap;
use std::fmt;

use super::JmlValue;

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct JmlObject<'a>(pub(crate) HashMap<String, JmlValue<'a>>);

impl<'a> JmlObject<'a> {
    pub fn access_by_key(&self, key: &str) -> JmlValue<'a> {
        self.0.get(key).map_or(JmlValue::null(), |v| v.clone())
    }
}

impl<'a> fmt::Display for JmlObject<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entries: Vec<String> = self
            .0
            .iter()
            .map(|(k, v)| format!("\"{}\": {}", k, v))
            .collect();
        write!(f, "{{{}}}", entries.join(", "))
    }
}
