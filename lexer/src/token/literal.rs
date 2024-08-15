use std::num::{ParseFloatError, ParseIntError};

use logos::Lexer;

use super::Token;

pub fn float<'source>(lex: &mut Lexer<'source, Token<'source>>) -> Result<f64, ParseFloatError> {
    lex.slice().parse()
}

pub fn int<'source>(lex: &mut Lexer<'source, Token<'source>>) -> Result<i64, ParseIntError> {
    lex.slice().parse()
}

pub fn string<'source>(lex: &mut Lexer<'source, Token<'source>>) -> &'source str {
    lex.slice()[1..lex.slice().len() - 1].as_ref()
}

pub fn ident<'source>(lex: &mut Lexer<'source, Token<'source>>) -> &'source str {
    lex.slice()
}
