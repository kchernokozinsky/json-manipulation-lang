#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Located<T> {
    pub l: usize,
    pub r: usize,
    pub node: T,
}

pub type Expression<'source> = Located<ExpressionKind<'source>>;

#[derive(Clone, Debug)]
pub enum ExpressionKind<'source> {
    Null,
    Float(f64),
    Bool(bool),
    Int(i64),
    String(&'source str),
}
