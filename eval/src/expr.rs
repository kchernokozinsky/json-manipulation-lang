use binary_op::eval_binary_op;
use if_expr::eval_if_expr;
use lambda::{eval_lambda_application, eval_lambda_defenition};
use list_constructor::eval_list;
use object_constructor::eval_object;
use parser::ast::Expression;
use unary_op::eval_unary_op;

use crate::{
    context::Context,
    errors::{EvalError, RuntimeError, TypeError, TypeErrorKind},
    jml_type::JmlType,
    value::JmlValue,
};

pub mod binary_op;
pub mod if_expr;
pub mod lambda;
pub mod list_constructor;
pub mod object_constructor;
pub mod unary_op;

pub fn eval_expr<'source>(
    expression: Expression<'source>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
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
            eval_index_access(*target, *index, ctx)
        }
        parser::ast::ExpressionKind::Selector { target, key } => eval_selector(*target, key, ctx),
        parser::ast::ExpressionKind::UnaryOp { op, expr } => eval_unary_op(span, op, *expr, ctx),
        parser::ast::ExpressionKind::BinaryOp { op, lhs, rhs } => {
            eval_binary_op(span, op, *lhs, *rhs, ctx)
        }
        parser::ast::ExpressionKind::IfExpr {
            condition,
            then_branch,
            else_branch,
        } => eval_if_expr(*condition, *then_branch, *else_branch, ctx),
        parser::ast::ExpressionKind::Lambda { params, body } => {
            eval_lambda_defenition(params, *body)
        }
        parser::ast::ExpressionKind::Apply { lambda, args } => {
            eval_lambda_application(span, *lambda, args, ctx)
        }
    }
}

fn eval_variable<'source, S, I>(
    span: S,
    ident: I,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError>
where
    S: Into<miette::SourceSpan>,
    I: AsRef<str>,
{
    match ctx.lookup_variable(ident) {
        Ok(bind) => match bind {
            crate::context::Binding::Expression(expr) => eval_expr(expr, ctx),
            crate::context::Binding::Value(value) => Ok(value.clone()),
        },
        Err(e) => Err(RuntimeError {
            span: span.into(),
            kind: e,
        }
        .into()),
    }
}

fn eval_selector<'source, I>(
    target: Expression<'source>,
    key: I,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError>
where
    I: AsRef<str>,
{
    let target_l = target.l;
    let target_r = target.r;
    let val = eval_expr(target, ctx)?;
    match val {
        JmlValue::Object(ob) => Ok(ob.access_by_key(key.as_ref())),
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

fn eval_index_access<'source>(
    target: Expression<'source>,
    index: Expression<'source>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
    let index_l = index.l;
    let index_r = index.r;

    let index: i64 = eval_expr(index, ctx)?.try_into().map_err(|e| TypeError {
        span: (index_l, index_r - index_l).into(),
        kind: e,
    })?;

    let target_l = target.l;
    let target_r = target.r;
    let target_val = eval_expr(target, ctx)?;
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
