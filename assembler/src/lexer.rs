use num_traits::FromPrimitive;
use regex::Regex;

use crate::arch::{ALUFunction, Condition, DoubleWord, Operation, Register};

pub type Lexer = fn(&str) -> Option<Token>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    IntegerLiteral(DoubleWord),
    RegisterIdentifier(Register),
    RegisterIdentifierPair(Register, Register),
    ImmediateAddress(DoubleWord),
    Operation(Operation),
    Conditional(Condition),
}

pub fn lexer(input: &str) -> Option<Token> {
    [
        lexer_integer(input),
        lexer_operation(input),
        lexer_register(input),
        lexer_registerpair(input),
        lexer_immediate_address(input),
        lexer_conditional(input)
    ].iter().filter_map(|&token| token).nth(0)
}

pub fn lexer_integer(input: &str) -> Option<Token> {
    let regex = Regex::new(r"^(?<sign>\+|\-)?(?<radix>0x|0b)?(?<value>[0-9a-fA-F]+)").unwrap();
    let captures = regex.captures(&input)?;
    let sign = captures.name("sign").map_or(1, |m| if m.as_str() == "-" {-1} else {1});
    let radix = captures.name("radix").map_or(10, |m| if m.as_str() == "0x" {16} else {2});
    let value_literal = captures.name("value")?.as_str();
    let value = i32::from_str_radix(value_literal, radix).ok()?;

    Some(Token::IntegerLiteral((value * sign) as DoubleWord))
}

pub fn lexer_register(input: &str) -> Option<Token> {
    match input.to_uppercase().as_str() {
        "R0" => Some(Token::RegisterIdentifier(Register::R0)),
        "R1" => Some(Token::RegisterIdentifier(Register::R1)),
        "R2" => Some(Token::RegisterIdentifier(Register::R2)),
        "R3" | "RP" => Some(Token::RegisterIdentifier(Register::R3)),
        _ => None
    }
}

pub fn lexer_immediate_address(input: &str) -> Option<Token> {
    if let Some(Token::IntegerLiteral(imm)) = lexer_integer(input.strip_suffix(']')?.strip_prefix('[')?) {
        Some(Token::ImmediateAddress(imm))
    } else {
        None
    }
}

pub fn lexer_conditional(input: &str) -> Option<Token> {
    match input.to_uppercase().as_str() {
        "(CB)" => Some(Token::Conditional(Condition::CarryOrBorrow)),
        "(EQ)" => Some(Token::Conditional(Condition::Equality)),
        "(Z)"  => Some(Token::Conditional(Condition::Zero)),
        _ => None
    }
}

pub fn lexer_registerpair(input: &str) -> Option<Token> {
    let input = input.strip_suffix(']')?.strip_prefix('[')?;
    let parts = input.split(':').collect::<Vec<&str>>();
    
    if let [a, b] = parts.as_slice() {
        if let (Some(Token::RegisterIdentifier(reg_a)), Some(Token::RegisterIdentifier(reg_b))) = (lexer_register(a), lexer_register(b)) {
            return Some(Token::RegisterIdentifierPair(reg_a, reg_b));
        }
    }
    
    None
}

pub fn lexer_operation(input: &str) -> Option<Token> {
    match input.to_uppercase().as_str() {
        "NOP"   => Some(Token::Operation(Operation::NOP)),
        "LW"    => Some(Token::Operation(Operation::LW)),
        "LWI"   => Some(Token::Operation(Operation::LWI)),
        "SW"    => Some(Token::Operation(Operation::SW)),
        "SWI"   => Some(Token::Operation(Operation::SWI)),
        "JP"    => Some(Token::Operation(Operation::JP)),
        "JPI"   => Some(Token::Operation(Operation::JPI)),

        "ADD"   => Some(Token::Operation(Operation::ALU(ALUFunction::ADD))),
        "ADC"   => Some(Token::Operation(Operation::ALU(ALUFunction::ADC))),
        "SUB"   => Some(Token::Operation(Operation::ALU(ALUFunction::SUB))),
        "SBB"   => Some(Token::Operation(Operation::ALU(ALUFunction::SBB))),
        "OR"    => Some(Token::Operation(Operation::ALU(ALUFunction::OR))),
        "NOR"   => Some(Token::Operation(Operation::ALU(ALUFunction::NOR))),
        "XOR"   => Some(Token::Operation(Operation::ALU(ALUFunction::XOR))),
        "AND"   => Some(Token::Operation(Operation::ALU(ALUFunction::AND))),
        "SHL"   => Some(Token::Operation(Operation::ALU(ALUFunction::SHL))),
        "SHR"   => Some(Token::Operation(Operation::ALU(ALUFunction::SHR))),

        "ADDI"   => Some(Token::Operation(Operation::ALUI(ALUFunction::ADD))),
        "ADCI"   => Some(Token::Operation(Operation::ALUI(ALUFunction::ADC))),
        "SUBI"   => Some(Token::Operation(Operation::ALUI(ALUFunction::SUB))),
        "SBBI"   => Some(Token::Operation(Operation::ALUI(ALUFunction::SBB))),
        "ORI"    => Some(Token::Operation(Operation::ALUI(ALUFunction::OR))),
        "NORI"   => Some(Token::Operation(Operation::ALUI(ALUFunction::NOR))),
        "XORI"   => Some(Token::Operation(Operation::ALUI(ALUFunction::XOR))),
        "ANDI"   => Some(Token::Operation(Operation::ALUI(ALUFunction::AND))),
        "SHLI"   => Some(Token::Operation(Operation::ALUI(ALUFunction::SHL))),
        "SHRI"   => Some(Token::Operation(Operation::ALUI(ALUFunction::SHR))),

        "CADD"   => Some(Token::Operation(Operation::CMP(ALUFunction::ADD))),
        "CADC"   => Some(Token::Operation(Operation::CMP(ALUFunction::ADC))),
        "CSUB"   => Some(Token::Operation(Operation::CMP(ALUFunction::SUB))),
        "CSBB"   => Some(Token::Operation(Operation::CMP(ALUFunction::SBB))),
        "COR"    => Some(Token::Operation(Operation::CMP(ALUFunction::OR))),
        "CNOR"   => Some(Token::Operation(Operation::CMP(ALUFunction::NOR))),
        "CXOR"   => Some(Token::Operation(Operation::CMP(ALUFunction::XOR))),
        "CAND"   => Some(Token::Operation(Operation::CMP(ALUFunction::AND))),
        "CSHL"   => Some(Token::Operation(Operation::CMP(ALUFunction::SHL))),
        "CSHR"   => Some(Token::Operation(Operation::CMP(ALUFunction::SHR))),

        "CADDI"   => Some(Token::Operation(Operation::CMPI(ALUFunction::ADD))),
        "CADCI"   => Some(Token::Operation(Operation::CMPI(ALUFunction::ADC))),
        "CSUBI"   => Some(Token::Operation(Operation::CMPI(ALUFunction::SUB))),
        "CSBBI"   => Some(Token::Operation(Operation::CMPI(ALUFunction::SBB))),
        "CORI"    => Some(Token::Operation(Operation::CMPI(ALUFunction::OR))),
        "CNORI"   => Some(Token::Operation(Operation::CMPI(ALUFunction::NOR))),
        "CXORI"   => Some(Token::Operation(Operation::CMPI(ALUFunction::XOR))),
        "CANDI"   => Some(Token::Operation(Operation::CMPI(ALUFunction::AND))),
        "CSHLI"   => Some(Token::Operation(Operation::CMPI(ALUFunction::SHL))),
        "CSHRI"   => Some(Token::Operation(Operation::CMPI(ALUFunction::SHR))),

        "HCF"   => Some(Token::Operation(Operation::HCF)),
        _ => None
    }
}