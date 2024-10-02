use std::ops::Neg;

use derive_more::{derive::Display, From, FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, From, Display, FromStr)]
#[from(i64, i32, i16, i8, u32, u16, u8)]
pub struct JmlInt(#[display("{}")] pub(crate) i64);

impl JmlInt {
    pub fn negative(self) -> Self {
        JmlInt(self.0.neg())
    }
}
