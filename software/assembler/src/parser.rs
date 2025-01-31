use std::collections::HashMap;
use isa::arch::{Instruction, Opcode, Register};
use crate::lexer::Token;
use crate::expression::Expression;

pub fn parse_line<'a>(mut tokens: &[Token<'a>]) -> Result<Option<Instruction>, String> {

    if let [] = tokens {
        return Ok(None); // empty line, which is fine.
    }

    let opcode: Opcode;

    if let [Token::Operation(op), ..] = tokens {
        opcode = *op;
        tokens = &tokens[1..];
    } else {
        return Err(format!("Expected Operation identifier as first token in line but got: {:?}", tokens[0]));
    }

    let operandlist_type = opcode.operandlist_type();
    let operation = match operandlist_type {
        isa::arch::OperandList::Empty => {
            if let [] = tokens {
                Some(Instruction {
                    opcode,
                    rd: Register::RZ,
                    ro1: Register::RZ,
                    ro2: Register::RZ,
                    immediate: 0,
                })
            } else {
                None
            }
        },
        isa::arch::OperandList::RdRo1Ro2 => {
            if let [Token::Register(rd), Token::Symbol(','), Token::Register(ro1), Token::Symbol(','), Token::Register(ro2)] = tokens {
                Some(Instruction {
                    opcode,
                    rd: *rd,
                    ro1: *ro1,
                    ro2: *ro2,
                    immediate: 0,
                })
            } else {
                None
            }
        },
        isa::arch::OperandList::RdRo1Imm8 => {
            if let [Token::Register(rd), Token::Symbol(','), Token::Register(ro1), Token::Symbol(','), Token::Number(immediate)] = tokens {
                Some(Instruction {
                    opcode,
                    rd: *rd,
                    ro1: *ro1,
                    ro2: Register::RZ,
                    immediate: *immediate as u16,
                })
            } else {
                None
            }
        },
        isa::arch::OperandList::RdBracketedRHRL => {
            if let [Token::Register(rd), Token::Symbol(','), Token::Symbol('['), Token::Register(Register::RH), Token::Symbol(':'), Token::Register(Register::RL), Token::Symbol(']')] = tokens {
                Some(Instruction {
                    opcode,
                    rd: *rd,
                    ro1: Register::RZ,
                    ro2: Register::RZ,
                    immediate: 0,
                })
            } else {
                None
            }
        },
        isa::arch::OperandList::RdBracketedImm16 => {
            if let [Token::Register(rd), Token::Symbol(','), Token::Symbol('['), Token::Number(immediate), Token::Symbol(']')] = tokens {
                Some(Instruction {
                    opcode,
                    rd: *rd,
                    ro1: Register::RZ,
                    ro2: Register::RZ,
                    immediate: *immediate as u16,
                })
            } else {
                None
            }
        },
        isa::arch::OperandList::BracketedRHRLRo2 => {
            if let [Token::Symbol('['), Token::Register(Register::RH), Token::Symbol(':'), Token::Register(Register::RL), Token::Symbol(']'), Token::Symbol(','), Token::Register(ro2)] = tokens {
                Some(Instruction {
                    opcode,
                    rd: Register::RZ,
                    ro1: Register::RZ,
                    ro2: *ro2,
                    immediate: 0,
                })
            } else {
                None
            }
        },
        isa::arch::OperandList::BracketedImm16Ro2 => {
            if let [Token::Symbol('['), Token::Number(immediate), Token::Symbol(']'), Token::Symbol(','), Token::Register(ro2)] = tokens {
                Some(Instruction {
                    opcode,
                    rd: Register::RZ,
                    ro1: Register::RZ,
                    ro2: *ro2,
                    immediate: *immediate as u16,
                })
            } else {
                None
            }
        }
    };

    match operation {
        Some(op) => Ok(Some(op)),
        None => Err(format!("Opcode {:?} expected an operand list of this format: '{:?}' but got: {:?}", opcode, operandlist_type, tokens)),
    }
}

fn resolve_labels<'a>(tokens: &[Token<'a>]) -> Result<Vec<Token<'a>>, String> {

    let mut label_map: HashMap<&str, usize> = HashMap::new();
    let mut resolved_tokens = Vec::new();

    // build label map
    let mut i = 0;
    let mut address = 0;
    while let Some(token) = tokens.get(i) {
        match token {
            Token::Identifier(label) => {
                if let Some(Token::Symbol(':')) = tokens.get(i + 1) {
                    if let Some(previous) = label_map.insert(label, address) {
                        return Err(format!("duplicate label: '{label}' ({previous:x})"));
                    }
                    i += 2;
                } else {
                    resolved_tokens.push(Token::Identifier(*label));
                    i += 1;
                }
            },
            Token::Symbol('$') => {
                resolved_tokens.push(Token::Number(address as i64));
                i += 1;
            }
            Token::Operation(op) => {
                resolved_tokens.push(Token::Operation(*op));
                address += 1;
                i += 1;
            }
            anything_else => {
                resolved_tokens.push(*anything_else);
                i += 1;
            }
        }
    }

    for token in resolved_tokens.iter_mut() {
        if let Token::Identifier(label) = token {
            match label_map.get(label) {
                Some(address) => *token = Token::Number(*address as i64),
                None => return Err(format!("undefined identifier: '{label}'")),
            }
        }
    }

    Ok(resolved_tokens)
}

fn evaluate_expressions<'a>(tokens: &mut Vec<Token<'a>>) -> Result<Vec<Token<'a>>, String> {
    for i in 0..tokens.len() {
        if let Some(Token::Symbol('{')) = tokens.get(i) {
            if let Some(closing_brace) = tokens.iter().position(|token| *token == Token::Symbol('}')) {
                let expression_tokens = &tokens[i + 1..closing_brace];
                let expression = Expression::from_lexemes(&expression_tokens[..])?.infix_to_postfix();
                let result = expression.evaluate()?;

                tokens.drain(i..closing_brace+1);
                tokens.insert(i, Token::Number(result));
            } else {
                return Err(format!("Expression starts here but doesn't have a closing brace: {:?}", &tokens[i..(i + 5).min(tokens.len() - 1)]));
            }
        }
    }

    Ok(tokens.iter().map(|x| *x).collect())
}

pub fn parse(mut tokens: Vec<Token>) -> Result<Vec<Instruction>, String> {
    
    tokens = resolve_labels(&tokens)?;
    tokens = evaluate_expressions(&mut tokens)?;
    let lines = tokens.split(|token| *token == Token::Symbol('\n')).enumerate();
    let mut instructions = Vec::new();

    for (line_index, tokens) in lines {
        match parse_line(tokens) {
            Ok(Some(parsed_line)) => { instructions.push(parsed_line); },
            Ok(None) => { /* empty line, don't care. */ }
            Err(error) => return Err(format!("error in line {}: {}", line_index + 1, error)),
        }
    }

    Ok(instructions)
}
