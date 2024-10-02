use derive_more::From;
use indexmap::IndexMap;
use std::fmt;

use super::JmlValue;

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct JmlObject<'source>(pub(crate) IndexMap<String, JmlValue<'source>>);

impl<'source> JmlObject<'source> {
    pub fn access_by_key(&self, key: impl AsRef<str>) -> JmlValue<'source> {
        self.0
            .get(key.as_ref())
            .map_or(JmlValue::null(), |v| v.clone())
    }
}

impl<'source> fmt::Display for JmlObject<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entries: Vec<String> = self
            .0
            .iter()
            .map(|(k, v)| format!("\"{}\": {}", k, v))
            .collect();
        write!(f, "{{{}}}", entries.join(", "))
    }
}
