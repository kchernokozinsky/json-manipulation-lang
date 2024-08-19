use std::fmt;

use crate::typings::Type;

use super::Value;

pub struct Float {
    value: f64
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Value for Float {
    fn get_type(&self) -> Type {
        Type::Float
    }
}