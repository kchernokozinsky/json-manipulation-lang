use parser::ast::{Statement, StatementKind};

use crate::{context::Context, error::EvalError};

pub fn eval_stmt<'a>(stmt: Statement<'a>, ctx: &mut Context<'a>) -> Result<(), EvalError> {
    let Statement { l, r, node } = stmt;

    match node {
        StatementKind::Bind {
            identifier,
            expression,
        } => ctx.bind(identifier.node.to_owned(), expression),
    }

    Ok(())
}
