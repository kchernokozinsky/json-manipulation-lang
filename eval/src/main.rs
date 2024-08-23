use eval::eval;

fn main() {
    let source = r#"
    a = 1
    d = "some_string"
    // c = \x y. x + y
    ---
    5 + 3 < 2  "#;
    let ast = parser::parse(source).unwrap();
    let value = eval(&ast);
    println!("{}", value.unwrap());
}
