use std::fs;

use eval::{context, eval};
use miette::Result;

fn main() -> Result<()> {
    let source = fs::read_to_string(
        "/Users/chernokozinskiy/Documents/Pets/json-manipulation-lang/examples/routing.jml",
    )
    .expect("Should have been able to read the file");

    let ast = parser::parse(&source).unwrap();
    let mut ctx = context::Context::new();

    let result = eval(ast, &mut ctx).map_err(|e| e.with_source_code(source))?;

    println!("{}", serde_json::to_string_pretty(&result).unwrap());

    Ok(())
}
