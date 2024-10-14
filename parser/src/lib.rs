use lalrpop_util::{lalrpop_mod, ParseError};
use lexer::{errors::LexingError, token::Token, Lexer};

pub mod ast;
lalrpop_mod!(
    #[allow(clippy::all, unused_variables, unused_imports)]
    #[rustfmt::skip]
    pub jml);

pub fn parse(source: &str) -> Result<ast::Jml<'_>, ParseError<usize, Token<'_>, LexingError>> {
    let lexer = Lexer::new(source);
    jml::JmlParser::new().parse(source, lexer)
}

#[cfg(test)]
mod tests {
    use super::jml;
    use crate::ast::{ExpressionKind, StatementKind};
    use lexer::Lexer;
    #[test]
    fn test_parse_jml() {
        let source = r#"
            x = 42
            y = "hello"
            --- 
            y
            "#;

        let lexer = Lexer::new(source);
        let jml = jml::JmlParser::new().parse(source, lexer).unwrap();
        assert_eq!(jml.header.len(), 2);
        match &jml.header[0].node {
            StatementKind::Bind {
                identifier,
                expression,
            } => {
                assert_eq!(identifier.node, "x");
                if let ExpressionKind::Int(value) = expression.node {
                    assert_eq!(value, 42);
                } else {
                    panic!("Expected an Int expression");
                }
            }
        }

        match &jml.header[1].node {
            StatementKind::Bind {
                identifier,
                expression,
            } => {
                assert_eq!(identifier.node, "y");
                if let ExpressionKind::String(value) = expression.node {
                    assert_eq!(value, "hello");
                } else {
                    panic!("Expected a String expression");
                }
            }
        }

        if let ExpressionKind::Variable(var) = jml.body.node {
            assert_eq!(var, "y");
        } else {
            panic!("Expected a Variable expression");
        }
    }

    #[test]
    fn test_parse_statement() {
        let source = "x = 42";
        let lexer = Lexer::new(source);
        let statement = jml::StatementParser::new().parse(source, lexer).unwrap();
        match statement.node {
            StatementKind::Bind {
                identifier,
                expression,
            } => {
                assert_eq!(identifier.node, "x");
                if let ExpressionKind::Int(value) = expression.node {
                    assert_eq!(value, 42);
                } else {
                    panic!("Expected an Int expression");
                }
            }
        }
    }

    #[test]
    fn test_parse_expression() {
        let source = "42";
        let lexer = Lexer::new(source);
        let expression = jml::ExpressionParser::new().parse(source, lexer).unwrap();

        if let ExpressionKind::Int(value) = expression.node {
            assert_eq!(value, 42);
        } else {
            panic!("Expected an Int expression");
        }
    }

    #[test]
    fn test_parse_object() {
        let source = r#"{"key1": 42, "key2": "value"}"#;

        let lexer = Lexer::new(source);
        let expression = jml::ExpressionParser::new().parse(source, lexer).unwrap();

        if let ExpressionKind::Object(map) = expression.node {
            assert_eq!(map.len(), 2);
            if let ExpressionKind::Int(value) = map[0].1.node {
                assert_eq!(value, 42);
            } else {
                panic!("Expected an Int expression for key1");
            }

            if let ExpressionKind::String(value) = map[1].1.node {
                assert_eq!(value, "value");
            } else {
                panic!("Expected a String expression for key2");
            }
        } else {
            panic!("Expected an Object expression");
        }
    }

    #[test]
    fn test_parse_list() {
        let source = "[1, 2, 3]";
        let lexer = Lexer::new(source);
        let expression = jml::ExpressionParser::new().parse(source, lexer).unwrap();

        if let ExpressionKind::List(vec) = expression.node {
            assert_eq!(vec.len(), 3);
            assert_eq!(vec[0].node, ExpressionKind::Int(1));
            assert_eq!(vec[1].node, ExpressionKind::Int(2));
            assert_eq!(vec[2].node, ExpressionKind::Int(3));
        } else {
            panic!("Expected a List expression");
        }
    }
}
