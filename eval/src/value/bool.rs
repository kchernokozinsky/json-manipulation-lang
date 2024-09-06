use derive_more::{derive::Display, From, FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, From, Display, FromStr)]
pub struct JmlBool(#[display("{}")] pub(crate) bool);

impl JmlBool {
    pub fn is_truthy(&self) -> bool {
        self.0
    }
}
