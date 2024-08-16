use errors::LexingError;
use literal::{float, ident, int, string};
use logos::Logos;

pub mod errors;
mod literal;

#[derive(Debug, Logos, PartialEq, Clone)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"#[^\n]*\n")]
pub enum Token<'source> {
    // Keywords
    #[token("let")]
    Let,

    #[token("null")]
    Null,

    #[token("filter")]
    Filter,

    #[token("map")]
    Map,

    #[token("mapobj")]
    MapObject,

    #[token("filterobj")]
    FilterObject,

    #[token("fn")]
    Fn,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("read")]
    Read,

    #[token("write")]
    Write,

    #[token("log")]
    Log,

    // Types
    #[token("String")]
    StringType,

    #[token("Float")]
    FloatType,

    #[token("Bool")]
    BoolType,

    #[token("Int")]
    IntType,

    #[token("Array")]
    ArrayType,

    #[token("Object")]
    ObjectType,

    #[token("Null")]
    NullType,

    // Identifiers and literals
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", callback=ident)]
    Identifier(&'source str),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, callback=string)]
    StringLiteral(&'source str),

    #[regex(r"[0-9][_0-9]*", callback=int, priority=2)]
    IntLiteral(i64),

    #[regex(r"(0|[1-9][0-9]*)(\.[0-9]+)?([Ee][\-+]?[0-9]+)?", callback=float, priority=1)]
    FloatLiteral(f64),

    #[token("false", |_| false)]
    #[token("true", |_| true)]
    BoolLiteral(bool),

    // Operators
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("==")]
    Equal,

    #[token("!=")]
    NotEqual,

    #[token("<")]
    LessThan,

    #[token(">")]
    GreaterThan,

    #[token("<=")]
    LessEqual,

    #[token(">=")]
    GreaterEqual,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("!")]
    Not,

    #[token("=")]
    Assign,

    // Symbols
    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("=>")]
    Arrow,

    // Comments
    #[regex(r"//[^\n]*", logos::skip)]
    LineComment,
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    #[test]
    fn test_keywords() {
        let mut lexer =
            Token::lexer("let null filter map mapobj filterobj fn if else read write log");

        assert_eq!(lexer.next(), Some(Ok(Token::Let)));
        assert_eq!(lexer.next(), Some(Ok(Token::Null)));
        assert_eq!(lexer.next(), Some(Ok(Token::Filter)));
        assert_eq!(lexer.next(), Some(Ok(Token::Map)));
        assert_eq!(lexer.next(), Some(Ok(Token::MapObject)));
        assert_eq!(lexer.next(), Some(Ok(Token::FilterObject)));
        assert_eq!(lexer.next(), Some(Ok(Token::Fn)));
        assert_eq!(lexer.next(), Some(Ok(Token::If)));
        assert_eq!(lexer.next(), Some(Ok(Token::Else)));
        assert_eq!(lexer.next(), Some(Ok(Token::Read)));
        assert_eq!(lexer.next(), Some(Ok(Token::Write)));
        assert_eq!(lexer.next(), Some(Ok(Token::Log)));
        assert_eq!(lexer.next(), None); // No more tokens
    }

    #[test]
    fn test_types() {
        let mut lexer = Token::lexer("String Float Bool Int Array Object Null");

        assert_eq!(lexer.next(), Some(Ok(Token::StringType)));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatType)));
        assert_eq!(lexer.next(), Some(Ok(Token::BoolType)));
        assert_eq!(lexer.next(), Some(Ok(Token::IntType)));
        assert_eq!(lexer.next(), Some(Ok(Token::ArrayType)));
        assert_eq!(lexer.next(), Some(Ok(Token::ObjectType)));
        assert_eq!(lexer.next(), Some(Ok(Token::NullType)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_operators() {
        let mut lexer = Token::lexer("+-*/ == != < > <= >= && || ! =");

        assert_eq!(lexer.next(), Some(Ok(Token::Plus)));
        assert_eq!(lexer.next(), Some(Ok(Token::Minus)));
        assert_eq!(lexer.next(), Some(Ok(Token::Star)));
        assert_eq!(lexer.next(), Some(Ok(Token::Slash)));
        assert_eq!(lexer.next(), Some(Ok(Token::Equal)));
        assert_eq!(lexer.next(), Some(Ok(Token::NotEqual)));
        assert_eq!(lexer.next(), Some(Ok(Token::LessThan)));
        assert_eq!(lexer.next(), Some(Ok(Token::GreaterThan)));
        assert_eq!(lexer.next(), Some(Ok(Token::LessEqual)));
        assert_eq!(lexer.next(), Some(Ok(Token::GreaterEqual)));
        assert_eq!(lexer.next(), Some(Ok(Token::And)));
        assert_eq!(lexer.next(), Some(Ok(Token::Or)));
        assert_eq!(lexer.next(), Some(Ok(Token::Not)));
        assert_eq!(lexer.next(), Some(Ok(Token::Assign)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_symbols() {
        let mut lexer = Token::lexer(". , : ; () [] {} =>");

        assert_eq!(lexer.next(), Some(Ok(Token::Dot)));
        assert_eq!(lexer.next(), Some(Ok(Token::Comma)));
        assert_eq!(lexer.next(), Some(Ok(Token::Colon)));
        assert_eq!(lexer.next(), Some(Ok(Token::Semicolon)));
        assert_eq!(lexer.next(), Some(Ok(Token::LParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::RParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::LBracket)));
        assert_eq!(lexer.next(), Some(Ok(Token::RBracket)));
        assert_eq!(lexer.next(), Some(Ok(Token::LBrace)));
        assert_eq!(lexer.next(), Some(Ok(Token::RBrace)));
        assert_eq!(lexer.next(), Some(Ok(Token::Arrow)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Token::lexer("foo bar _baz qux123");

        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("foo"))));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("bar"))));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("_baz"))));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("qux123"))));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_string_literals() {
        let mut lexer = Token::lexer(r#""hello" "world""#);

        assert_eq!(lexer.next(), Some(Ok(Token::StringLiteral("hello"))));
        assert_eq!(lexer.next(), Some(Ok(Token::StringLiteral("world"))));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_int_literals() {
        let mut lexer = Token::lexer("123 456 789");

        assert_eq!(lexer.next(), Some(Ok(Token::IntLiteral(123))));
        assert_eq!(lexer.next(), Some(Ok(Token::IntLiteral(456))));
        assert_eq!(lexer.next(), Some(Ok(Token::IntLiteral(789))));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_float_literals() {
        let mut lexer = Token::lexer("0.123 1.23 123.456 1e10 1E-10 1.23e+10");

        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(0.123))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(1.23))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(123.456))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(1e10))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(1E-10))));
        assert_eq!(lexer.next(), Some(Ok(Token::FloatLiteral(1.23e+10))));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_bool_literals() {
        let mut lexer = Token::lexer("true false");

        assert_eq!(lexer.next(), Some(Ok(Token::BoolLiteral(true))));
        assert_eq!(lexer.next(), Some(Ok(Token::BoolLiteral(false))));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_comments() {
        let mut lexer = Token::lexer("// This is a comment\nlet foo = 42; // Another comment");

        assert_eq!(lexer.next(), Some(Ok(Token::Let)));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("foo"))));
        assert_eq!(lexer.next(), Some(Ok(Token::Assign)));
        assert_eq!(lexer.next(), Some(Ok(Token::IntLiteral(42))));
        assert_eq!(lexer.next(), Some(Ok(Token::Semicolon)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_undefined_token() {
        let mut lexer = Token::lexer("@");

        match lexer.next() {
            Some(e) => assert!(e.is_err()),
            None => unreachable!(),
        }
    }
}
