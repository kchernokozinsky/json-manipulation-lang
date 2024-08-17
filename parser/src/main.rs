use lexer::Lexer;
use parser::jml;

fn main() {
    let source = r#" a = true b = 3 c = b d = a ---  {}"#;
    let lexer = Lexer::new(source);

    let ast = jml::JmlParser::new().parse(source, lexer);
    dbg!(ast.unwrap());
}
