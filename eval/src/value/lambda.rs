use derive_more::Display;
use parser::ast::Expression;

pub type Identifier<'source> = &'source str;

#[derive(Debug, Clone, Display)]
#[display("lambda ({}) -> output", params.join(", "))]
pub struct JmlLambda<'source> {
    pub(crate) params: Vec<Identifier<'source>>,
    pub(crate) body: Expression<'source>,
}

impl<'source> PartialEq for JmlLambda<'source> {
    fn eq(&self, other: &Self) -> bool {
        self.params.len() == other.params.len()
    }
}

impl<'a> Eq for JmlLambda<'a> {}
