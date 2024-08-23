use crate::jml_type::JmlType;

#[derive(Debug, thiserror::Error)]

pub enum EvalError {
    #[error("Type error: {0}")]
    TypeError(#[from] TypeError),

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Undefined variable during evaluation: {name}")]
    UndefinedVariable { name: String },

    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum TypeError {
    #[error("Mismatched types: expected {expected:?}, found {found:?}")]
    MismatchedTypes {
        expected: Vec<JmlType>,
        found: JmlType,
    },

    #[error("Expected {expected_count} arguments, but got {actual_count}")]
    ArgumentCountMismatch {
        expected_count: usize,
        actual_count: usize,
    },

    #[error("Cannot compare types: {found}")]
    NotComparableType { found: JmlType },
}
