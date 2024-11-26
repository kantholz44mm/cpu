use std::collections::HashMap;
use isa::arch::{DoubleWord, FlagWriteMode, Instruction, Opcode, OperandSelect, Register, Word};
use crate::lexer::Token;

fn parse_line<'a>(tokens: &[Token<'a>]) -> Result<Option<Instruction>, String> {

    // optionally, an instruction
    match tokens {
        [] => Ok(None), // empty line, no instruction
        [Token::Operation(Opcode::HCF)] => Ok(Some(Instruction { opcode: Opcode::HCF, ..Default::default() })),
        [
            Token::Operation(opcode),
            modifiers@..,
            Token::Register(rd),
            Token::Symbol(','),
            Token::Register(ro1),
            Token::Symbol(','),
            Token::Register(ro2)
        ] if opcode.is_arithmetic() => {
            let flagmode = match modifiers {
                [] => FlagWriteMode::WriteFlags,
                [Token::Symbol('*')] => FlagWriteMode::DontWriteFlags,
                _ => { return Err(format!("Expected modifier or parameter list after operation '{:?}' but got {:?}", opcode, &tokens[..tokens.len().min(8)])); }
            };

            Ok(Some(Instruction {
                opcode: *opcode,
                operand: OperandSelect::OperandRegister,
                flags: flagmode,
                rd: *rd,
                ro1: *ro1,
                ro2: *ro2,
                immediate: 0x0000,
            }))
        },
        [
            Token::Operation(opcode),
            modifiers@..,
            Token::Register(rd),
            Token::Symbol(','),
            Token::Register(ro1),
            Token::Symbol(','),
            Token::Number(immediate)
        ] if opcode.is_arithmetic() => {
            let flagmode = match modifiers {
                [] => FlagWriteMode::WriteFlags,
                [Token::Symbol('*')] => FlagWriteMode::DontWriteFlags,
                _ => { return Err(format!("Expected modifier or parameter list after operation '{:?}' but got {:?}", opcode, &tokens[..tokens.len().min(8)])); }
            };
            Ok(Some(Instruction {
                opcode: *opcode,
                operand: OperandSelect::OperandImmediate,
                flags: flagmode,
                rd: *rd,
                ro1: *ro1,
                ro2: Register::RZ,
                immediate: *immediate as Word as DoubleWord,
            }))
        },
        [
            Token::Operation(Opcode::LW),
            Token::Register(rd),
            Token::Symbol(','),
            Token::Symbol('['),
            Token::Register(Register::RH),
            Token::Symbol(':'),
            Token::Register(Register::RL),
            Token::Symbol(']')
        ] => {
            Ok(Some(Instruction {
                opcode: Opcode::LW,
                operand: OperandSelect::OperandRegister,
                rd: *rd,
                ..Default::default()
            }))
        },
        [
            Token::Operation(Opcode::LW),
            Token::Register(rd),
            Token::Symbol(','),
            Token::Symbol('['),
            Token::Number(address),
            Token::Symbol(']')
        ] => {
            Ok(Some(Instruction {
                opcode: Opcode::LW,
                operand: OperandSelect::OperandImmediate,
                rd: *rd,
                immediate: *address as DoubleWord,
                ..Default::default()
            }))
        },
        [
            Token::Operation(Opcode::SW),
            Token::Symbol('['),
            Token::Register(Register::RH),
            Token::Symbol(':'),
            Token::Register(Register::RL),
            Token::Symbol(']'),
            Token::Symbol(','),
            Token::Register(ro2)
        ] => {
            Ok(Some(Instruction {
                opcode: Opcode::SW,
                operand: OperandSelect::OperandRegister,
                ro2: *ro2,
                ..Default::default()
            }))
        },
        [
            Token::Operation(Opcode::SW),
            Token::Symbol('['),
            Token::Number(address),
            Token::Symbol(']'),
            Token::Symbol(','),
            Token::Register(ro2)
        ] => {
            Ok(Some(Instruction {
                opcode: Opcode::SW,
                operand: OperandSelect::OperandImmediate,
                ro2: *ro2,
                immediate: *address as DoubleWord,
                ..Default::default()
            }))
        },
        [
            Token::Operation(opcode),
            Token::Register(ro2)
        ] if opcode.is_branch() => {
            Ok(Some(Instruction {
                opcode: *opcode,
                operand: OperandSelect::OperandRegister,
                ro2: *ro2,
                ..Default::default()
            }))
        },
        [
            Token::Operation(opcode),
            Token::Number(address),
            Token::Symbol(','),
            Token::Register(ro2)
        ] if opcode.is_branch() => {
            Ok(Some(Instruction {
                opcode: *opcode,
                operand: OperandSelect::OperandImmediate,
                immediate: *address as DoubleWord,
                ro2: *ro2,
                ..Default::default()
            }))
        },
        [
            Token::Operation(opcode),
            Token::Register(Register::RH),
            Token::Symbol(':'),
            Token::Register(Register::RL),
            Token::Symbol(','),
            Token::Register(ro2)
        ] if opcode.is_branch() => {
            Ok(Some(Instruction {
                opcode: *opcode,
                operand: OperandSelect::OperandRegister,
                ro2: *ro2,
                ..Default::default()
            }))
        },
        [
            Token::Operation(Opcode::LA),
            Token::Number(immediate)
        ] => {
            Ok(Some(Instruction {
                opcode: Opcode::LA,
                operand: OperandSelect::OperandImmediate,
                immediate: *immediate as DoubleWord,
                ..Default::default()
            }))
        },
        [Token::Operation(opcode), paramlist @..] => Err(format!("invalid parameters for Opcode {opcode:?}: {:?}", paramlist)),
        _ => Err(format!("invalid input sequence: {:?}", &tokens[..tokens.len().min(8)]))
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
                if let Some([Token::Symbol('('), Token::Number(offset), Token::Symbol(')')]) = tokens.get(i+1..i+4) {
                    resolved_tokens.push(Token::Number(address as i64 + offset));
                    i += 4;
                } else {
                    resolved_tokens.push(Token::Number(address as i64));
                    i += 1;
                }
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

pub fn parse(mut tokens: Vec<Token>) -> Result<Vec<Instruction>, String> {
    
    tokens = resolve_labels(&tokens)?;
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
