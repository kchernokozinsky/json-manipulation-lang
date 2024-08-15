use std::num::{ParseFloatError, ParseIntError};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidFloat(usize, String, usize),
    InvalidInteger(usize, String, usize),
    #[default]
    UndefinedToken,
}

impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        todo!()
    }
}

impl From<ParseFloatError> for LexingError {
    fn from(err: ParseFloatError) -> Self {
        todo!()
    }
}
