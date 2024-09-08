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

pub fn eval_with_ctx<'a>(jml: Jml<'a>, ctx: &mut Context<'a>) -> miette::Result<JmlValue> {
    for stmt in jml.header.into_iter() {
        eval_stmt(stmt, ctx)?;
    }
    eval_expr(jml.body, ctx).map_err(|e| e.into())
}

pub fn eval_with_source(jml: Jml<'_>, source: &'static str) -> miette::Result<JmlValue> {
    let mut ctx = context::Context::new();

    eval_with_ctx(jml, &mut ctx).map_err(|e| e.with_source_code(source))
}

pub fn eval_with_ctx_source(jml: Jml<'_>, source: &'static str) -> miette::Result<JmlValue> {
    let mut ctx = context::Context::new();

    eval_with_ctx(jml, &mut ctx).map_err(|e| e.with_source_code(source))
}



