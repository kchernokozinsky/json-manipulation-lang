use crate::jml_type::JmlType;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("evaluation error")]
pub enum EvalError {
    #[diagnostic(transparent)]
    TypeError(#[from] TypeError),

    #[diagnostic(transparent)]
    RuntimeError(#[from] RuntimeError),
}

#[derive(Error, Diagnostic, Debug)]
#[error("type error")]

pub struct TypeError {
    #[label("Error occurred here")]
    pub span: SourceSpan,

    #[source]
    #[diagnostic_source]
    pub kind: TypeErrorKind,
}

#[derive(Debug, Error, Diagnostic)]
pub enum TypeErrorKind {
    #[error("Mismatched types: expected {expected:?}, found {found:?}")]
    #[diagnostic(
        code(type_error::mismatched_types),
        help("Ensure that the types being compared or assigned are compatible.")
    )]
    MismatchedTypes {
        expected: Vec<JmlType>,
        found: JmlType,
    },

    #[error("Expected {expected_count} arguments, but got {actual_count}")]
    #[diagnostic(
        code(type_error::argument_count_mismatch),
        help("Check the function call to ensure the correct number of arguments are provided.")
    )]
    ArgumentCountMismatch {
        expected_count: usize,
        actual_count: usize,
    },

    #[error("Cannot compare types: {found}")]
    #[diagnostic(code(type_error::not_comparable), help("Consider implementing comparison logic for the type '{found}' or avoid comparing incompatible types."))]
    NotComparableType { found: JmlType },
}

#[derive(Error, Diagnostic, Debug)]
#[error("runtime error")]
pub struct RuntimeError {
    #[label("Runtime error occurred here")]
    pub span: SourceSpan,

    #[source]
    #[diagnostic_source]
    pub kind: RuntimeErrorKind,
}

#[derive(Debug, Error, Diagnostic)]
#[diagnostic()]
pub enum RuntimeErrorKind {
    #[error("Division by zero")]
    #[diagnostic(
        code(eval::division_by_zero),
        help("Ensure the divisor is not zero before performing division.")
    )]
    DivisionByZero,

    #[error("Undefined variable during evaluation: {name}")]
    #[diagnostic(
        code(eval::undefined_variable),
        help("Check if the variable '{name}' is defined before using it.")
    )]
    UndefinedVariable { name: String },

    #[error("{message}")]
    #[diagnostic(code(runtime_error::generic_runtime_error))]
    GenericError { message: String },
}
