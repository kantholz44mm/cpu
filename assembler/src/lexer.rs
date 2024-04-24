use regex::Regex;
use crate::arch::{ALUFunction, Condition, DoubleWord, Operation, Register};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Whitespace,
    Number(DoubleWord),
    Register(Register),
    Operation(Operation),
    Condition(Condition),
    Symbol(char),
    Identifier(String),
    Comment(String),
    EndOfInput,
}

pub type Lexer = fn(&str) -> Option<(Token, usize)>;

fn fold_longest_token(longest: Option<(Token, usize)>, current: (Token, usize)) -> Option<(Token, usize)> {
    if let Some((_, longest_len)) = longest {
        if current.1 > longest_len {
            Some(current)
        } else {
            longest
        }
    } else {
        Some(current)
    }
}

pub fn lex_program(mut input: &str) -> Result<Vec<Token>, usize> {
    const LEXERS: [Lexer; 8] = [
        lexer_whitespace,
        lexer_number,
        lexer_register,
        lexer_operation,
        lexer_symbol,
        lexer_condition,
        lexer_identifier,
        lexer_comment
    ];

    let mut tokens = Vec::new();
    let mut consumed = 0;

    while let Some((token, length)) = LEXERS.iter().filter_map(|lexer| lexer(input)).fold(None, fold_longest_token) {
        input = &input[length..];
        consumed += length;
        tokens.push(token);
    }

    if !input.is_empty() {
        return Err(consumed);
    }

    tokens.retain(|token| {
        match token {
            Token::Comment(_) |
            Token::Whitespace => false,
            _ => true
        }
    });
    tokens.push(Token::EndOfInput);
    Ok(tokens)
}

pub fn lexer_comment(input: &str) -> Option<(Token, usize)> {
    if input.starts_with(";") {
        let comment_length = input.chars().position(|c| c == '\n').unwrap_or(input.chars().count());
        Some((Token::Comment(input[..comment_length].to_string()), comment_length))
    } else {
        None
    }
}

pub fn lexer_whitespace(input: &str) -> Option<(Token, usize)> {
    match input.chars().take_while(|c| "\t ,".contains(*c)).count() {
        0 => None,
        len => Some((Token::Whitespace, len)),
    }
}

pub fn lexer_number(input: &str) -> Option<(Token, usize)> {
    let regex = Regex::new(r"^(\-)?(0x|0b)?([0-9a-zA-Z]+)").unwrap();
    let captures = regex.captures(&input)?;
    let radix = match captures.get(2) {
        Some(v) if v.as_str().starts_with("0x") => 16,
        Some(v) if v.as_str().starts_with("0b") => 2,
        _ => 10
    };
    let sign = captures.get(1).map_or(1, |_| -1);
    let value = i32::from_str_radix(&captures[3], radix).ok()?;

    Some((Token::Number((value * sign) as DoubleWord), captures[0].chars().count()))
}

pub fn lexer_register(input: &str) -> Option<(Token, usize)> {
    match input.to_uppercase().as_str() {
        s if s.starts_with("R0") => Some((Token::Register(Register::R0), 2)),
        s if s.starts_with("R1") => Some((Token::Register(Register::R1), 2)),
        s if s.starts_with("R2") => Some((Token::Register(Register::R2), 2)),
        s if s.starts_with("R3") => Some((Token::Register(Register::R3), 2)),
        _ => None
    }
}

pub fn lexer_symbol(input: &str) -> Option<(Token, usize)> {
    let character = input.chars().nth(0)?;
    const VALID_SYMBOLS: &str = ";:\n[]";

    if VALID_SYMBOLS.contains(character) {
        Some((Token::Symbol(character), 1))
    } else {
        None
    }
}

pub fn lexer_condition(input: &str) -> Option<(Token, usize)> {
    match input.to_uppercase().as_str() {
        s if s.starts_with("CB>") => Some((Token::Condition(Condition::CarryOrBorrow), 3)),
        s if s.starts_with("EQ>") => Some((Token::Condition(Condition::Equality), 3)),
        s if s.starts_with("Z>")  => Some((Token::Condition(Condition::Zero), 2)),
        _ => None
    }
}

pub fn lexer_operation(input: &str) -> Option<(Token, usize)> {
    match input.to_uppercase().as_str() {
        s if s.starts_with("NOP")   => Some((Token::Operation(Operation::NOP), 3)),
        s if s.starts_with("LWI")   => Some((Token::Operation(Operation::LWI), 3)),
        s if s.starts_with("LW")    => Some((Token::Operation(Operation::LW),  2)),
        s if s.starts_with("SWI")   => Some((Token::Operation(Operation::SWI), 3)),
        s if s.starts_with("SW")    => Some((Token::Operation(Operation::SW),  2)),
        s if s.starts_with("JPI")   => Some((Token::Operation(Operation::JPI), 3)),
        s if s.starts_with("JP")    => Some((Token::Operation(Operation::JP),  2)),

        s if s.starts_with("ADDFI")  => Some((Token::Operation(Operation::ALUFI(ALUFunction::ADD)), 5)),
        s if s.starts_with("ADCFI")  => Some((Token::Operation(Operation::ALUFI(ALUFunction::ADC)), 5)),
        s if s.starts_with("SUBFI")  => Some((Token::Operation(Operation::ALUFI(ALUFunction::SUB)), 5)),
        s if s.starts_with("SBBFI")  => Some((Token::Operation(Operation::ALUFI(ALUFunction::SBB)), 5)),
        s if s.starts_with("ORFI")   => Some((Token::Operation(Operation::ALUFI(ALUFunction::OR)),  4)),
        s if s.starts_with("NORFI")  => Some((Token::Operation(Operation::ALUFI(ALUFunction::NOR)), 5)),
        s if s.starts_with("XORFI")  => Some((Token::Operation(Operation::ALUFI(ALUFunction::XOR)), 5)),
        s if s.starts_with("ANDFI")  => Some((Token::Operation(Operation::ALUFI(ALUFunction::AND)), 5)),
        s if s.starts_with("SHLFI")  => Some((Token::Operation(Operation::ALUFI(ALUFunction::SHL)), 5)),
        s if s.starts_with("SHRFI")  => Some((Token::Operation(Operation::ALUFI(ALUFunction::SHR)), 5)),

        s if s.starts_with("ADDF")  => Some((Token::Operation(Operation::ALUF(ALUFunction::ADD)), 4)),
        s if s.starts_with("ADCF")  => Some((Token::Operation(Operation::ALUF(ALUFunction::ADC)), 4)),
        s if s.starts_with("SUBF")  => Some((Token::Operation(Operation::ALUF(ALUFunction::SUB)), 4)),
        s if s.starts_with("SBBF")  => Some((Token::Operation(Operation::ALUF(ALUFunction::SBB)), 4)),
        s if s.starts_with("ORF")   => Some((Token::Operation(Operation::ALUF(ALUFunction::OR)),  3)),
        s if s.starts_with("NORF")  => Some((Token::Operation(Operation::ALUF(ALUFunction::NOR)), 4)),
        s if s.starts_with("XORF")  => Some((Token::Operation(Operation::ALUF(ALUFunction::XOR)), 4)),
        s if s.starts_with("ANDF")  => Some((Token::Operation(Operation::ALUF(ALUFunction::AND)), 4)),
        s if s.starts_with("SHLF")  => Some((Token::Operation(Operation::ALUF(ALUFunction::SHL)), 4)),
        s if s.starts_with("SHRF")  => Some((Token::Operation(Operation::ALUF(ALUFunction::SHR)), 4)),

        s if s.starts_with("ADDI")   => Some((Token::Operation(Operation::ALUI(ALUFunction::ADD)), 4)),
        s if s.starts_with("ADCI")   => Some((Token::Operation(Operation::ALUI(ALUFunction::ADC)), 4)),
        s if s.starts_with("SUBI")   => Some((Token::Operation(Operation::ALUI(ALUFunction::SUB)), 4)),
        s if s.starts_with("SBBI")   => Some((Token::Operation(Operation::ALUI(ALUFunction::SBB)), 4)),
        s if s.starts_with("ORI")    => Some((Token::Operation(Operation::ALUI(ALUFunction::OR)),  3)),
        s if s.starts_with("NORI")   => Some((Token::Operation(Operation::ALUI(ALUFunction::NOR)), 4)),
        s if s.starts_with("XORI")   => Some((Token::Operation(Operation::ALUI(ALUFunction::XOR)), 4)),
        s if s.starts_with("ANDI")   => Some((Token::Operation(Operation::ALUI(ALUFunction::AND)), 4)),
        s if s.starts_with("SHLI")   => Some((Token::Operation(Operation::ALUI(ALUFunction::SHL)), 4)),
        s if s.starts_with("SHRI")   => Some((Token::Operation(Operation::ALUI(ALUFunction::SHR)), 4)),

        s if s.starts_with("ADD")   => Some((Token::Operation(Operation::ALU(ALUFunction::ADD)), 3)),
        s if s.starts_with("ADC")   => Some((Token::Operation(Operation::ALU(ALUFunction::ADC)), 3)),
        s if s.starts_with("SUB")   => Some((Token::Operation(Operation::ALU(ALUFunction::SUB)), 3)),
        s if s.starts_with("SBB")   => Some((Token::Operation(Operation::ALU(ALUFunction::SBB)), 3)),
        s if s.starts_with("OR")    => Some((Token::Operation(Operation::ALU(ALUFunction::OR)),  2)),
        s if s.starts_with("NOR")   => Some((Token::Operation(Operation::ALU(ALUFunction::NOR)), 3)),
        s if s.starts_with("XOR")   => Some((Token::Operation(Operation::ALU(ALUFunction::XOR)), 3)),
        s if s.starts_with("AND")   => Some((Token::Operation(Operation::ALU(ALUFunction::AND)), 3)),
        s if s.starts_with("SHL")   => Some((Token::Operation(Operation::ALU(ALUFunction::SHL)), 3)),
        s if s.starts_with("SHR")   => Some((Token::Operation(Operation::ALU(ALUFunction::SHR)), 3)),

        s if s.starts_with("CADDI")   => Some((Token::Operation(Operation::CMPI(ALUFunction::ADD)), 5)),
        s if s.starts_with("CADCI")   => Some((Token::Operation(Operation::CMPI(ALUFunction::ADC)), 5)),
        s if s.starts_with("CSUBI")   => Some((Token::Operation(Operation::CMPI(ALUFunction::SUB)), 5)),
        s if s.starts_with("CSBBI")   => Some((Token::Operation(Operation::CMPI(ALUFunction::SBB)), 5)),
        s if s.starts_with("CORI")    => Some((Token::Operation(Operation::CMPI(ALUFunction::OR)),  4)),
        s if s.starts_with("CNORI")   => Some((Token::Operation(Operation::CMPI(ALUFunction::NOR)), 5)),
        s if s.starts_with("CXORI")   => Some((Token::Operation(Operation::CMPI(ALUFunction::XOR)), 5)),
        s if s.starts_with("CANDI")   => Some((Token::Operation(Operation::CMPI(ALUFunction::AND)), 5)),
        s if s.starts_with("CSHLI")   => Some((Token::Operation(Operation::CMPI(ALUFunction::SHL)), 5)),
        s if s.starts_with("CSHRI")   => Some((Token::Operation(Operation::CMPI(ALUFunction::SHR)), 5)),

        s if s.starts_with("CADD")   => Some((Token::Operation(Operation::CMP(ALUFunction::ADD)), 4)),
        s if s.starts_with("CADC")   => Some((Token::Operation(Operation::CMP(ALUFunction::ADC)), 4)),
        s if s.starts_with("CSUB")   => Some((Token::Operation(Operation::CMP(ALUFunction::SUB)), 4)),
        s if s.starts_with("CSBB")   => Some((Token::Operation(Operation::CMP(ALUFunction::SBB)), 4)),
        s if s.starts_with("COR")    => Some((Token::Operation(Operation::CMP(ALUFunction::OR)),  3)),
        s if s.starts_with("CNOR")   => Some((Token::Operation(Operation::CMP(ALUFunction::NOR)), 4)),
        s if s.starts_with("CXOR")   => Some((Token::Operation(Operation::CMP(ALUFunction::XOR)), 4)),
        s if s.starts_with("CAND")   => Some((Token::Operation(Operation::CMP(ALUFunction::AND)), 4)),
        s if s.starts_with("CSHL")   => Some((Token::Operation(Operation::CMP(ALUFunction::SHL)), 4)),
        s if s.starts_with("CSHR")   => Some((Token::Operation(Operation::CMP(ALUFunction::SHR)), 4)),

        s if s.starts_with("WPRI")  => Some((Token::Operation(Operation::WPRI), 4)),
        s if s.starts_with("WPR")   => Some((Token::Operation(Operation::WPR),  3)),
        s if s.starts_with("HCF")   => Some((Token::Operation(Operation::HCF),  3)),
        _ => None
    }
}

pub fn lexer_identifier(input: &str) -> Option<(Token, usize)> {
    let identifier: String = input.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
    if !identifier.is_empty() && identifier.chars().nth(0).unwrap().is_alphabetic() {
        let length = identifier.chars().count();
        Some((Token::Identifier(identifier), length))
    } else {
        None
    }
}