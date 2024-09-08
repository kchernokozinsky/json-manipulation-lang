use logos::Lexer;

use super::{errors::LexingError, Token};

pub fn float<'source>(lex: &mut Lexer<'source, Token<'source>>) -> Result<f64, LexingError> {
    lex.slice().parse().map_err(|e| LexingError::InvalidFloat {
        source: lex.source().into(),
        span: (lex.span().start, lex.span().end - lex.span().start).into(),
        e,
    })
}

pub fn int<'source>(lex: &mut Lexer<'source, Token<'source>>) -> Result<i64, LexingError> {
    lex.slice()
        .parse()
        .map_err(|e| LexingError::InvalidInteger {
            source: lex.source().into(),
            span: (lex.span().start, lex.span().end - lex.span().start).into(),
            e,
        })
}

pub fn string<'source>(lex: &mut Lexer<'source, Token<'source>>) -> &'source str {
    lex.slice()[1..lex.slice().len() - 1].as_ref()
}

pub fn ident<'source>(lex: &mut Lexer<'source, Token<'source>>) -> &'source str {
    lex.slice()
}
