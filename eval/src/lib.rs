use context::Context;
use expr::eval_expr;
use parser::ast::Jml;
use stdlib::define_std_lib;
use stmt::eval_stmt;
use value::JmlValue;

pub mod context;
pub mod errors;
pub mod expr;
pub mod jml_type;
pub mod stdlib;
pub mod stmt;
pub mod value;

pub fn eval_with_ctx<'source>(
    jml: Jml<'source>,
    ctx: &mut Context<'source>,
) -> miette::Result<JmlValue<'source>> {
    define_std_lib(ctx);
    for stmt in jml.header.into_iter() {
        eval_stmt(stmt, ctx)?;
    }
    eval_expr(jml.body, ctx).map_err(|e| e.into())
}

pub fn eval_with_source<'source>(
    jml: Jml<'source>,
    source: &'static str,
) -> miette::Result<JmlValue<'source>> {
    let mut ctx = context::Context::new();
    define_std_lib(&mut ctx);
    eval_with_ctx(jml, &mut ctx).map_err(|e| e.with_source_code(source))
}

pub fn eval_with_ctx_source<'source>(
    jml: Jml<'source>,
    source: &'static str,
    ctx: &mut Context<'source>,
) -> miette::Result<JmlValue<'source>> {
    define_std_lib(ctx);
    eval_with_ctx(jml, ctx).map_err(|e| e.with_source_code(source))
}
