use std::num::{ParseFloatError, ParseIntError};

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, Default, PartialEq, Clone)]
#[error("evaluation error")]
pub enum LexingError {
    InvalidFloat {
        #[source_code]
        source: String,
        #[label("Error occurred here")]
        span: SourceSpan,
        #[source]
        e: ParseFloatError,
    },
    InvalidInteger {
        #[source_code]
        source: String,
        #[label("Error occurred here")]
        span: SourceSpan,
        #[source]
        e: ParseIntError,
    },
    #[default]
    UndefinedToken,
}
