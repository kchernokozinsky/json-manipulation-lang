use std::fmt;

use crate::typings::Type;

use super::Value;

pub struct Bool {
    value: bool
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Value for Bool {
    fn get_type(&self) -> Type {
        Type::Bool
    }
}