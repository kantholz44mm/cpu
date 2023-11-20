use std::mem::transmute;

use regex::Regex;

#[derive(Debug)]
pub enum Opcode {
    NOP   = 0x0,
    LW    = 0x1,
    LWI   = 0x2,
    SW    = 0x3,
    SWI   = 0x4,
    MW    = 0x5,
    MWI   = 0x6,
    JP    = 0x7,
    JPI   = 0x8,
    ALU   = 0x9,
    ALUI  = 0xA,
    ALUF  = 0xB,
    ALUFI = 0xC,
    CMP   = 0xD,
}

#[derive(Debug)]
pub enum Operation {
    NOP,
    LW,
    LWI,
    SW,
    SWI,
    MW,
    MWI,
    JP,
    JPI,

    // ALU operations. these all run under one of 4 opcodes, depending on the combination of F and I.
    ADD,
    ADDI,
    ADDF,
    ADDFI,
    ADC,
    ADCI,
    ADCF,
    ADCFI,
    SUB,
    SUBI,
    SUBF,
    SUBFI,
    SBB,
    SBBI,
    SBBF,
    SBBFI,
    OR,
    ORI,
    ORF,
    ORFI,
    NOR,
    NORI,
    NORF,
    NORFI,
    XOR,
    XORI,
    XORF,
    XORFI,
    AND,
    ANDI,
    ANDF,
    ANDFI,

    CMP
}

pub enum ALUOpcode {
    ADD = 0b000,
    ADC = 0b001,
    SUB = 0b010,
    SBB = 0b011,
    OR  = 0b100,
    NOR = 0b101,
    XOR = 0b110,
    AND = 0b111,
}

#[derive(Debug)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    RP = 7,
}

#[derive(Debug)]
pub enum Token {
    Imm8(u8),
    Imm16(u8, u8),
    RegisterName(Register),
    RegisterPair{ low: Register, high: Register },
    Operation(Operation),
    Label(String),
}

impl Token {

    fn parse_immediate(input: &str) -> Option<Self> {
        let match_imm8 = Token::Imm8(0).get_pattern().unwrap();
        let captures = match_imm8.captures(input)?;

        let (radix, literal) = if let Some(hex) = captures.name("hex") {
            (16, hex.as_str())
        } else if let Some(bin) = captures.name("bin") {
            (2, bin.as_str())
        } else {
            let dec = captures.name("dec")?;
            (10, dec.as_str())
        };

        let signed_value: i64 = i64::from_str_radix(literal, radix).ok()?;

        if let Some(val) = i8::try_from(signed_value).ok() {
            unsafe { Some(Self::Imm8(transmute(val))) }
        } else if let Some(val) = u8::try_from(signed_value).ok() {
            Some(Self::Imm8(val))
        } else if let Some(val) = i16::try_from(signed_value).ok() {
            let bytes = val.to_le_bytes();
            Some(Self::Imm16(bytes[0], bytes[1]))
        } else if let Some(val) = u16::try_from(signed_value).ok() {
            let bytes = val.to_le_bytes();
            Some(Self::Imm16(bytes[0], bytes[1]))
        } else {
            println!("integer literal '{}' does not fit into 16 bits.", signed_value);
            None
        }
    }

    fn parse_register(input: &str) -> Option<Self> {
        let match_reg = Token::RegisterName(Register::R0).get_pattern().unwrap();
        let captures = match_reg.captures(input)?;
        let digit = captures.name("digit").unwrap()
                            .as_str().chars().nth(0).unwrap();

        
        match digit {
            '0'             => Some(Self::RegisterName(Register::R0)),
            '1'             => Some(Self::RegisterName(Register::R1)),
            '2'             => Some(Self::RegisterName(Register::R2)),
            '3'             => Some(Self::RegisterName(Register::R3)),
            '4'             => Some(Self::RegisterName(Register::R4)),
            '5'             => Some(Self::RegisterName(Register::R5)),
            '6'             => Some(Self::RegisterName(Register::R6)),
            '7' | 'p' | 'P' => Some(Self::RegisterName(Register::RP)),
            _               => None
        }
    }

    fn parse_register_pair(input: &str) -> Option<Self> {
        let tokens: Vec<&str> = input.split(':').collect();
        if let (Self::RegisterName(first), Self::RegisterName(second))
        = (Self::parse_register(tokens.get(0)?)?, Self::parse_register(tokens.get(1)?)?) {
            Some(Self::RegisterPair { low: first, high: second })
        } else {
            None
        }
    }

    fn parse_operation_mnemonic(input: &str) -> Option<Self> {

        Some(Self::Operation(Operation::from_mnemonic(input)?))
    }

    fn parse_label(input: &str) -> Option<Self> {
        let pattern = Self::get_pattern(&Token::Label(String::new())).unwrap();
        Some(Self::Label(pattern.captures(input)?.name("name")?.as_str().to_string()))
    }

    pub fn try_parse(input: &str) -> Option<Self> {

        let tokens = input.split([' ', '\t', '\n', '\r', ',']);
        for token in tokens {
            println!("TODO parse here");
            todo!();
        }

        None
    }

    pub fn get_pattern(&self) -> Option<Regex> {
        match self {
            Token::Imm8(_) | Token::Imm16(_,_)  => Some(Regex::new(r"^(?:\$(?P<hex>[0-9a-zA-Z_]+))|(?:[bB](?P<bin>[01_]+))|(?P<dec>[\-+]?[0-9]+)(?:$|,|\s)").unwrap()),
            Token::RegisterName(_)              => Some(Regex::new(r"^r(?P<digit>[0-7p])(?:$|,|\s)").unwrap()),
            Token::Label(_)                     => Some(Regex::new(r"^(?P<name>[a-zA-Z_][a-zA-Z0-9_]*):(?:$|,|\s)").unwrap()),
            _                                   => None,
        }
    }
}

impl Operation {
    pub fn mnemonic(self) -> &'static str {
        match self {
            Operation::NOP => "NOP",
            Operation::LW => "LW",
            Operation::LWI => "LWI",
            Operation::SW => "SW",
            Operation::SWI => "SWI",
            Operation::MW => "MW",
            Operation::MWI => "MWI",
            Operation::JP => "JP",
            Operation::JPI => "JPI",
            Operation::ADD => "ADD",
            Operation::ADDI => "ADDI",
            Operation::ADDF => "ADDF",
            Operation::ADDFI => "ADDFI",
            Operation::ADC => "ADC",
            Operation::ADCI => "ADCI",
            Operation::ADCF => "ADCF",
            Operation::ADCFI => "ADCFI",
            Operation::SUB => "SUB",
            Operation::SUBI => "SUBI",
            Operation::SUBF => "SUBF",
            Operation::SUBFI => "SUBFI",
            Operation::SBB => "SBB",
            Operation::SBBI => "SBBI",
            Operation::SBBF => "SBBF",
            Operation::SBBFI => "SBBFI",
            Operation::OR => "OR",
            Operation::ORI => "ORI",
            Operation::ORF => "ORF",
            Operation::ORFI => "ORFI",
            Operation::NOR => "NOR",
            Operation::NORI => "NORI",
            Operation::NORF => "NORF",
            Operation::NORFI => "NORFI",
            Operation::XOR => "XOR",
            Operation::XORI => "XORI",
            Operation::XORF => "XORF",
            Operation::XORFI => "XORFI",
            Operation::AND => "AND",
            Operation::ANDI => "ANDI",
            Operation::ANDF => "ANDF",
            Operation::ANDFI => "ANDFI",
            Operation::CMP => "CMP"
        }
    }

    pub fn from_mnemonic(input: &str) -> Option<Self> {
        match input {
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
            "ADDI" => Some(Operation::ADDI),
            "ADDF" => Some(Operation::ADDF),
            "ADDFI" => Some(Operation::ADDFI),
            "ADC" => Some(Operation::ADC),
            "ADCI" => Some(Operation::ADCI),
            "ADCF" => Some(Operation::ADCF),
            "ADCFI" => Some(Operation::ADCFI),
            "SUB" => Some(Operation::SUB),
            "SUBI" => Some(Operation::SUBI),
            "SUBF" => Some(Operation::SUBF),
            "SUBFI" => Some(Operation::SUBFI),
            "SBB" => Some(Operation::SBB),
            "SBBI" => Some(Operation::SBBI),
            "SBBF" => Some(Operation::SBBF),
            "SBBFI" => Some(Operation::SBBFI),
            "OR" => Some(Operation::OR),
            "ORI" => Some(Operation::ORI),
            "ORF" => Some(Operation::ORF),
            "ORFI" => Some(Operation::ORFI),
            "NOR" => Some(Operation::NOR),
            "NORI" => Some(Operation::NORI),
            "NORF" => Some(Operation::NORF),
            "NORFI" => Some(Operation::NORFI),
            "XOR" => Some(Operation::XOR),
            "XORI" => Some(Operation::XORI),
            "XORF" => Some(Operation::XORF),
            "XORFI" => Some(Operation::XORFI),
            "AND" => Some(Operation::AND),
            "ANDI" => Some(Operation::ANDI),
            "ANDF" => Some(Operation::ANDF),
            "ANDFI" => Some(Operation::ANDFI),
            "CMP" => Some(Operation::CMP),
            _ => None
        }
    }
}