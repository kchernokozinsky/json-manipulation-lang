use std::collections::HashMap;

use binary_op::eval_binary_op;
use if_expr::eval_if_expr;
use list_constructor::eval_list;
use object_constructor::eval_object;
use parser::ast::Expression;
use unary_op::eval_unary_op;

use crate::{
    context::Context,
    error::{EvalError, RuntimeError, TypeError, TypeErrorKind},
    jml_type::JmlType,
    value::JmlValue,
};

pub mod binary_op;
pub mod if_expr;
pub mod list_constructor;
pub mod object_constructor;
pub mod unary_op;

pub fn eval_expr(expression: Expression, ctx: &Context) -> Result<JmlValue, EvalError> {
    let Expression { l, r, node } = expression;
    let span = (l, r - l);
    match node {
        parser::ast::ExpressionKind::Null => Ok(JmlValue::null()),
        parser::ast::ExpressionKind::Float(v) => Ok(JmlValue::float(v)),
        parser::ast::ExpressionKind::Bool(v) => Ok(JmlValue::bool(v)),
        parser::ast::ExpressionKind::Int(v) => Ok(JmlValue::int(v)),
        parser::ast::ExpressionKind::String(v) => Ok(JmlValue::string(v)),
        parser::ast::ExpressionKind::Object(data) => eval_object(data, ctx),
        parser::ast::ExpressionKind::List(elems) => eval_list(elems, ctx),
        parser::ast::ExpressionKind::Variable(ident) => eval_variable(span, ident, ctx),
        parser::ast::ExpressionKind::IndexAccess { target, index } => {
            let index_l = index.l;
            let index_r = index.r;

            let index: i64 = eval_expr(*index, ctx)?.try_into().map_err(|e| TypeError {
                span: (index_l, index_r - index_l).into(),
                kind: e,
            })?;

            let target_l = target.l;
            let target_r = target.r;
            let target_val = eval_expr(*target, ctx)?;
            match &target_val {
                JmlValue::List(v) => Ok(v.access_by_index(index as usize)),
                JmlValue::String(_) => todo!(),
                _ => {
                    let type_error = TypeError {
                        span: (target_l, target_r - target_l).into(),
                        kind: TypeErrorKind::MismatchedTypes {
                            expected: vec![JmlType::List, JmlType::String],
                            found: target_val.type_of(),
                        },
                    };
                    Err(type_error)?
                }
            }
        }
        parser::ast::ExpressionKind::Selector { target, key } => {
            let target_l = target.l;
            let target_r = target.r;
            let val = eval_expr(*target, ctx)?;
            match val {
                JmlValue::Object(ob) => Ok(ob.access_by_key(key)),
                _ => {
                    let type_error = TypeError {
                        span: (target_l, target_r - target_l).into(),
                        kind: TypeErrorKind::MismatchedTypes {
                            expected: vec![JmlType::List, JmlType::String],
                            found: val.type_of(),
                        },
                    };
                    Err(type_error)?
                }
            }
        }
        parser::ast::ExpressionKind::UnaryOp { op, expr } => eval_unary_op(span, op, *expr, ctx),
        parser::ast::ExpressionKind::BinaryOp { op, lhs, rhs } => {
            eval_binary_op(span, op, *lhs, *rhs, ctx)
        }
        parser::ast::ExpressionKind::IfExpr {
            condition,
            then_branch,
            else_branch,
        } => eval_if_expr(*condition, *then_branch, *else_branch, ctx),
        parser::ast::ExpressionKind::Lambda { params, body } => todo!(),
        parser::ast::ExpressionKind::Apply { lambda, args } => todo!(),
    }
}

fn eval_variable(
    span: impl Into<miette::SourceSpan>,
    ident: &str,
    ctx: &Context,
) -> Result<JmlValue, EvalError> {
    match ctx.lookup_variable(ident) {
        Ok(bind) => match bind {
            crate::context::Binding::Expression(expr) => eval_expr(expr, ctx),
            crate::context::Binding::Value(value) => Ok(value),
        },
        Err(e) => Err(RuntimeError {
            span: span.into(),
            kind: e,
        }
        .into()),
    }
}
