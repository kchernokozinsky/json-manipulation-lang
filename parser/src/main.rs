use lexer::Lexer;
use parser::jml;

fn main() {
    let source = r#"
a = 1
d = "some_string"
---

{
    "key1": a, 
    "key2": d,
    "key3": 1 
}"#;
    let lexer = Lexer::new(source);

    let ast = jml::JmlParser::new().parse(source, lexer);
    dbg!(ast.unwrap());
}
