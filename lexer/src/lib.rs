use logos::{Logos, SpannedIter};
use token::{errors::LexingError, Token};

pub mod token;

pub struct Lexer<'source> {
    token_stream: SpannedIter<'source, Token<'source>>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            token_stream: Token::lexer(source).spanned(),
        }
    }
}

impl<'source> From<&'source str> for Lexer<'source> {
    fn from(source: &'source str) -> Self {
        Lexer::new(source)
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<(usize, Token<'source>, usize), LexingError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
            .next()
            .map(|(token, span)| token.map(|token| (span.start, token, span.end)))
    }
}
