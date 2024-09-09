use anyhow::Error;
use miette::Result;
use parser::ast::{BinaryOp, Expression};

use crate::{
    context::Context,
    error::{EvalError, RuntimeError, RuntimeErrorKind, TypeError, TypeErrorKind},
    value::JmlValue,
};

use super::eval_expr;

pub fn eval_binary_op<'source>(
    span: impl Into<miette::SourceSpan>,
    op: BinaryOp,
    lhs: Expression<'source>,
    rhs: Expression<'source>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
    let lhs = eval_expr(lhs, ctx)?;
    let rhs = eval_expr(rhs, ctx)?;

    match op {
        BinaryOp::EQ => Ok(JmlValue::bool(lhs == rhs)),
        BinaryOp::NE => Ok(JmlValue::bool(lhs != rhs)),
        BinaryOp::GT => greater(lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::LT => less(lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::GE => greater_equal(lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::LE => less_equal(lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::Sum => add(lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::Sub => subtract(lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::Mul => multiply(lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::Div => divide(lhs, rhs).map_err(|e| map_anyhow(e, span)),
        BinaryOp::Pow => pow(lhs, rhs).map_err(|e| map_anyhow(e, span)),
        BinaryOp::Mod => mod_op(lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::And => and(lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::Or => or(lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::Concat => todo!(),
    }
}

fn map_anyhow(e: Error, span: impl Into<miette::SourceSpan>) -> EvalError {
    {
        if let Some(runtime_error) = e.downcast_ref::<RuntimeErrorKind>() {
            RuntimeError {
                span: span.into(),
                kind: runtime_error.clone(),
            }
            .into()
        } else if let Some(type_error) = e.downcast_ref::<TypeErrorKind>() {
            TypeError {
                span: span.into(),
                kind: type_error.clone(),
            }
            .into()
        } else {
            unreachable!()
        }
    }
}

macro_rules! ord_op {
    ($func_name:ident, $operator:tt, $op_str:expr) => {
        fn $func_name<'a>(lhs: JmlValue, rhs: JmlValue) -> Result<JmlValue<'a>, TypeErrorKind> {

            if !&lhs.is_ord() {
                Err(TypeErrorKind::NotOrderedType {found: lhs.type_of()})?
            }

            if !&rhs.is_ord() {
                Err(TypeErrorKind::NotOrderedType {found: rhs.type_of()})?
            }

            match (&lhs, &rhs) {
                (JmlValue::Float(lhs), JmlValue::Float(rhs)) => Ok(JmlValue::bool(lhs.0 $operator rhs.0)),
                (JmlValue::Float(lhs), JmlValue::Int(rhs)) => Ok(JmlValue::bool(lhs.0 $operator rhs.0 as f64)),
                (JmlValue::Int(lhs), JmlValue::Float(rhs)) => Ok(JmlValue::bool((lhs.0 as f64) $operator rhs.0 )),
                (JmlValue::Int(lhs), JmlValue::Int(rhs)) =>Ok(JmlValue::bool(lhs.0 $operator rhs.0)),
                (JmlValue::Bool(lhs), JmlValue::Bool(rhs)) => Ok(JmlValue::bool(lhs.0 $operator rhs.0)),
                (JmlValue::String(lhs), JmlValue::String(rhs)) => Ok(JmlValue::bool(lhs.0 $operator rhs.0)),
                _ => Err(TypeErrorKind::InvalidBinaryOperator {
                    operator: $op_str.to_string(),
                    left: lhs.type_of(),
                    right: rhs.type_of(),
                }),
            }
        }
    };
}

ord_op!(greater, >, ">");
ord_op!(greater_equal, >=, ">=");
ord_op!(less, <, "<");
ord_op!(less_equal, <=, "<=");

macro_rules! logical_op {
    ($func_name:ident, $operator:tt, $op_str:expr) => {
        fn $func_name<'a>(lhs: JmlValue, rhs: JmlValue) -> Result<JmlValue<'a>, TypeErrorKind> {
            match (&lhs, &rhs) {
                (JmlValue::Bool(lhs), JmlValue::Bool(rhs)) => Ok(JmlValue::bool(lhs.0 $operator rhs.0)),
                _ => Err(TypeErrorKind::InvalidBinaryOperator {
                    operator: $op_str.to_string(),
                    left: lhs.type_of(),
                    right: rhs.type_of(),
                }.into()),
            }
        }
    };
}

logical_op!(and, &&, "&&");
logical_op!(or, ||, "||");

macro_rules! arithmetic_op {
    ($func_name:ident, /) => {
        fn $func_name<'a>(lhs: JmlValue, rhs: JmlValue) -> anyhow::Result<JmlValue<'a>> {
            if rhs.is_zero() {
                Err(RuntimeErrorKind::DivisionByZero)?
            }
            match (&lhs, &rhs) {
                (JmlValue::Float(lhs), JmlValue::Float(rhs)) => Ok(JmlValue::float(lhs.0 / rhs.0)),
                (JmlValue::Float(lhs), JmlValue::Int(rhs)) => Ok(JmlValue::float(lhs.0 / rhs.0 as f64)),
                (JmlValue::Int(lhs), JmlValue::Float(rhs)) => Ok(JmlValue::float(lhs.0 as f64 / rhs.0 )),
                (JmlValue::Int(lhs), JmlValue::Int(rhs)) =>Ok(JmlValue::int(lhs.0 / rhs.0)),
                _ => Err(TypeErrorKind::InvalidBinaryOperator {
                    operator: "/".to_owned(),
                    left: lhs.type_of(),
                    right: rhs.type_of(),
                })?,
            }

        }
    };
    ($func_name:ident, ^) => {
        fn $func_name<'a>(lhs: JmlValue, rhs: JmlValue) -> anyhow::Result<JmlValue<'a>> {
            match (&lhs, &rhs) {
                (JmlValue::Float(lhs), JmlValue::Float(rhs)) => Ok(JmlValue::float(lhs.0.powf(rhs.0))),
                (JmlValue::Float(lhs), JmlValue::Int(rhs)) => Ok(JmlValue::float(lhs.0.powf(rhs.0 as f64))),
                (JmlValue::Int(lhs), JmlValue::Float(rhs)) => Ok(JmlValue::float((lhs.0 as f64).powf(rhs.0))),
                (JmlValue::Int(lhs), JmlValue::Int(rhs)) => {
                    match lhs.0.checked_pow(rhs.0 as u32) {
                        Some(result) => Ok(JmlValue::int(result)),
                        None => Err(RuntimeErrorKind::Overflow)?,
                    }
                }
                _ => Err(TypeErrorKind::InvalidBinaryOperator {
                    operator: "^".to_string(),
                    left: lhs.type_of(),
                    right: rhs.type_of(),
                }.into()),
            }
        }
    };
    ($func_name:ident, $operator:tt, $op_str:expr) => {
        fn $func_name<'a>(lhs: JmlValue, rhs: JmlValue) -> Result<JmlValue<'a>, TypeErrorKind> {
            match (&lhs, &rhs) {
                (JmlValue::Float(lhs), JmlValue::Float(rhs)) => Ok(JmlValue::float(lhs.0 $operator rhs.0)),
                (JmlValue::Float(lhs), JmlValue::Int(rhs)) => Ok(JmlValue::float(lhs.0 $operator rhs.0 as f64)),
                (JmlValue::Int(lhs), JmlValue::Float(rhs)) => Ok(JmlValue::float(lhs.0 as f64 $operator rhs.0 )),
                (JmlValue::Int(lhs), JmlValue::Int(rhs)) =>Ok(JmlValue::int(lhs.0 $operator rhs.0)),
                _ => Err(TypeErrorKind::InvalidBinaryOperator {
                    operator: $op_str.to_string(),
                    left: lhs.type_of(),
                    right: rhs.type_of(),
                }),
            }
        }
    };
}

arithmetic_op!(add, +, "+");
arithmetic_op!(subtract, -, "-");
arithmetic_op!(multiply, *, "*");
arithmetic_op!(mod_op, %, "%");
arithmetic_op!(divide, /);
arithmetic_op!(pow, ^);
