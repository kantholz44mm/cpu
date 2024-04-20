use crate::{arch::{Condition, Instruction, Operation, Register}, lexer::Token};

pub fn parse_line(mut tokens: Vec<Token>) -> Option<Instruction> {

    let mut condition = Condition::Always;
    if tokens.len() > 1 {
        if let [_, Token::Conditional(cond)] = tokens[..2] {
            condition = cond;
            tokens.remove(1);
        };
    }

    match tokens.as_slice() {
        [Token::Operation(Operation::NOP)] => Some(Instruction { 
                op: Operation::NOP, 
                cond: condition, 
                dest: Register::R0, 
                op1: Register::R0, 
                op2: Register::R0, 
                imm: 0
            }),
        [Token::Operation(Operation::LW), Token::RegisterIdentifier(rd), Token::RegisterIdentifierPair(Register::R3, ro1)] => Some(Instruction { 
                op: Operation::LW, 
                cond: condition, 
                dest: *rd, 
                op1: *ro1, 
                op2: *ro1, 
                imm: 0
            }),
        [Token::Operation(Operation::LWI), Token::RegisterIdentifier(rd), Token::ImmediateAddress(imm)] => Some(Instruction { 
                op: Operation::LWI, 
                cond: condition, 
                dest: *rd, 
                op1: Register::R0, 
                op2: Register::R0, 
                imm: *imm
            }),
        [Token::Operation(Operation::SW), Token::RegisterIdentifierPair(Register::R3, ro1), Token::RegisterIdentifier(ro2)] => Some(Instruction { 
                op: Operation::SW, 
                cond: condition, 
                dest: Register::R0, 
                op1: *ro1, 
                op2: *ro2, 
                imm: 0
            }),
        [Token::Operation(Operation::SWI), Token::ImmediateAddress(imm), Token::RegisterIdentifier(ro2)] => Some(Instruction { 
                op: Operation::SWI, 
                cond: condition, 
                dest: Register::R0, 
                op1: Register::R0, 
                op2: *ro2, 
                imm: *imm
            }),
        [Token::Operation(Operation::JP), Token::RegisterIdentifierPair(Register::R3, ro1)] => Some(Instruction { 
                op: Operation::JP, 
                cond: condition, 
                dest: Register::R0, 
                op1: *ro1, 
                op2: Register::R0, 
                imm: 0
            }),
        [Token::Operation(Operation::JPI), Token::ImmediateAddress(imm)] => Some(Instruction {
                op: Operation::JPI,
                cond: condition,
                dest: Register::R0,
                op1: Register::R0,
                op2: Register::R0,
                imm: *imm
            }),
        [Token::Operation(Operation::ALU(aluop)), Token::RegisterIdentifier(rd), Token::RegisterIdentifier(ro1), Token::RegisterIdentifier(ro2)] => Some(Instruction {
                op: Operation::ALU(*aluop),
                cond: condition,
                dest: *rd,
                op1: *ro1,
                op2: *ro2,
                imm: 0
            }),
        [Token::Operation(Operation::ALUI(aluop)), Token::RegisterIdentifier(rd), Token::RegisterIdentifier(ro1), Token::IntegerLiteral(imm)] => Some(Instruction {
                op: Operation::ALU(*aluop),
                cond: condition,
                dest: *rd,
                op1: *ro1,
                op2: Register::R0,
                imm: *imm
            }),
        [Token::Operation(Operation::CMP(aluop)), Token::RegisterIdentifier(rd), Token::RegisterIdentifier(ro1), Token::RegisterIdentifier(ro2)] => Some(Instruction {
                op: Operation::CMP(*aluop),
                cond: condition,
                dest: *rd,
                op1: *ro1,
                op2: *ro2,
                imm: 0
            }),
        [Token::Operation(Operation::CMPI(aluop)), Token::RegisterIdentifier(rd), Token::RegisterIdentifier(ro1), Token::IntegerLiteral(imm)] => Some(Instruction {
                op: Operation::CMPI(*aluop),
                cond: condition,
                dest: *rd,
                op1: *ro1,
                op2: Register::R0,
                imm: *imm
            }),
        [Token::Operation(Operation::HCF)] => Some(Instruction { 
                op: Operation::HCF, 
                cond: condition, 
                dest: Register::R0, 
                op1: Register::R0, 
                op2: Register::R0, 
                imm: 0
            }),
        _ => None
    }
}