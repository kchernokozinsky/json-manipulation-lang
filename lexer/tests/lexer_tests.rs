#[cfg(test)]
mod tests {
    use lexer::{token::Token, Lexer};


    #[test]
    fn test_keyword_let() {
        let source = "let";
        let mut lexer = Lexer::new(source);

        assert_eq!(
            lexer.next(),
            Some(Ok((0, Token::Let, 3)))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_identifier() {
        let source = "myVar";
        let mut lexer = Lexer::new(source);

        assert_eq!(
            lexer.next(),
            Some(Ok((0, Token::Identifier("myVar"), 5)))
        );
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

        assert_eq!(
            lexer.next(),
            Some(Ok((0, Token::IntLiteral(12345), 5)))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_float_literal() {
        let source = "123.45";
        let mut lexer = Lexer::new(source);

        assert_eq!(
            lexer.next(),
            Some(Ok((0, Token::FloatLiteral(123.45), 6)))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_operator_plus() {
        let source = "+";
        let mut lexer = Lexer::new(source);

        assert_eq!(
            lexer.next(),
            Some(Ok((0, Token::Plus, 1)))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_combined_expression() {
        let source = r#"let x = 123 + 456.78;"#;
        let mut lexer = Lexer::new(source);

        assert_eq!(
            lexer.next(),
            Some(Ok((0, Token::Let, 3)))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok((4, Token::Identifier("x"), 5)))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok((6, Token::Assign, 7)))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok((8, Token::IntLiteral(123), 11)))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok((12, Token::Plus, 13)))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok((14, Token::FloatLiteral(456.78), 20)))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok((20, Token::Semicolon, 21)))
        );
        assert_eq!(lexer.next(), None);
    }


}