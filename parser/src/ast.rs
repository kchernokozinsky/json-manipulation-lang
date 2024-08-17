use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Located<T> {
    pub l: usize,
    pub r: usize,
    pub node: T,
}

#[derive(Clone, Debug)]
pub struct Jml<'source> {
    pub header: Vec<Statement<'source>>,
    pub body: Expression<'source>,
}

pub type Statement<'source> = Located<StatementKind<'source>>;

#[derive(Clone, Debug)]
pub enum StatementKind<'source> {
    Bind {
        identifier: Identifier<'source>,
        expression: Expression<'source>,
    },
}
pub type Identifier<'source> = Located<&'source str>;

pub type Expression<'source> = Located<ExpressionKind<'source>>;

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionKind<'source> {
    Null,
    Float(f64),
    Bool(bool),
    Int(i64),
    String(&'source str),
    Object(HashMap<&'source str, Expression<'source>>),
    List(Vec<Expression<'source>>),
    Variable(&'source str),
}
