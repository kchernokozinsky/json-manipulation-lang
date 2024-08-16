use lalrpop_util::lalrpop_mod;
use lexer::Lexer;
// use lexer::token::Token;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub jml);

#[test]
fn test() {
    let source = r#""hello""#;
    let lexer = Lexer::new(source);
    
    assert!(jml::StringLiteralParser::new().parse(source, lexer).is_ok());
}
