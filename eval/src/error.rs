use crate::jml_type::JmlType;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("evaluation error")]
pub enum EvalError {
    #[diagnostic(transparent)]
    TypeError(
        #[from]
        TypeError,
    ),
    
    #[error("Division by zero")]
    #[diagnostic(code(eval::division_by_zero))]
    DivisionByZero,

    #[error("Undefined variable during evaluation: {name}")]
    #[diagnostic(code(eval::undefined_variable))]
    UndefinedVariable { name: String },

    #[error("Runtime error: {0}")]
    #[diagnostic(code(eval::runtime_error))]
    RuntimeError(String),
}

#[derive(Error, Diagnostic, Debug)]
#[diagnostic()]
#[error("type error")]
pub struct TypeError {
    #[label("Error occurred here")]
    pub span: SourceSpan,

    #[source]
    pub err: TypeErrorKind,
}

#[derive(Debug, Error, Diagnostic)]
#[diagnostic()]
pub enum TypeErrorKind {
    #[error("Mismatched types: expected {expected:?}, found {found:?}")]
    #[diagnostic(code(type_error::mismatched_types))]
    MismatchedTypes {
        expected: Vec<JmlType>,
        found: JmlType,
    },

    #[error("Expected {expected_count} arguments, but got {actual_count}")]
    #[diagnostic(code(type_error::argument_count_mismatch))]
    ArgumentCountMismatch {
        expected_count: usize,
        actual_count: usize,
    },

    #[error("Cannot compare types: {found}")]
    #[diagnostic(code(type_error::not_comparable))]
    NotComparableType { found: JmlType },
}
