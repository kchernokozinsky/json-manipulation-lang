use crate::{
    context::Context,
    errors::{EvalError, TypeError, TypeErrorKind},
    expr::lambda::eval_lambda_application_with_evaluated_args,
    jml_type::JmlType,
    value::{bool::JmlBool, list::JmlList, JmlValue},
};

pub fn map<'source>(
    span: miette::SourceSpan,
    args: Vec<JmlValue<'source>>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
    let list = match &args[0] {
        JmlValue::List(JmlList(l)) => l,
        _ => {
            let type_error_kind = TypeErrorKind::MismatchedTypes {
                expected: vec![JmlType::List],
                found: args[0].type_of(),
            };

            return Err(TypeError {
                kind: type_error_kind,
                span,
            }
            .into());
        }
    };

    let mut mapped_list: Vec<JmlValue> = vec![];

    for elem in list {
        mapped_list.push(eval_lambda_application_with_evaluated_args(
            span,
            args[1].clone(),
            vec![elem.clone()],
            ctx,
        )?);
    }

    Ok(JmlValue::List(mapped_list.into()))
}

pub fn filter<'source>(
    span: miette::SourceSpan,
    args: Vec<JmlValue<'source>>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
    let list = match &args[0] {
        JmlValue::List(JmlList(l)) => l,
        _ => {
            let type_error_kind = TypeErrorKind::MismatchedTypes {
                expected: vec![JmlType::List],
                found: args[0].type_of(),
            };

            return Err(TypeError {
                kind: type_error_kind,
                span,
            }
            .into());
        }
    };

    let mut filtered_list: Vec<JmlValue> = vec![];

    for elem in list {
        let result = eval_lambda_application_with_evaluated_args(
            span,
            args[1].clone(),
            vec![elem.clone()],
            ctx,
        )?;

        if let JmlValue::Bool(JmlBool(true)) = result {
            filtered_list.push(elem.clone());
        }
    }

    Ok(JmlValue::List(filtered_list.into()))
}

pub fn reduce<'source>(
    span: miette::SourceSpan,
    args: Vec<JmlValue<'source>>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
    let list = match &args[0] {
        JmlValue::List(JmlList(l)) => l,
        _ => {
            let type_error_kind = TypeErrorKind::MismatchedTypes {
                expected: vec![JmlType::List],
                found: args[0].type_of(),
            };

            return Err(TypeError {
                kind: type_error_kind,
                span,
            }
            .into());
        }
    };

    let mut accumulator = args[1].clone();

    for elem in list {
        accumulator = eval_lambda_application_with_evaluated_args(
            span,
            args[2].clone(),
            vec![elem.clone(), accumulator],
            ctx,
        )?;
    }

    Ok(accumulator)
}
