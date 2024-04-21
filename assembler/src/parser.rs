use std::collections::HashMap;

use crate::{arch::{Condition, DoubleWord, Instruction, Operation, Register}, lexer::Token};

#[derive(Clone, Debug)]
pub enum Statement {
    Empty,
    Label(String),
    Instruction(Instruction)
}

#[derive(Clone, Debug)]
pub struct Program {
    pub labels: HashMap<String, DoubleWord>,
    pub instructions: Vec<ProgramInstruction>
}

#[derive(Clone, Debug)]
pub struct ProgramInstruction {
    pub condition: Condition,
    pub operation: Operation,
    pub operands: Vec<Operand>
}

#[derive(Clone, Debug)]
pub enum Operand {
    Register(Register),
    Immediate(DoubleWord),
    AddressRegister(Register),
    AddressImmediate(DoubleWord),
    AddressLabel(String),
}

fn parse_label(tokens: &[Token]) -> Option<String> {
    if let [Token::Identifier(label), Token::Symbol(':')] = tokens {
        Some(label.clone())
    } else {
        None
    }
}

fn parse_instruction(mut tokens: &[Token]) -> Option<ProgramInstruction> {
    let condition = if let Some(Token::Condition(cond)) = tokens.get(0) {
        tokens = &tokens[1..];
        *cond
    } else {
        Condition::Always
    };

    let operation = if let Some(Token::Operation(operation)) = tokens.get(0) {
        tokens = &tokens[1..];
        Some(*operation)
    } else {
        None
    }?;

    let mut operands = Vec::new();

    while !tokens.is_empty() {
        let potential_operand: Option<(usize, Operand)> = {
            if let Token::Register(register) = tokens[0] {
                Some((1, Operand::Register(register)))
            } else if let Token::Number(imm) = tokens[0] {
                Some((1, Operand::Immediate(imm)))
            } else if tokens.len() >= 3 && tokens[0] == Token::Symbol('[') && tokens[2] == Token::Symbol(']') {
                match &tokens[1] {
                    Token::Register(register) => Some((3, Operand::AddressRegister(*register))),
                    Token::Number(imm)        => Some((3, Operand::AddressImmediate(*imm))),
                    Token::Identifier(label)  => Some((3, Operand::AddressLabel(label.clone()))),
                    _ => None
                }
            } else {
                None
            }
        };

        if let Some((consumed_tokens, operand)) = potential_operand {
            operands.push(operand);
            tokens = &tokens[consumed_tokens..];
        } else {
            return None;
        }
    }

    Some(ProgramInstruction { condition, operation, operands })
}

fn parse_statement(mut tokens: &[Token], labels: &HashMap<String, DoubleWord>) -> Option<Statement> {
    let condition = if let Some(Token::Condition(cond)) = tokens.get(0) {
        tokens = &tokens[1..];
        *cond
    } else {
        Condition::Always
    };
    
    match tokens {
        [] if condition == Condition::Always => Some(Statement::Empty),
        [Token::Identifier(identifier), Token::Symbol(':')] if condition == Condition::Always => Some(Statement::Label(identifier.to_string())),
        [Token::Operation(Operation::NOP)] => Some(Statement::Instruction(Instruction { 
            op: Operation::NOP, 
            cond: condition, 
            dest: Register::R0, 
            op1: Register::R0, 
            op2: Register::R0, 
            imm: 0
        })),
        [Token::Operation(Operation::LW), Token::Register(rd), Token::Symbol('['), Token::Register(ro1), Token::Symbol(']')] => Some(Statement::Instruction(Instruction { 
            op: Operation::LW, 
            cond: condition, 
            dest: *rd, 
            op1: *ro1, 
            op2: *ro1, 
            imm: 0
        })),
        [Token::Operation(Operation::LWI), Token::Register(rd), Token::Symbol('['), Token::Number(imm), Token::Symbol(']')] => Some(Statement::Instruction(Instruction { 
            op: Operation::LWI, 
            cond: condition, 
            dest: *rd, 
            op1: Register::R0, 
            op2: Register::R0, 
            imm: *imm
        })),
        [Token::Operation(Operation::SW), Token::Symbol('['), Token::Register(ro1), Token::Symbol(']'), Token::Register(ro2)] => Some(Statement::Instruction(Instruction { 
            op: Operation::SW, 
            cond: condition, 
            dest: Register::R0, 
            op1: *ro1, 
            op2: *ro2, 
            imm: 0
        })),
        [Token::Operation(Operation::SWI), Token::Symbol('['), Token::Number(imm), Token::Symbol(']'), Token::Register(ro2)] => Some(Statement::Instruction(Instruction { 
            op: Operation::SWI, 
            cond: condition, 
            dest: Register::R0, 
            op1: Register::R0, 
            op2: *ro2, 
            imm: *imm
        })),
        [Token::Operation(Operation::JP), Token::Symbol('['), Token::Register(ro1), Token::Symbol(']')] => Some(Statement::Instruction(Instruction { 
            op: Operation::JP, 
            cond: condition, 
            dest: Register::R0, 
            op1: *ro1, 
            op2: Register::R0, 
            imm: 0
        })),
        [Token::Operation(Operation::JPI), Token::Symbol('['), Token::Number(imm), Token::Symbol(']')] => Some(Statement::Instruction(Instruction {
            op: Operation::JPI,
            cond: condition,
            dest: Register::R0,
            op1: Register::R0,
            op2: Register::R0,
            imm: *imm
        })),
        [Token::Operation(Operation::JPI), Token::Symbol('['), Token::Identifier(label), Token::Symbol(']')] if labels.contains_key(label) => Some(Statement::Instruction(Instruction {
            op: Operation::JPI,
            cond: condition,
            dest: Register::R0,
            op1: Register::R0,
            op2: Register::R0,
            imm: labels[label]
        })),
        [Token::Operation(Operation::ALU(aluop)), Token::Register(rd), Token::Register(ro1), Token::Register(ro2)] => Some(Statement::Instruction(Instruction {
            op: Operation::ALU(*aluop),
            cond: condition,
            dest: *rd,
            op1: *ro1,
            op2: *ro2,
            imm: 0
        })),
        [Token::Operation(Operation::ALUI(aluop)), Token::Register(rd), Token::Register(ro1), Token::Number(imm)] => Some(Statement::Instruction(Instruction {
            op: Operation::ALU(*aluop),
            cond: condition,
            dest: *rd,
            op1: *ro1,
            op2: Register::R0,
            imm: *imm
        })),
        [Token::Operation(Operation::CMP(aluop)), Token::Register(ro1), Token::Register(ro2)] => Some(Statement::Instruction(Instruction {
            op: Operation::CMP(*aluop),
            cond: condition,
            dest: Register::R0,
            op1: *ro1,
            op2: *ro2,
            imm: 0
        })),
        [Token::Operation(Operation::CMPI(aluop)), Token::Register(ro1), Token::Number(imm)] => Some(Statement::Instruction(Instruction {
            op: Operation::CMPI(*aluop),
            cond: condition,
            dest: Register::R0,
            op1: *ro1,
            op2: Register::R0,
            imm: *imm
        })),
        [Token::Operation(Operation::WPR), Token::Register(ro1)] => Some(Statement::Instruction(Instruction {
            op: Operation::WPR,
            cond: condition,
            dest: Register::R0,
            op1: *ro1,
            op2: Register::R0,
            imm: 0
        })),
        [Token::Operation(Operation::WPRI), Token::Number(imm)] => Some(Statement::Instruction(Instruction {
            op: Operation::WPRI,
            cond: condition,
            dest: Register::R0,
            op1: Register::R0,
            op2: Register::R0,
            imm: *imm
        })),
        [Token::Operation(Operation::HCF)] => Some(Statement::Instruction(Instruction { 
            op: Operation::HCF, 
            cond: condition, 
            dest: Register::R0, 
            op1: Register::R0, 
            op2: Register::R0, 
            imm: 0
        })),
        [] | _ => None
    }
}

pub fn parse_program(mut tokens: Vec<Token>) -> Result<Program, usize> {
    let find_line_terminator: fn(&[Token]) -> Option<usize> = |toks| toks.iter().position(|t| matches!(t, Token::Symbol('\n')) 
                                                                                           || matches!(t, Token::EndOfInput));

    let mut program = Program::new();
    let mut lines_processed: usize = 0;

    while !tokens.is_empty() {
        // unwrap is safe here because we have at least the end terminator.
        let line_length = find_line_terminator(&tokens).unwrap();
        let line_tokens = &tokens[..line_length];

        if let Some(label) = parse_label(&line_tokens) {
            let label_target = program.instructions.len() as DoubleWord;
            let existing_label = program.labels.insert(label, label_target);
            if existing_label.is_some() {
                return Err(lines_processed);
            }
        } else if let Some(instruction) = parse_instruction(&line_tokens) {
            program.instructions.push(instruction);
        } else if let [] = &line_tokens {
            // skip empty lines.
        } else {
            return Err(lines_processed);
        }

        lines_processed += 1;
        tokens.drain(..=line_length);
    }
    Ok(program)
}

impl Program {
    pub fn new() -> Self {
        Program {
            labels: HashMap::new(),
            instructions: Vec::new()
        }
    }
}