use crate::lexer;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Token {
    Constant(i64),
    ShiftLeft,
    ShiftRight,
    BinaryNOT,
    BinaryAND,
    BinaryOR,
    Multiplication,
    Division,
    Modulo,
    Addition,
    Subtraction,
    ParenthesisLeft,
    ParenthesisRight,
}

#[derive(PartialEq, Debug, Eq)]
pub struct Expression {
    pub tokens: Vec<Token>
}

impl Token {
    pub fn is_operator(self) -> bool {
        match self {
            Token::ShiftLeft        => true,
            Token::ShiftRight       => true,
            Token::BinaryNOT        => true,
            Token::BinaryAND        => true,
            Token::BinaryOR         => true,
            Token::Multiplication   => true,
            Token::Division         => true,
            Token::Modulo           => true,
            Token::Addition         => true,
            Token::Subtraction      => true,
            _                       => false
        }
    }
}

impl Expression {
    pub fn from_lexemes(lexemes: &[lexer::Token]) -> Result<Self, String> {
        let mut tokens = Vec::new();
        let mut i = 0;

        while i < lexemes.len() {
            let lexeme = lexemes.get(i).unwrap();
            let next = lexemes.get(i + 1);

            tokens.push(match lexeme {
                lexer::Token::Number(num) => {
                    i += 1;
                    Ok(Token::Constant(*num))
                },
                lexer::Token::Symbol('<') if Some(&lexer::Token::Symbol('<')) == next => {
                    i += 2;
                    Ok(Token::ShiftLeft)
                },
                lexer::Token::Symbol('>') if Some(&lexer::Token::Symbol('>')) == next => {
                    i += 2;
                    Ok(Token::ShiftRight)
                },
                lexer::Token::Symbol('~') => {
                    i += 1;
                    Ok(Token::BinaryNOT)
                },
                lexer::Token::Symbol('&') => {
                    i += 1;
                    Ok(Token::BinaryAND)
                },
                lexer::Token::Symbol('|') => {
                    i += 1;
                    Ok(Token::BinaryOR)
                },
                lexer::Token::Symbol('*') => {
                    i += 1;
                    Ok(Token::Multiplication)
                },
                lexer::Token::Symbol('/') => {
                    i += 1;
                    Ok(Token::Division)
                },
                lexer::Token::Symbol('%') => {
                    i += 1;
                    Ok(Token::Modulo)
                },
                lexer::Token::Symbol('+') => {
                    i += 1;
                    Ok(Token::Addition)
                },
                lexer::Token::Symbol('-') => {
                    i += 1;
                    Ok(Token::Subtraction)
                },
                lexer::Token::Symbol('(') => {
                    i += 1;
                    Ok(Token::ParenthesisLeft)
                },
                lexer::Token::Symbol(')') => {
                    i += 1;
                    Ok(Token::ParenthesisRight)
                },
                _ => Err(format!("Expected number or mathematical operator but got: {:?}", lexeme))
            }?);
        }

        Ok(Expression { tokens })
    }

    fn process_token(token: Token, postfix: &mut Vec<Token>, shunt: &mut Vec<Token>) {
        match token {
            Token::Constant(_)      => postfix.push(token),
            Token::ShiftLeft        |
            Token::ShiftRight       |
            Token::BinaryNOT        |
            Token::BinaryAND        |
            Token::BinaryOR         |
            Token::Multiplication   |
            Token::Division         |
            Token::Modulo           |
            Token::Addition         |
            Token::Subtraction      => {
                while !shunt.is_empty() && shunt.last().unwrap().is_operator() && shunt.last().unwrap() <= &token {
                    postfix.push(shunt.pop().unwrap());
                }
                shunt.push(token);
            }
            Token::ParenthesisLeft => {
                shunt.push(token);
            },
            Token::ParenthesisRight => {
                while !shunt.is_empty() && *shunt.last().unwrap() != Token::ParenthesisLeft {
                    postfix.push(shunt.pop().unwrap());
                }
                shunt.pop();
            },
        }
    }

    fn execute_token(token: Token, numeric_stack: &mut Vec<i64>) -> Result<(), String> {
        match token {
            Token::Constant(num) => numeric_stack.push(num),
            Token::ShiftLeft => {
                let b = numeric_stack.pop().ok_or("Missing operand 0 for <")?;
                let a = numeric_stack.pop().ok_or("Missing operand 1 for <")?;
                numeric_stack.push(a << b);
            },
            Token::ShiftRight => {
                let b = numeric_stack.pop().ok_or("Missing operand 0 for >")?;
                let a = numeric_stack.pop().ok_or("Missing operand 1 for >")?;
                numeric_stack.push(a >> b);
            },
            Token::BinaryNOT => {
                let a = numeric_stack.pop().ok_or("Missing operand 0 for ~")?;
                numeric_stack.push(!a);
            },
            Token::BinaryAND => {
                let b = numeric_stack.pop().ok_or("Missing operand 0 for &")?;
                let a = numeric_stack.pop().ok_or("Missing operand 1 for &")?;
                numeric_stack.push(a & b);
            },
            Token::BinaryOR => {
                let b = numeric_stack.pop().ok_or("Missing operand 0 for |")?;
                let a = numeric_stack.pop().ok_or("Missing operand 1 for |")?;
                numeric_stack.push(a | b);
            },
            Token::Multiplication => {
                let b = numeric_stack.pop().ok_or("Missing operand 0 for *")?;
                let a = numeric_stack.pop().ok_or("Missing operand 1 for *")?;
                numeric_stack.push(a * b);
            },
            Token::Division => {
                let b = numeric_stack.pop().ok_or("Missing operand 0 for /")?;
                let a = numeric_stack.pop().ok_or("Missing operand 1 for /")?;
                numeric_stack.push(a / b);
            },
            Token::Modulo => {
                let b = numeric_stack.pop().ok_or("Missing operand 0 for %")?;
                let a = numeric_stack.pop().ok_or("Missing operand 1 for %")?;
                numeric_stack.push(a % b);
            },
            Token::Addition => {
                let b = numeric_stack.pop().ok_or("Missing operand 0 for +")?;
                let a = numeric_stack.pop().ok_or("Missing operand 1 for +")?;
                numeric_stack.push(a + b);
            },
            Token::Subtraction => {
                let b = numeric_stack.pop().ok_or("Missing operand 0 for -")?;
                let a = numeric_stack.pop().ok_or("Missing operand 1 for -")?;
                numeric_stack.push(a - b);
            },
            _ => {
                return Err(format!("Invalid Token in expression: {:?}", token))
            }
        }

        Ok(())
    }

    pub fn infix_to_postfix(&self) -> Self {
        let mut postfix = Vec::new();
        let mut shunt = Vec::new();

        for token in self.tokens.iter() {
            Self::process_token(*token, &mut postfix, &mut shunt);
        }

        while let Some(token) = shunt.pop() {
            postfix.push(token);
        }

        Self { tokens: postfix }
    }

    pub fn evaluate(&self) -> Result<i64, String> {
        let mut execution_stack: Vec<Token> = Vec::new();
        let mut numeric_stack: Vec<i64> = Vec::new();

        execution_stack.extend(self.tokens.iter());
        execution_stack.reverse();

        while let Some(token) = execution_stack.pop() {
            Self::execute_token(token, &mut numeric_stack)?;
        }

        numeric_stack.pop().ok_or("Expression yields no value".to_string())
    }
}
