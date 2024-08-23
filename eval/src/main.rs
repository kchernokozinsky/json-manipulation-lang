use eval::{context, eval};

fn main() {
    let source = r#"
    a = 1
    ---
    a + 3  "#;
    let ast = parser::parse(source).unwrap();
    let mut ctx = context::Context::new();
    let value = eval(ast, &mut ctx);
    println!("{}", value.unwrap());
}
