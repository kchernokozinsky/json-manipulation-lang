use binary_op::eval_binary_op;
use if_expr::eval_if_expr;
use parser::ast::Expression;

use crate::{
    context::Context,
    error::{EvalError, RuntimeError},
    value::JmlValue,
};

pub mod binary_op;
pub mod if_expr;
pub mod list_constructor;
pub mod unary_op;

pub fn eval_expr<'a>(expression: Expression, ctx: &Context<'a>) -> Result<JmlValue, EvalError> {
    let Expression { l, r, node } = expression;
    match node {
        parser::ast::ExpressionKind::Null => Ok(JmlValue::null()),
        parser::ast::ExpressionKind::Float(v) => Ok(JmlValue::float(v)),
        parser::ast::ExpressionKind::Bool(v) => Ok(JmlValue::bool(v)),
        parser::ast::ExpressionKind::Int(v) => Ok(JmlValue::int(v)),
        parser::ast::ExpressionKind::String(v) => Ok(JmlValue::string(v)),
        parser::ast::ExpressionKind::Object(_) => todo!(),
        parser::ast::ExpressionKind::List(v) => todo!(),
        parser::ast::ExpressionKind::Variable(ident) => match ctx.lookup_variable(ident) {
            Ok(bind) => match bind {
                crate::context::Binding::Expression(expr) => eval_expr(expr, ctx),
                crate::context::Binding::Value(value) => Ok(value),
            },
            Err(e) => Err(RuntimeError {
                span: (l, r - l).into(),
                kind: e,
            }
            .into()),
        },
        parser::ast::ExpressionKind::IndexAccess { target, index } => todo!(),
        parser::ast::ExpressionKind::Selector { target, key } => todo!(),
        parser::ast::ExpressionKind::UnaryOp { op, expr } => todo!(),
        parser::ast::ExpressionKind::BinaryOp { op, lhs, rhs } => {
            eval_binary_op(op, *lhs, *rhs, ctx)
        }
        parser::ast::ExpressionKind::IfExpr {
            condition,
            then_branch,
            else_branch,
        } => eval_if_expr(*condition, *then_branch, *else_branch, ctx),
    }
}
