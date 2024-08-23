use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JmlType {
    Null,
    String,
    Bool,
    Int,
    Float,
    List,
    Object,
    Lambda,
}

impl JmlType {
    pub fn is_comparable(self) -> bool {
        !matches!(self, JmlType::Lambda)
    }

    pub fn is_number(self) -> bool {
        matches!(self, JmlType::Float | JmlType::Int)
    }
}

impl fmt::Display for JmlType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_name = match self {
            JmlType::Null => "Null",
            JmlType::String => "String",
            JmlType::Bool => "Bool",
            JmlType::Int => "Int",
            JmlType::Float => "Float",
            JmlType::List => "List",
            JmlType::Object => "Object",
            JmlType::Lambda => "Lambda",
        };
        write!(f, "{}", type_name)
    }
}
