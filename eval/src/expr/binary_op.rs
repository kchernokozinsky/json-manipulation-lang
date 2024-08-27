use parser::ast::{BinaryOp, Expression};

use crate::{
    context::Context,
    error::{EvalError, TypeError, TypeErrorKind},
    value::JmlValue,
};

use super::eval_expr;

pub fn eval_binary_op(
    op: BinaryOp,
    lhs: Expression,
    rhs: Expression,
    ctx: &Context<'_>,
) -> Result<JmlValue, EvalError> {
    let span = (lhs.l, rhs.r - lhs.l);
    let lhs = eval_expr(lhs, ctx)?;
    let rhs = eval_expr(rhs, ctx)?;

    match op {
        BinaryOp::EQ => Ok(JmlValue::bool(lhs == rhs)),
        BinaryOp::NE => Ok(JmlValue::bool(lhs != rhs)),
        BinaryOp::GT => eval_ord_op(op, lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::LT => eval_ord_op(op, lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::GE => eval_ord_op(op, lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::LE => eval_ord_op(op, lhs, rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::Sum => (lhs + rhs).map_err(|e| {
            TypeError {
                span: span.into(),
                kind: e,
            }
            .into()
        }),
        BinaryOp::Sub => todo!(),
        BinaryOp::Mul => todo!(),
        BinaryOp::Div => todo!(),
        BinaryOp::Pow => todo!(),
        BinaryOp::Mod => todo!(),
        BinaryOp::And => todo!(),
        BinaryOp::Or => todo!(),
        BinaryOp::Concat => todo!(),
    }
}

fn eval_ord_op(op: BinaryOp, lhs: JmlValue, rhs: JmlValue) -> Result<JmlValue, TypeErrorKind> {
    let lhs_type = lhs.type_of();
    let rhs_type = rhs.type_of();
    if lhs_type == rhs_type {
        match op {
            BinaryOp::GT => Ok(JmlValue::bool(lhs > rhs)),
            BinaryOp::LT => Ok(JmlValue::bool(lhs < rhs)),
            BinaryOp::GE => Ok(JmlValue::bool(lhs >= rhs)),
            BinaryOp::LE => Ok(JmlValue::bool(lhs <= rhs)),
            _ => unreachable!(),
        }
    } else {
        Err(TypeErrorKind::MismatchedTypes {
            expected: vec![lhs_type],
            found: rhs_type,
        })
    }
}
