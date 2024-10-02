use std::rc::Rc;

use list::{filter, map, reduce};
use object::pluck;

use crate::{
    context::Context,
    errors::EvalError,
    value::{
        lambda::{JmlLambda, LambdaBody},
        JmlValue,
    },
};

pub mod list;
pub mod object;

pub(crate) fn define_std_lib(ctx: &mut Context) {
    ctx.bind_with_value(
        "log",
        JmlLambda {
            params: vec!["msg", "to_log"],
            body: LambdaBody::Native(Rc::new(log)),
        },
    );

    ctx.bind_with_value(
        "map",
        JmlLambda {
            params: vec!["list", "lambda"],
            body: LambdaBody::Native(Rc::new(map)),
        },
    );

    ctx.bind_with_value(
        "filter",
        JmlLambda {
            params: vec!["list", "lambda"],
            body: LambdaBody::Native(Rc::new(filter)),
        },
    );

    ctx.bind_with_value(
        "reduce",
        JmlLambda {
            params: vec!["list", "acc", "lambda"],
            body: LambdaBody::Native(Rc::new(reduce)),
        },
    );

    ctx.bind_with_value(
        "pluck",
        JmlLambda {
            params: vec!["object"],
            body: LambdaBody::Native(Rc::new(pluck)),
        },
    );
}

fn log<'source>(
    _: miette::SourceSpan,
    args: Vec<JmlValue<'source>>,
    _: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
    let value = args[1].clone();
    println!("{} : {}", args[0], args[1]);
    Ok(value)
}
