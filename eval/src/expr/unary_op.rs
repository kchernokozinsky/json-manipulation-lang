use miette::Result;
use parser::ast::{Expression, UnaryOp};

use crate::{
    context::Context,
    error::{EvalError, TypeError, TypeErrorKind},
    value::JmlValue,
};

use super::eval_expr;

pub fn eval_unary_op(
    span: impl Into<miette::SourceSpan>,
    op: UnaryOp,
    rhs: Expression,
    ctx: &Context<'_>,
) -> Result<JmlValue, EvalError> {
    let rhs = eval_expr(rhs, ctx)?;
    match op {
        UnaryOp::Minus => match rhs {
            JmlValue::Float(f) => Ok(f.negative().into()),
            JmlValue::Int(i) => Ok(i.negative().into()),
            _ => {
                let type_erro_kind = TypeErrorKind::InvalidUnaryOperator {
                    operator: "-".to_owned(),
                    right: rhs.type_of(),
                };

                Err(TypeError {
                    span: span.into(),
                    kind: type_erro_kind,
                })
            }?,
        },
        UnaryOp::Not => match rhs {
            JmlValue::Bool(b) => Ok(b.not().into()),
            _ => {
                let type_erro_kind = TypeErrorKind::InvalidUnaryOperator {
                    operator: "!".to_owned(),
                    right: rhs.type_of(),
                };

                Err(TypeError {
                    span: span.into(),
                    kind: type_erro_kind,
                })
            }?,
        },
    }
}
