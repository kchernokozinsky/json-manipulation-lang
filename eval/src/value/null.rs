use std::fmt;

use crate::typings::Type;

use super::Value;


pub struct Null;

impl fmt::Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "null")
    }
}

impl Value for Null {
    fn get_type(&self) -> Type {
        Type::Null
    }
}