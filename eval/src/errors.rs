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

#[derive(Debug, Error, Diagnostic, Clone)]
pub enum TypeErrorKind {
    #[error("Mismatched types: expected {expected:?}, found {found:?}")]
    #[diagnostic(code(type_error::mismatched_types))]
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

    #[error("Type {found} is not ordered")]
    #[diagnostic(code(type_error::not_ordered))]
    NotOrderedType { found: JmlType },

    #[error("Binary operator '{operator}' cannot be applied to types {left:?} and {right:?}")]
    #[diagnostic(
        code(type_error::invalid_binary_operator),
        help("Ensure the operator '{operator}' is used with compatible types.")
    )]
    InvalidBinaryOperator {
        operator: String,
        left: JmlType,
        right: JmlType,
    },

    #[error("Uanry operator '{operator}' cannot be applied to type {right:?}")]
    #[diagnostic(
        code(type_error::invalid_binary_operator),
        help("Ensure the operator '{operator}' is used with compatible type.")
    )]
    InvalidUnaryOperator { operator: String, right: JmlType },
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

#[derive(Debug, Error, Diagnostic, Clone)]
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

    #[error("Overflow occurred during evaluation.")]
    #[diagnostic(
        code(eval::overflow),
        help("Consider using a larger data type or rethinking the operation to avoid overflow.")
    )]
    Overflow,

    #[error("{message}")]
    #[diagnostic(code(runtime_error::generic_runtime_error))]
    GenericError { message: String },
}
