use std::fmt;

use crate::typings::Type;

use super::Value;

pub struct List {
    value: Vec<Box<dyn Value>>
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let values: Vec<String> = self.value
            .iter()
            .map(|v| format!("{}", v))
            .collect();
        write!(f, "[{}]", values.join(", "))
    }
}

impl Value for List {
    fn get_type(&self) -> Type {
         Type::List
    }
}