#[cfg(test)]
mod tests {
    use lexer::{errors::LexingError, token::Token, Lexer};

    #[test]
    fn test_undefined_token() {
        let source = "@";
        let mut lexer = Lexer::new(source);

        match lexer.next() {
            Some(Err(LexingError::UndefinedToken)) => {}
            _ => panic!("Expected UndefinedToken error"),
        }
    }

    #[test]
    fn test_identifier() {
        let source = "myVar";
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Ok((0, Token::Identifier("myVar"), 5))));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_string_literal() {
        let source = r#""hello""#;
        let mut lexer = Lexer::new(source);

        assert_eq!(
            lexer.next(),
            Some(Ok((0, Token::StringLiteral("hello"), 7)))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_int_literal() {
        let source = "12345";
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Ok((0, Token::IntLiteral(12345), 5))));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_float_literal() {
        let source = "123.45";
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Ok((0, Token::FloatLiteral(123.45), 6))));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_operator_plus() {
        let source = "+";
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Ok((0, Token::Plus, 1))));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_combined_expression() {
        let source = r#"x = 123 + 456.78"#;
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Ok((0, Token::Identifier("x"), 1))));
        assert_eq!(lexer.next(), Some(Ok((2, Token::Assign, 3))));
        assert_eq!(lexer.next(), Some(Ok((4, Token::IntLiteral(123), 7))));
        assert_eq!(lexer.next(), Some(Ok((8, Token::Plus, 9))));
        assert_eq!(
            lexer.next(),
            Some(Ok((10, Token::FloatLiteral(456.78), 16)))
        );
        assert_eq!(lexer.next(), None);
    }
}
