use derive_more::{derive::Display, From, FromStr};

#[derive(Debug, Clone, PartialEq, Eq, From, FromStr, Display)]
#[from(String, &str)]
pub struct JmlString(#[display("\"{}\"")] pub(crate) String);
