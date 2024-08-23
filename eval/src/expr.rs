use binary_op::eval_binary_op;
use parser::ast::{Expression, Located};

use crate::{error::EvalError, value::JmlValue};

pub mod binary_op;
pub mod list_constructor;
pub mod unary_op;

pub fn eval_expr<'a>(expression: &Expression) -> Result<JmlValue, EvalError> {
    let Expression { l, r, node } = expression;
    match node {
        parser::ast::ExpressionKind::Null => Ok(JmlValue::null()),
        parser::ast::ExpressionKind::Float(v) => Ok(JmlValue::float(*v)),
        parser::ast::ExpressionKind::Bool(v) => Ok(JmlValue::bool(*v)),
        parser::ast::ExpressionKind::Int(v) => Ok(JmlValue::int(*v)),
        parser::ast::ExpressionKind::String(v) => Ok(JmlValue::string(*v)),
        parser::ast::ExpressionKind::Object(_) => todo!(),
        parser::ast::ExpressionKind::List(v) => todo!(),
        parser::ast::ExpressionKind::Variable(_) => todo!(),
        parser::ast::ExpressionKind::IndexAccess { target, index } => todo!(),
        parser::ast::ExpressionKind::Selector { target, key } => todo!(),
        parser::ast::ExpressionKind::UnaryOp { op, expr } => todo!(),
        parser::ast::ExpressionKind::BinaryOp { op, lhs, rhs } => eval_binary_op(*op, lhs, rhs),
    }
}
