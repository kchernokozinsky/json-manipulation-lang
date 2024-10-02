use derive_more::{Debug, Display};
use parser::ast::Expression;
use std::rc::Rc;

use crate::{context::Context, errors::EvalError};

use super::JmlValue;

pub type Identifier<'source> = &'source str;

pub type NativeFunction<'source, Span> = dyn Fn(Span, Vec<JmlValue<'source>>, &mut Context<'source>) -> Result<JmlValue<'source>, EvalError>
    + 'source;

#[derive(Debug, Clone)]
pub enum LambdaBody<'source, S> {
    Common(Expression<'source>),
    #[debug("Native function")]
    Native(Rc<NativeFunction<'source, S>>),
}

#[derive(Debug, Clone, Display)]
#[display("lambda ({}) -> output", params.join(", "))]
pub struct JmlLambda<'source> {
    pub(crate) params: Vec<Identifier<'source>>,
    pub(crate) body: LambdaBody<'source, miette::SourceSpan>,
}

impl<'source> PartialEq for JmlLambda<'source> {
    fn eq(&self, other: &Self) -> bool {
        self.params.len() == other.params.len()
    }
}

impl<'a> Eq for JmlLambda<'a> {}
