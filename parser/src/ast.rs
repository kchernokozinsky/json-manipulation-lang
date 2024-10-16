#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Key<'source> {
    Ident(Identifier<'source>),
    Expression(Expression<'source>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionKind<'source> {
    Null,
    Float(f64),
    Bool(bool),
    Int(i64),
    String(&'source str),
    Object(Vec<(Key<'source>, Expression<'source>)>),
    List(Vec<Expression<'source>>),
    Variable(&'source str),
    IndexAccess {
        target: Box<Expression<'source>>,
        index: Box<Expression<'source>>,
    },
    Selector {
        target: Box<Expression<'source>>,
        key: &'source str,
    },

    UnaryOp {
        op: UnaryOp,
        expr: Box<Expression<'source>>,
    },

    BinaryOp {
        op: BinaryOp,
        lhs: Box<Expression<'source>>,
        rhs: Box<Expression<'source>>,
    },

    IfExpr {
        condition: Box<Expression<'source>>,
        then_branch: Box<Expression<'source>>,
        else_branch: Box<Expression<'source>>,
    },
    Lambda {
        params: Vec<Identifier<'source>>,
        body: Box<Expression<'source>>,
    },
    Apply {
        lambda: Box<Expression<'source>>,
        args: Vec<Expression<'source>>,
    },
}

impl<'source> Eq for Expression<'source> {}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum BinaryOp {
    EQ,
    NE,
    GT,
    LT,
    GE,
    LE,
    Sum,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    And,
    Or,
    Concat,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Minus,
    Not,
}
