use eval::{context, error, eval, value::JmlValue};
use miette::{Context, GraphicalReportHandler, IntoDiagnostic, Result};

fn main() -> Result<()> {
    let source = r#"
    a = true
    ---
    (b + 5.3)
    "#;
    let ast = parser::parse(source).unwrap();
    let mut ctx = context::Context::new();

    eval(ast, &mut ctx).map_err(|e| e.with_source_code(source))?;

    Ok(())
}
