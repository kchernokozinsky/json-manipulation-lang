use parser::ast::{Expression, Identifier};

use crate::{
    context::Context,
    error::{EvalError, TypeError, TypeErrorKind},
    value::{lambda::JmlLambda, JmlValue},
};

use super::eval_expr;

pub(crate) fn eval_lambda_defenition<'source>(
    params: Vec<Identifier<'source>>,
    body: Expression<'source>,
) -> Result<JmlValue<'source>, EvalError> {
    Ok(JmlLambda {
        params: params.iter().map(|e| e.node).collect(),
        body,
    }
    .into())
}

pub(crate) fn eval_lambda_application<'source>(
    span: impl Into<miette::SourceSpan>,
    lambda: Expression<'source>,
    args: Vec<Expression<'source>>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
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
                let mut local_context = Context::default();
                for (param, arg) in params.into_iter().zip(args.into_iter()) {

                    local_context.bind_with_value(param.to_owned(), eval_expr(arg, ctx)?);
                }
                ctx.add_local_ctx(local_context);

                let result = eval_expr(body, ctx)?;

                ctx.pop_local_ctx();

                Ok(result)
            }
        }
        _ => todo!(),
    }
}
