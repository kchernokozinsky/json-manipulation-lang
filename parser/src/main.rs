use lexer::Lexer;
use parser::jml;

fn main() {
    let source = r#"false"#;
    let lexer = Lexer::new(source);
    
    let ast = jml::LiteralParser::new().parse(source, lexer);
    dbg!(ast.unwrap());
}