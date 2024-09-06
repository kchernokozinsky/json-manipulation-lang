use derive_more::{derive::Display, From, FromStr};

#[derive(Debug, Copy, Clone, From, Display, FromStr)]
#[from(f64, f32, i32, i16, i8, u32, u16, u8)]
pub struct JmlFloat(#[display("{}")] pub(crate) f64);

impl PartialEq for JmlFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for JmlFloat {}
