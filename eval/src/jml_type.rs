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
    Lambda { arity: usize },
}

impl JmlType {
    pub fn is_comparable(self) -> bool {
        !matches!(self, JmlType::Lambda { .. })
    }

    pub fn is_ord(self) -> bool {
        !matches!(
            self,
            JmlType::Lambda { .. } | JmlType::Object | JmlType::List
        )
    }

    pub fn is_number(self) -> bool {
        matches!(self, JmlType::Float | JmlType::Int)
    }

    pub fn is_bool(self) -> bool {
        matches!(self, JmlType::Bool)
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
            JmlType::Lambda { arity } => {
                let params: String = (0..*arity)
                    .map(|i| ((b'a' + i as u8) as char).to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                &format!("Fn ({}) -> output", params)
            }
        };
        write!(f, "{}", type_name)
    }
}
