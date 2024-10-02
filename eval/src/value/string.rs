use derive_more::{derive::Display, From, FromStr};

use super::JmlValue;

#[derive(Debug, Clone, PartialEq, Eq, From, FromStr, Display)]
#[from(String, &String, &str)]
pub struct JmlString(#[display("\"{}\"")] pub(crate) String);

impl JmlString {
    pub fn get_by_index<'source>(&self, index: usize) -> JmlValue<'source> {
        match self.0.chars().nth(index) {
            Some(c) => JmlValue::string(c.to_string()),
            None => JmlValue::null(),
        }
    }
}
