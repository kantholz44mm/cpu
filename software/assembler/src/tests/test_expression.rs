#[cfg(test)]
mod tests {
    use crate::expression::Expression;
    use crate::{expression, lexer};

    #[test]
    pub fn test_empty_expression() {
        assert_eq!(Ok(Expression { tokens: Vec::new() }), Expression::from_lexemes(&[][..]));
    }

    #[test]
    pub fn test_parse_basic_expression() {
        let expected_infix = Expression {
            tokens: vec![
                expression::Token::Constant(42),
                expression::Token::Addition,
                expression::Token::Constant(43),
            ]
        };
        let expected_postfix = Expression {
            tokens: vec![
                expression::Token::Constant(42),
                expression::Token::Constant(43),
                expression::Token::Addition,
            ]
        };
        let lexemes = [
            lexer::Token::Number(42),
            lexer::Token::Symbol('+'),
            lexer::Token::Number(43),
        ];

        let actual = Expression::from_lexemes(&lexemes[..]);
        assert_eq!(Ok(expected_infix), actual);

        let actual_postfix = actual.unwrap().infix_to_postfix();
        assert_eq!(expected_postfix, actual_postfix);
        
    }

    #[test]
    pub fn test_parse_complex_expression() {
        // 42 + (-12 * 3 << 4)
        let expected_infix = Expression {
            tokens: vec![
                expression::Token::Constant(42),
                expression::Token::Addition,
                expression::Token::ParenthesisLeft,
                expression::Token::Constant(-12),
                expression::Token::Multiplication,
                expression::Token::Constant(3),
                expression::Token::ShiftLeft,
                expression::Token::Constant(4),
                expression::Token::ParenthesisRight,
            ]
        };
        let expected_postfix = Expression {
            tokens: vec![
                expression::Token::Constant(42),
                expression::Token::Constant(-12),
                expression::Token::Constant(3),
                expression::Token::Constant(4),
                expression::Token::ShiftLeft,
                expression::Token::Multiplication,
                expression::Token::Addition,
            ]
        };
        let lexemes = [
            lexer::Token::Number(42),
            lexer::Token::Symbol('+'),
            lexer::Token::Symbol('('),
            lexer::Token::Number(-12),
            lexer::Token::Symbol('*'),
            lexer::Token::Number(3),
            lexer::Token::Symbol('<'),
            lexer::Token::Symbol('<'),
            lexer::Token::Number(4),
            lexer::Token::Symbol(')'),
        ];

        let actual = Expression::from_lexemes(&lexemes[..]);
        assert_eq!(Ok(expected_infix), actual);

        let actual_postfix = actual.unwrap().infix_to_postfix();
        assert_eq!(expected_postfix, actual_postfix);
    }
}