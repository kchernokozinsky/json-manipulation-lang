use context::Context;
use expr::eval_expr;
use parser::ast::Jml;
use stmt::eval_stmt;
use value::JmlValue;

pub mod context;
pub mod error;
pub mod expr;
pub mod jml_type;
pub mod stmt;
pub mod value;

pub fn eval<'a>(jml: Jml<'a>, ctx: &mut Context<'a>) -> miette::Result<JmlValue> {
    for stmt in jml.header.into_iter() {
        eval_stmt(stmt, ctx)?;
    }
    eval_expr(jml.body, ctx).map_err(|e| e.into())
}
