use regex::Regex;

use crate::instructions::{Register, RegisterPair, Word, DoubleWord, Operation, Label, Condition};


pub fn try_parse_register(input: &str) -> Option<Register> {
    let mut chars = input.chars();
    let first_char = chars.next()?;
    let index = chars.next()?;
    
    if let Some(_) = chars.next() {
        return None;
    }

    if first_char != 'r' && first_char != 'R' {
        return None;
    }

    if index == 'P' || index == 'p' {
        return Some(Register::RP);
    }

    let register = Register::from_index(index.to_digit(8)? as u8)?;
    return Some(register);
}

pub fn try_parse_register_pair(input: &str) -> Option<RegisterPair> {
    let tokens: Vec<&str> = input.split(':').collect();
    if tokens.len() != 2 {
        return None;
    }

    let reg_first = try_parse_register(tokens.get(0)?)?;
    let reg_second = try_parse_register(tokens.get(1)?)?;

    Some((reg_first, reg_second))
}

pub fn try_parse_doubleword(mut input: &str) -> Option<DoubleWord> {
    let mut radix = 10;
    if input.starts_with("0x") {
        input = &input[2..];
        radix = 16;
    } else if input.starts_with("$") {
        input = &input[1..];
        radix = 16;
    } else if input.starts_with("0b") || input.starts_with("0B") {
        input = &input[2..];
        radix = 2;
    }

    if let Some(signed) = i16::from_str_radix(input, radix).ok() {
        return Some(unsafe { std::mem::transmute(signed) });
    }
    
    if let Some(unsigned) = u16::from_str_radix(input, radix).ok() {
        return Some(unsigned);
    }

    None
}

pub fn try_parse_operation(input: &str) -> Option<(Operation, Condition)> {
    
    let mut token = input.to_uppercase();

    let condition = if token.ends_with("CB") {
        token.pop()?;
        token.pop()?;
        Condition::CarryOrBorrow
    } else if token.ends_with("OF") {
        token.pop()?;
        token.pop()?;
        Condition::Overflow
    } else if token.ends_with("EQ") {
        token.pop()?;
        token.pop()?;
        Condition::Equal
    } else if token.ends_with("NE") {
        token.pop()?;
        token.pop()?;
        Condition::NotEqual
    } else if token.ends_with("NZ") {
        token.pop()?;
        token.pop()?;
        Condition::NotZero
    } else if token.ends_with("N") {
        token.pop()?;
        Condition::Negative
    } else if token.ends_with("Z") {
        token.pop()?;
        Condition::Zero
    } else {
        Condition::Always
    };

    let operation = match token.as_str() {
        "NOP" => Some(Operation::NOP),
        "LW" => Some(Operation::LW),
        "LWI" => Some(Operation::LWI),
        "SW" => Some(Operation::SW),
        "SWI" => Some(Operation::SWI),
        "MW" => Some(Operation::MW),
        "MWI" => Some(Operation::MWI),
        "JP" => Some(Operation::JP),
        "JPI" => Some(Operation::JPI),
        "ADD" => Some(Operation::ADD),
        "ADC" => Some(Operation::ADC),
        "SUB" => Some(Operation::SUB),
        "SBB" => Some(Operation::SBB),
        "OR" => Some(Operation::OR),
        "NOR" => Some(Operation::NOR),
        "XOR" => Some(Operation::XOR),
        "AND" => Some(Operation::AND),
        "ADDI" => Some(Operation::ADDI),
        "ADCI" => Some(Operation::ADCI),
        "SUBI" => Some(Operation::SUBI),
        "SBBI" => Some(Operation::SBBI),
        "ORI" => Some(Operation::ORI),
        "NORI" => Some(Operation::NORI),
        "XORI" => Some(Operation::XORI),
        "ANDI" => Some(Operation::ANDI),
        "ADDF" => Some(Operation::ADDF),
        "ADCF" => Some(Operation::ADCF),
        "SUBF" => Some(Operation::SUBF),
        "SBBF" => Some(Operation::SBBF),
        "ORF" => Some(Operation::ORF),
        "NORF" => Some(Operation::NORF),
        "XORF" => Some(Operation::XORF),
        "ANDF" => Some(Operation::ANDF),
        "ADDFI" => Some(Operation::ADDFI),
        "ADCFI" => Some(Operation::ADCFI),
        "SUBFI" => Some(Operation::SUBFI),
        "SBBFI" => Some(Operation::SBBFI),
        "ORFI" => Some(Operation::ORFI),
        "NORFI" => Some(Operation::NORFI),
        "XORFI" => Some(Operation::XORFI),
        "ANDFI" => Some(Operation::ANDFI),
        "CMP" => Some(Operation::CMP),
        "CMPI" => Some(Operation::CMPI),
        "HCF" => Some(Operation::HCF),
        _ => None
    };

    Some((operation?, condition))
}

pub fn try_parse_label(input: &str) -> Option<Label> {
    let pattern = Regex::new("^(?P<label>[a-zA-Z_][a-zA-Z0-9_]*):$").unwrap();
    let name = pattern.captures(input)?.name("label")?.as_str();
    Some(name.to_string())
}