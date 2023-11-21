use crate::lexer;


pub type Word = u8;
pub type DoubleWord = u16;
pub type QuadWord = u32;

#[derive(Debug, Clone, Copy)]
pub enum ALUFunction {
    ADD = 0,
    ADC = 1,
    SUB = 2,
    SBB = 3,
    OR  = 4,
    NOR = 5,
    XOR = 6,
    AND = 7,
}

#[derive(Debug, Clone, Copy)]
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

pub type RegisterPair = (Register, Register);
pub type Label = String;

#[derive(Debug, Clone, Copy)]
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

    ADD,
    ADC,
    SUB,
    SBB,
    OR,
    NOR,
    XOR,
    AND,

    ADDI,
    ADCI,
    SUBI,
    SBBI,
    ORI,
    NORI,
    XORI,
    ANDI,

    ADDF,
    ADCF,
    SUBF,
    SBBF,
    ORF,
    NORF,
    XORF,
    ANDF,

    ADDFI,
    ADCFI,
    SUBFI,
    SBBFI,
    ORFI,
    NORFI,
    XORFI,
    ANDFI,

    CMP,
    CMPI,
}

#[derive(Debug, Clone)]
pub enum Token {
    Imm8(Word),
    Imm16(DoubleWord),
    Register(Register),
    RegisterPair(RegisterPair),
    Operation(Operation),
    Label(Label)
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    NOP,
    LW      { dest: Register, addr: RegisterPair },
    LWI     { dest: Register, addr: DoubleWord },
    SW      { addr: RegisterPair, source: Register },
    SWI     { addr: DoubleWord, source: Register },
    MW      { dest: Register, source: Register },
    MWI     { dest: Register, datum: Word },
    JP      { addr: RegisterPair },
    JPI     { addr: DoubleWord },
    ALU     { dest: Register, op_a: Register, op_b: Register, op: ALUFunction },
    ALUI    { dest: Register, op_a: Register, op_b: Word, op: ALUFunction },
    ALUF    { dest: Register, op_a: Register, op_b: Register, op: ALUFunction },
    ALUFI   { dest: Register, op_a: Register, op_b: Word, op: ALUFunction },
    CMP     { op_a: Register, op_b: Register },
    CMPI    { op_a: Register, op_b: Word },
}

impl Instruction {
    pub fn encode(&self) -> QuadWord {
        match self {
            Instruction::NOP => {
                0x00000000
            },
            Instruction::LW { dest, addr } => {
                0x10000000 | ((*dest as u32) << 22) | ((addr.0 as u32) << 19) | ((addr.1 as u32) << 16)
            },
            Instruction::LWI { dest, addr } =>  {
                0x20000000 | ((*dest as u32) << 22) | *addr as u32
            },
            Instruction::SW { addr, source } =>  {
                0x30000000 | ((addr.0 as u32) << 19) | ((*source as u32) << 16)
            },
            Instruction::SWI { addr, source } =>  {
                0x40000000 | ((*source as u32) << 16) | *addr as u32
            },
            Instruction::MW { dest, source } =>  {
                0x50000000 | ((*dest as u32) << 22) | ((*source as u32) << 19)
            },
            Instruction::MWI { dest, datum } =>  {
                0x60000000 | ((*dest as u32) << 22) | ((*datum as u32) << 8)
            },
            Instruction::JP { addr } =>  {
                0x70000000 | ((addr.0 as u32) << 19) | ((addr.1 as u32) << 16)
            },
            Instruction::JPI { addr } =>  {
                0x80000000 | *addr as u32
            },
            Instruction::ALU { dest, op_a, op_b, op } =>  {
                0x90000000 | (*op as u32) | ((*dest as u32) << 22) | ((*op_a as u32) << 19) | ((*op_b as u32) << 16)
            },
            Instruction::ALUI { dest, op_a, op_b, op } =>  {
                0xA0000000 | (*op as u32) | ((*dest as u32) << 22) | ((*op_a as u32) << 19) | ((*op_b as u32) << 8)
            },
            Instruction::ALUF { dest, op_a, op_b, op } =>  {
                0xB0000000 | (*op as u32) | ((*dest as u32) << 22) | ((*op_a as u32) << 19) | ((*op_b as u32) << 16)
            },
            Instruction::ALUFI { dest, op_a, op_b, op } =>  {
                0xC0000000 | (*op as u32) | ((*dest as u32) << 22) | ((*op_a as u32) << 19) | ((*op_b as u32) << 8)
            },
            Instruction::CMP { op_a, op_b } =>  {
                0xD0000000 | ((*op_a as u32) << 19) | ((*op_b as u32) << 16)
            },
            Instruction::CMPI { op_a, op_b } =>  {
                0xE0000000 | ((*op_a as u32) << 19) | ((*op_b as u32) << 8)
            },
        }
    }
}

impl Register {
    pub fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::R0),
            1 => Some(Self::R1),
            2 => Some(Self::R2),
            3 => Some(Self::R3),
            4 => Some(Self::R4),
            5 => Some(Self::R5),
            6 => Some(Self::R6),
            7 => Some(Self::RP),
            _ => None
        }
    }
}

impl Token {
    pub fn parse(input: &str) -> Option<Self> {
        if let Some(parsed_imm8) = lexer::try_parse_word(input) {
            return Some(Token::Imm8(parsed_imm8));
        }

        if let Some(parsed_imm16) = lexer::try_parse_doubleword(input) {
            return Some(Token::Imm16(parsed_imm16));
        }

        if let Some(parsed_reg) = lexer::try_parse_register(input) {
            return Some(Token::Register(parsed_reg));
        }

        if let Some(parsed_regpair) = lexer::try_parse_register_pair(input) {
            return Some(Token::RegisterPair(parsed_regpair));
        }

        if let Some(parsed_operation) = lexer::try_parse_operation(input) {
            return Some(Token::Operation(parsed_operation));
        }

        if let Some(parsed_label) = lexer::try_parse_label(input) {
            return Some(Token::Label(parsed_label));
        }

        None
    }
}

impl Instruction {
    pub fn from_tokens(tokens: &[Token]) -> Option<Self> {
        let mut mnemonic = Operation::NOP;
        if let Some(Token::Operation(mnem)) = tokens.get(0) {
            mnemonic = *mnem;
        } else {
            return None;
        }
        
/*
    LW      { dest: Register, addr: RegisterPair },
    LWI     { dest: Register, addr: DoubleWord },
    SW      { addr: RegisterPair, source: Register },
    SWI     { addr: DoubleWord, source: Register },
    MW      { dest: Register, source: Register },
    MWI     { dest: Register, datum: Word },
    JP      { addr: RegisterPair },
    JPI     { addr: DoubleWord },
    ALU     { dest: Register, op_a: Register, op_b: Register, op: ALUFunction },
    ALUI    { dest: Register, op_a: Register, op_b: Word, op: ALUFunction },
    ALUF    { dest: Register, op_a: Register, op_b: Register, op: ALUFunction },
    ALUFI   { dest: Register, op_a: Register, op_b: Word, op: ALUFunction },
    CMP     { op_a: Register, op_b: Register },
    CMPI    { op_a: Register, op_b: Word },
*/

        match mnemonic {
            Operation::NOP => if let None = tokens.get(1) {
                Some(Instruction::NOP)
            } else { None },
            Operation::LW => if let (Some(Token::Register(dest)), Some(Token::RegisterPair(addr))) = (tokens.get(1), tokens.get(2)) {
                Some(Instruction::LW { dest: *dest, addr: *addr })
            } else { None },
            Operation::LWI => if let (Some(Token::Register(dest)), Some(Token::Imm16(addr))) = (tokens.get(1), tokens.get(2)) {
                Some(Instruction::LWI { dest: *dest, addr: *addr })
            } else { None }
            Operation::SW => if let (Some(Token::RegisterPair(addr)), Some(Token::Register(source))) = (tokens.get(1), tokens.get(2)) {
                Some(Instruction::SW { addr: *addr, source: *source })
            } else { None }
            Operation::SWI => if let (Some(Token::Imm16(addr)), Some(Token::Register(source))) = (tokens.get(1), tokens.get(2)) {
                Some(Instruction::SWI { addr: *addr, source: *source })
            } else { None },
            Operation::MW => if let (Some(Token::Register(dest)), Some(Token::Register(source))) = (tokens.get(1), tokens.get(2)) {
                Some(Instruction::MW { dest: *dest, source: *source })
            } else { None },
            Operation::MWI => if let (Some(Token::Register(dest)), Some(Token::Imm8(datum))) = (tokens.get(1), tokens.get(2)) {
                Some(Instruction::MWI { dest: *dest, datum: *datum })
            } else { None },
            Operation::JP => if let Some(Token::RegisterPair(addr)) = tokens.get(1) {
                Some(Instruction::JP { addr: *addr })
            } else { None },
            Operation::JPI => if let Some(Token::Imm16(addr)) = tokens.get(1) {
                Some(Instruction::JPI { addr: *addr })
            } else { None },
            Operation::ADD => todo!(),
            Operation::ADC => todo!(),
            Operation::SUB => todo!(),
            Operation::SBB => todo!(),
            Operation::OR => todo!(),
            Operation::NOR => todo!(),
            Operation::XOR => todo!(),
            Operation::AND => todo!(),
            Operation::ADDI => todo!(),
            Operation::ADCI => todo!(),
            Operation::SUBI => todo!(),
            Operation::SBBI => todo!(),
            Operation::ORI => todo!(),
            Operation::NORI => todo!(),
            Operation::XORI => todo!(),
            Operation::ANDI => todo!(),
            Operation::ADDF => todo!(),
            Operation::ADCF => todo!(),
            Operation::SUBF => todo!(),
            Operation::SBBF => todo!(),
            Operation::ORF => todo!(),
            Operation::NORF => todo!(),
            Operation::XORF => todo!(),
            Operation::ANDF => todo!(),
            Operation::ADDFI => todo!(),
            Operation::ADCFI => todo!(),
            Operation::SUBFI => todo!(),
            Operation::SBBFI => todo!(),
            Operation::ORFI => todo!(),
            Operation::NORFI => todo!(),
            Operation::XORFI => todo!(),
            Operation::ANDFI => todo!(),
            Operation::CMP => todo!(),
            Operation::CMPI => todo!(),
        }
    }
}