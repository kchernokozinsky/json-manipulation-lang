use error::EvalError;
use expr::eval_expr;
use parser::ast::{self, Expression, Jml};
use std::{error::Error};
use value::JmlValue;

pub mod context;
pub mod error;
pub mod expr;
pub mod jml_type;
pub mod value;

pub fn eval(jml: &Jml) ->Result<JmlValue, EvalError> {
    eval_expr(&jml.body)
}
