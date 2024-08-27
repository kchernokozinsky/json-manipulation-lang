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
    #[diagnostic(code(eval::division_by_zero), help("Ensure the divisor is not zero before performing division."))]
    DivisionByZero,

    #[error("Undefined variable during evaluation: {name}")]
    #[diagnostic(code(eval::undefined_variable), help("Check if the variable '{name}' is defined before using it."))]
    UndefinedVariable { name: String },

    #[error("Runtime error: {0}")]
    #[diagnostic(code(eval::runtime_error), help("Review the runtime conditions that could have led to this error."))]
    RuntimeError(String),
}

#[derive(Error, Diagnostic, Debug)]
#[error("type error")]
pub struct TypeError {
    #[label("Error occurred here")]
    pub span: SourceSpan,


    #[source]
    #[diagnostic_source]
    pub err: TypeErrorKind,
}

#[derive(Debug, Error, Diagnostic)]
#[diagnostic()]
pub enum TypeErrorKind {
    #[error("Mismatched types: expected {expected:?}, found {found:?}")]
    #[diagnostic(code(type_error::mismatched_types), help("Ensure that the types being compared or assigned are compatible."))]
    MismatchedTypes {
        expected: Vec<JmlType>,
        found: JmlType,
    },

    #[error("Expected {expected_count} arguments, but got {actual_count}")]
    #[diagnostic(code(type_error::argument_count_mismatch), help("Check the function call to ensure the correct number of arguments are provided."))]
    ArgumentCountMismatch {
        expected_count: usize,
        actual_count: usize,
    },

    #[error("Cannot compare types: {found}")]
    #[diagnostic(code(type_error::not_comparable), help("Consider implementing comparison logic for the type '{found}' or avoid comparing incompatible types."))]
    NotComparableType { found: JmlType },
}
