use regex::Regex;
use isa::arch::{Opcode, Register};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token<'a> {
    Number(i64),
    Register(Register),
    Operation(Opcode),
    Identifier(&'a str),
    Symbol(char),
}

pub fn lex_number<'a>(input: &'a str) -> Option<(Token<'a>, usize)> {
    let regex = Regex::new(r"^(\-)?(0x|0b)?([0-9a-fA-F]+)").unwrap();
    let captures = regex.captures(&input)?;
    let radix = match captures.get(2) {
        Some(v) if v.as_str().starts_with("0x") => 16,
        Some(v) if v.as_str().starts_with("0b") => 2,
        _ => 10
    };
    let sign = captures.get(1).map_or(1, |_| -1);
    let value = u64::from_str_radix(&captures[3], radix).ok()?;

    Some((Token::Number(value as i64 * sign), captures[0].chars().count()))
}

pub fn lex_register<'a>(input: &'a str) -> Option<(Token<'a>, usize)> {
    if input.len() < 2 {
        None
    } else if input.len() > 2 && input.as_bytes()[2].is_ascii_alphanumeric() {
        None
    } else {
        match &input[0..2] {
            "R0" | "r0" | "RZ" | "rz" => Some((Token::Register(Register::RZ), 2)),
            "R1" | "r1"               => Some((Token::Register(Register::R1), 2)),
            "R2" | "r2"               => Some((Token::Register(Register::R2), 2)),
            "R3" | "r3"               => Some((Token::Register(Register::R3), 2)),
            "R4" | "r4"               => Some((Token::Register(Register::R4), 2)),
            "R5" | "r5" | "RF" | "rf" => Some((Token::Register(Register::RF), 2)),
            "R6" | "r6" | "RH" | "rh" => Some((Token::Register(Register::RH), 2)),
            "R7" | "r7" | "RL" | "rl" => Some((Token::Register(Register::RL), 2)),
            _ => None
        }
    }
}

pub fn lex_operation<'a>(input: &'a str) -> Option<(Token<'a>, usize)> {
    let mnemonic_length = input.find(|b: char| !b.is_alphanumeric()).unwrap_or(input.len());
    let mnemonic = &input[..mnemonic_length];
    match mnemonic {
        "ADC" | "adc" => Some((Token::Operation(Opcode::ADC), mnemonic_length)),
        "SBB" | "sbb" => Some((Token::Operation(Opcode::SBB), mnemonic_length)),
        "SHL" | "shl" => Some((Token::Operation(Opcode::SHL), mnemonic_length)),
        "SHR" | "shr" => Some((Token::Operation(Opcode::SHR), mnemonic_length)),
        "OR"  | "or"  => Some((Token::Operation(Opcode::OR),  mnemonic_length)),
        "AND" | "and" => Some((Token::Operation(Opcode::AND), mnemonic_length)),
        "XOR" | "xor" => Some((Token::Operation(Opcode::XOR), mnemonic_length)),
        "NAND"| "nand"=> Some((Token::Operation(Opcode::NAND),mnemonic_length)),
        "SW"  | "sw"  => Some((Token::Operation(Opcode::SW),  mnemonic_length)),
        "LW"  | "lw"  => Some((Token::Operation(Opcode::LW),  mnemonic_length)),
        "BZ"  | "bz"  => Some((Token::Operation(Opcode::BZ),  mnemonic_length)),
        "BNZ" | "bnz" => Some((Token::Operation(Opcode::BNZ), mnemonic_length)),
        //RES0
        //RES1
        "LA"  | "la"  => Some((Token::Operation(Opcode::LA),  mnemonic_length)),
        "HCF" | "hcf" => Some((Token::Operation(Opcode::HCF), mnemonic_length)),
        _ => None
    }
}

pub fn lex_identifier<'a>(input: &'a str) -> Option<(Token<'a>, usize)> {
    let token_length = input.find(|b: char| !(b.is_alphanumeric() || b == '_' || b == '-')).unwrap_or(input.len());
    let token = &input[..token_length];

    if token_length == 0 || !token.as_bytes()[0].is_ascii_alphabetic() {
        None
    } else {
        Some((Token::Identifier(token), token_length))
    }
}

pub fn lex_symbol<'a>(input: &'a str) -> Option<(Token<'a>, usize)> {
    Some((Token::Symbol(char::from(*input.as_bytes().get(0)?)), 1))
}


pub fn lex<'a>(mut input: &'a str) -> Result<Vec<Token<'a>>, String> {

    if !input.is_ascii() {
        return Err(String::from("Non ASCII character detected."));
    }

    let lexers = [
        lex_symbol,
        lex_identifier,
        lex_operation,
        lex_register,
        lex_number,
    ];

    let mut tokens = Vec::new();
    let mut parsed_total = 0;

    input = input.trim_start_matches(&[' ', '\t', '\r']);
    while !input.is_empty() {
        let longest_lexeme = lexers.iter()
            .filter_map(|lexer| lexer(input))
            .max_by(|a, b| a.1.cmp(&b.1));

        match longest_lexeme {
            Some((token, parsed_length)) => {
                parsed_total += parsed_length;
                input = &input[parsed_length..];
                input = input.trim_start_matches(&[' ', '\t', '\r']);
                tokens.push(token);
            },
            None => {
                let erroneous_input = &input[..(16.min(input.len()))];
                return Err(format!("Invalid token at position {parsed_total}: '{erroneous_input}...'"));
            }
        }
    }

    Ok(tokens)
}