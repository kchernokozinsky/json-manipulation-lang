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
        BinaryOp::GT => todo!(),
        BinaryOp::LT => todo!(),
        BinaryOp::GE => todo!(),
        BinaryOp::LE => todo!(),
        BinaryOp::Sum => todo!(),
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
