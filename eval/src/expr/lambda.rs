use std::rc::Rc;

use parser::ast::{Expression, Identifier};

use crate::{
    context::Context,
    errors::{EvalError, TypeError, TypeErrorKind},
    value::{
        lambda::{self, JmlLambda},
        JmlValue,
    },
};

use super::eval_expr;

pub(crate) fn eval_lambda_defenition<'source, I>(
    params: I,
    body: Expression<'source>,
) -> Result<JmlValue<'source>, EvalError>
where
    I: IntoIterator<Item = Identifier<'source>>,
{
    Ok(JmlLambda {
        params: params.into_iter().map(|e| e.node).collect(),
        body: lambda::LambdaBody::Common(body),
    }
    .into())
}

pub(crate) fn eval_lambda_application<'source, S>(
    span: S,
    lambda: Expression<'source>,
    args: Vec<Expression<'source>>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError>
where
    S: Into<miette::SourceSpan>,
{
    let target = eval_expr(lambda, ctx)?;
    match target {
        JmlValue::Lambda(JmlLambda { params, body }) => {
            if params.len() != args.len() {
                let type_error_kind = TypeErrorKind::ArgumentCountMismatch {
                    expected_count: params.len(),
                    actual_count: args.len(),
                };
                Err(TypeError {
                    kind: type_error_kind,
                    span: span.into(),
                }
                .into())
            } else {
                let result = match body {
                    lambda::LambdaBody::Common(body) => {
                        let mut local_context = Context::new_with_parent(Rc::new(ctx.clone()));
                        for (param, arg) in params.into_iter().zip(args.into_iter()) {
                            local_context.bind_with_value(param.to_owned(), eval_expr(arg, ctx)?);
                        }

                        eval_expr(body, &mut local_context)?
                    }
                    lambda::LambdaBody::Native(fun) => {
                        let mut evaluated_args = vec![];
                        for arg in args.into_iter() {
                            evaluated_args.push(eval_expr(arg, ctx)?);
                        }
                        fun(span.into(), evaluated_args, ctx)?
                    }
                };

                Ok(result)
            }
        }
        _ => todo!(),
    }
}

pub(crate) fn eval_lambda_application_with_evaluated_args<'source, S>(
    span: S,
    lambda: JmlValue<'source>,
    args: Vec<JmlValue<'source>>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError>
where
    S: Into<miette::SourceSpan>,
{
    let target = lambda;
    match target {
        JmlValue::Lambda(JmlLambda { params, body }) => {
            if params.len() != args.len() {
                let type_error_kind = TypeErrorKind::ArgumentCountMismatch {
                    expected_count: params.len(),
                    actual_count: args.len(),
                };
                Err(TypeError {
                    kind: type_error_kind,
                    span: span.into(),
                }
                .into())
            } else {
                let result = match body {
                    lambda::LambdaBody::Common(body) => {
                        let mut local_context = Context::new_with_parent(Rc::new(ctx.clone()));
                        for (param, arg) in params.into_iter().zip(args.into_iter()) {
                            local_context.bind_with_value(param.to_owned(), arg);
                        }

                        eval_expr(body, &mut local_context)?
                    }
                    lambda::LambdaBody::Native(fun) => fun(span.into(), args, ctx)?,
                };

                Ok(result)
            }
        }
        _ => todo!(),
    }
}
