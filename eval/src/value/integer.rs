use std::fmt;

use crate::typings::Type;

use super::Value;

pub struct Int {
    value: i64
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Value for Int {
    fn get_type(&self) -> Type {
        Type::Int
    }
}