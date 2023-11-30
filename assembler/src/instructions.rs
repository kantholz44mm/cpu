use crate::lexer;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
    RP = 3,
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
    HCF,
}

#[derive(Debug, Clone)]
pub enum Token {
    Immediate(DoubleWord),
    Register(Register),
    RegisterPair(RegisterPair),
    Operation(Operation, Condition),
    Label(Label)
}

#[derive(Debug, Clone, Copy, EnumIter)]
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
    CMPI  = 0xE,
    HCF   = 0xF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Always          = 0,
    CarryOrBorrow   = 1,
    Overflow        = 2,
    Equal           = 3,
    Negative        = 4,
    Zero            = 5,
    NotEqual        = 6,
    NotZero         = 7,
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub operation: Opcode,
    pub condition: Condition,
    pub alu: ALUFunction,
    pub destination: Register,
    pub operand1: Register,
    pub operand2: Register,
    pub immediate: DoubleWord,
}

pub enum PCSSEL {
    Zero        = 0b00,
    Increment   = 0b01,
    R1R2        = 0b10,
    Imm16       = 0b11,
}

pub enum RSSEL {
    Imm8        = 0b00,
    Memory      = 0b01,
    ALU         = 0b10,
    Register1   = 0b11,
}

pub enum BSSEL {
    Imm8       = 0b0,
    Register2  = 0b1
}

pub enum MASEL {
    Imm16       = 0b0,
    R1RP        = 0b1,
}

pub struct ControlLines {
    pub RWEN: bool,
    pub FWEN: bool,
    pub MREN: bool,
    pub MWEN: bool,
    pub BSSEL: BSSEL,
    pub MASEL: MASEL,
    pub RSSEL: RSSEL,
    pub PCSSEL: PCSSEL,
}

impl Instruction {
    pub fn encode(&self) -> QuadWord {
        (0 as QuadWord)
        | (self.operation as u32) << 28
        | (self.condition as u32) << 25
        | (self.alu as u32) << 22
        | (self.destination as u32) << 20
        | (self.operand1 as u32) << 18
        | (self.operand2 as u32) << 16
        | (self.immediate as u32)
    }
}

impl Opcode {
    pub fn get_control_lines(&self) -> ControlLines {
        match self {
            Opcode::NOP   => ControlLines { RWEN: false, FWEN: false, MREN: false, MWEN: false, BSSEL: BSSEL::Imm8, MASEL: MASEL::Imm16, RSSEL: RSSEL::Imm8, PCSSEL: PCSSEL::Increment },
            Opcode::LW    => ControlLines { RWEN: true, FWEN: false, MREN: true, MWEN: false, BSSEL: BSSEL::Imm8, MASEL: MASEL::R1RP, RSSEL: RSSEL::Memory, PCSSEL: PCSSEL::Increment },
            Opcode::LWI   => ControlLines { RWEN: true, FWEN: false, MREN: true, MWEN: false, BSSEL: BSSEL::Imm8, MASEL: MASEL::Imm16, RSSEL: RSSEL::Memory, PCSSEL: PCSSEL::Increment },
            Opcode::SW    => ControlLines { RWEN: false, FWEN: false, MREN: false, MWEN: true, BSSEL: BSSEL::Imm8, MASEL: MASEL::R1RP, RSSEL: RSSEL::Imm8, PCSSEL: PCSSEL::Increment },
            Opcode::SWI   => ControlLines { RWEN: false, FWEN: false, MREN: false, MWEN: true, BSSEL: BSSEL::Imm8, MASEL: MASEL::Imm16, RSSEL: RSSEL::Imm8, PCSSEL: PCSSEL::Increment },
            Opcode::MW    => ControlLines { RWEN: true, FWEN: false, MREN: false, MWEN: false, BSSEL: BSSEL::Imm8, MASEL: MASEL::Imm16, RSSEL: RSSEL::Register1, PCSSEL: PCSSEL::Increment },
            Opcode::MWI   => ControlLines { RWEN: true, FWEN: false, MREN: false, MWEN: false, BSSEL: BSSEL::Imm8, MASEL: MASEL::Imm16, RSSEL: RSSEL::Imm8, PCSSEL: PCSSEL::Increment },
            Opcode::JP    => ControlLines { RWEN: false, FWEN: false, MREN: false, MWEN: false, BSSEL: BSSEL::Imm8, MASEL: MASEL::Imm16, RSSEL: RSSEL::Imm8, PCSSEL: PCSSEL::R1R2 },
            Opcode::JPI   => ControlLines { RWEN: false, FWEN: false, MREN: false, MWEN: false, BSSEL: BSSEL::Imm8, MASEL: MASEL::Imm16, RSSEL: RSSEL::Imm8, PCSSEL: PCSSEL::Imm16 },
            Opcode::ALU   => ControlLines { RWEN: true, FWEN: false, MREN: false, MWEN: false, BSSEL: BSSEL::Register2, MASEL: MASEL::Imm16, RSSEL: RSSEL::ALU, PCSSEL: PCSSEL::Increment },
            Opcode::ALUI  => ControlLines { RWEN: true, FWEN: false, MREN: false, MWEN: false, BSSEL: BSSEL::Imm8, MASEL: MASEL::Imm16, RSSEL: RSSEL::ALU, PCSSEL: PCSSEL::Increment },
            Opcode::ALUF  => ControlLines { RWEN: true, FWEN: true, MREN: false, MWEN: false, BSSEL: BSSEL::Register2, MASEL: MASEL::Imm16, RSSEL: RSSEL::ALU, PCSSEL: PCSSEL::Increment },
            Opcode::ALUFI => ControlLines { RWEN: true, FWEN: true, MREN: false, MWEN: false, BSSEL: BSSEL::Imm8, MASEL: MASEL::Imm16, RSSEL: RSSEL::ALU, PCSSEL: PCSSEL::Increment },
            Opcode::CMP   => ControlLines { RWEN: false, FWEN: true, MREN: false, MWEN: false, BSSEL: BSSEL::Register2, MASEL: MASEL::Imm16, RSSEL: RSSEL::Imm8, PCSSEL: PCSSEL::Increment },
            Opcode::CMPI  => ControlLines { RWEN: false, FWEN: true, MREN: false, MWEN: false, BSSEL: BSSEL::Imm8, MASEL: MASEL::Imm16, RSSEL: RSSEL::Imm8, PCSSEL: PCSSEL::Increment },
            Opcode::HCF   => ControlLines { RWEN: false, FWEN: false, MREN: false, MWEN: false, BSSEL: BSSEL::Imm8, MASEL: MASEL::Imm16, RSSEL: RSSEL::Imm8, PCSSEL: PCSSEL::Increment },
        }
    }
}

impl Register {
    pub fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::R0),
            1 => Some(Self::R1),
            2 => Some(Self::R2),
            3 => Some(Self::RP),
            _ => None
        }
    }
}

impl Token {
    pub fn parse(input: &str) -> Option<Self> {
        if let Some(parsed_immediate) = lexer::try_parse_doubleword(input) {
            return Some(Token::Immediate(parsed_immediate));
        }

        if let Some(parsed_reg) = lexer::try_parse_register(input) {
            return Some(Token::Register(parsed_reg));
        }

        if let Some(parsed_regpair) = lexer::try_parse_register_pair(input) {
            return Some(Token::RegisterPair(parsed_regpair));
        }

        if let Some(parsed_operation) = lexer::try_parse_operation(input) {
            return Some(Token::Operation(parsed_operation.0, parsed_operation.1));
        }

        if let Some(parsed_label) = lexer::try_parse_label(input) {
            return Some(Token::Label(parsed_label));
        }

        None
    }
}

impl ControlLines {
    pub fn encode(self) -> u16 {
        (0 as u16)
        | (self.RWEN as u16) << 0
        | (self.FWEN as u16) << 1
        | (self.MREN as u16) << 2
        | (self.MWEN as u16) << 3
        | (self.BSSEL as u16) << 4
        | (self.MASEL as u16) << 5
        | (self.RSSEL as u16) << 6
        | (self.PCSSEL as u16) << 8
    }
}

impl Instruction {
    /// returns a tuple containing the number of tokens consumed, and a Token, if successful.
    /// if not successful, you've got a syntax error. 
    pub fn from_tokens(tokens: &[Token]) -> Option<(usize, Self)> {
        if let Some(Token::Operation(operation, condition)) = tokens.get(0) {
            return match operation {
                Operation::NOP => Some((1, Instruction
                {
                    operation: Opcode::NOP,
                    destination: Register::R0,
                    operand1: Register::R0,
                    operand2: Register::R0,
                    immediate: 0,
                    condition: *condition,
                    alu: ALUFunction::ADD,
                })),
                Operation::LW => if let (Token::Register(dest), Token::RegisterPair((lower_addr, Register::RP))) = (tokens.get(1)?, tokens.get(2)?) {
                    Some((3, Instruction
                    {
                        operation: Opcode::LW,
                        destination: *dest,
                        operand1: *lower_addr,
                        operand2: Register::RP,
                        immediate: 0,
                        condition: *condition,
                        alu: ALUFunction::ADD,
                    }))
                } else { None },
                Operation::LWI => if let (Some(Token::Register(dest)), Some(Token::Immediate(addr))) = (tokens.get(1), tokens.get(2)) {
                    Some((3, Instruction
                    {
                        operation: Opcode::LWI,
                        destination: *dest,
                        operand1: Register::R0,
                        operand2: Register::R0,
                        immediate: *addr,
                        condition: *condition,
                        alu: ALUFunction::ADD,
                    }))
                } else { None },
                Operation::SW => if let (Some(Token::RegisterPair((lower_addr, Register::RP))), Some(Token::Register(source))) = (tokens.get(1), tokens.get(2)) {
                    Some((3, Instruction
                    {
                        operation: Opcode::SW,
                        destination: Register::R0,
                        operand1: *lower_addr,
                        operand2: *source,
                        immediate: 0,
                        condition: *condition,
                        alu: ALUFunction::ADD,
                    }))
                } else { None },
                Operation::SWI => if let (Some(Token::Immediate(addr)), Some(Token::Register(source))) = (tokens.get(1), tokens.get(2)) {
                    Some((3, Instruction
                    {
                        operation: Opcode::SWI,
                        destination: Register::R0,
                        operand1: Register::R0,
                        operand2: *source,
                        immediate: *addr,
                        condition: *condition,
                        alu: ALUFunction::ADD,
                    }))
                } else { None },
                Operation::MW => if let (Some(Token::Register(dest)), Some(Token::Register(source))) = (tokens.get(1), tokens.get(2)) {
                    Some((3, Instruction
                    {
                        operation: Opcode::MW,
                        destination: *dest,
                        operand1: *source,
                        operand2: Register::R0,
                        immediate: 0,
                        condition: *condition,
                        alu: ALUFunction::ADD,
                    }))
                } else { None },
                Operation::MWI => if let (Some(Token::Register(dest)), Some(Token::Immediate(datum))) = (tokens.get(1), tokens.get(2)) {
                    Some((3, Instruction
                    {
                        operation: Opcode::MWI,
                        destination: *dest,
                        operand1: Register::R0,
                        operand2: Register::R0,
                        immediate: *datum,
                        condition: *condition,
                        alu: ALUFunction::ADD,
                    }))
                } else { None },
                Operation::JP => if let Some(Token::RegisterPair((lower, upper))) = tokens.get(1) {
                    Some((2, Instruction
                    {
                        operation: Opcode::JP,
                        destination: Register::R0,
                        operand1: *lower,
                        operand2: *upper,
                        immediate: 0,
                        condition: *condition,
                        alu: ALUFunction::ADD,
                    }))
                } else { None },
                Operation::JPI => if let Some(Token::Immediate(addr)) = tokens.get(1) {
                    Some((2, Instruction
                    {
                        operation: Opcode::JPI,
                        destination: Register::R0,
                        operand1: Register::R0,
                        operand2: Register::R0,
                        immediate: *addr,
                        condition: *condition,
                        alu: ALUFunction::ADD,
                    }))
                } else { None },
                Operation::ADD |
                Operation::ADC |
                Operation::SUB |
                Operation::SBB |
                Operation::OR |
                Operation::NOR |
                Operation::XOR |
                Operation::AND => if let (Token::Register(dest), Token::Register(op_a), Token::Register(op_b)) = (tokens.get(1)?, tokens.get(2)?, tokens.get(3)?) {
                    Some((4, Instruction
                        {
                            operation: Opcode::ALU,
                            destination: *dest,
                            operand1: *op_a,
                            operand2: *op_b,
                            immediate: 0,
                            condition: *condition,
                            alu: operation.get_alu_function(),
                        }))
                } else { None },
                Operation::ADDI |
                Operation::ADCI |
                Operation::SUBI |
                Operation::SBBI |
                Operation::ORI |
                Operation::NORI |
                Operation::XORI |
                Operation::ANDI => if let (Token::Register(dest), Token::Register(op_a), Token::Immediate(op_b)) = (tokens.get(1)?, tokens.get(2)?, tokens.get(3)?) {
                    Some((4, Instruction
                        {
                            operation: Opcode::ALUI,
                            destination: *dest,
                            operand1: *op_a,
                            operand2: Register::R0,
                            immediate: *op_b,
                            condition: *condition,
                            alu: operation.get_alu_function(),
                        }))
                } else { None },
                Operation::ADDF |
                Operation::ADCF |
                Operation::SUBF |
                Operation::SBBF |
                Operation::ORF |
                Operation::NORF |
                Operation::XORF |
                Operation::ANDF => if let (Token::Register(dest), Token::Register(op_a), Token::Register(op_b)) = (tokens.get(1)?, tokens.get(2)?, tokens.get(3)?) {
                    Some((4, Instruction
                        {
                            operation: Opcode::ALUF,
                            destination: *dest,
                            operand1: *op_a,
                            operand2: *op_b,
                            immediate: 0,
                            condition: *condition,
                            alu: operation.get_alu_function(),
                        }))
                } else { None },
                Operation::ADDFI |
                Operation::ADCFI |
                Operation::SUBFI |
                Operation::SBBFI |
                Operation::ORFI |
                Operation::NORFI |
                Operation::XORFI |
                Operation::ANDFI => if let (Token::Register(dest), Token::Register(op_a), Token::Immediate(op_b)) = (tokens.get(1)?, tokens.get(2)?, tokens.get(3)?) {
                    Some((4, Instruction
                        {
                            operation: Opcode::ALUFI,
                            destination: *dest,
                            operand1: *op_a,
                            operand2: Register::R0,
                            immediate: *op_b,
                            condition: *condition,
                            alu: operation.get_alu_function(),
                        }))
                } else { None },
                Operation::CMP => if let (Some(Token::Register(op_a)), Some(Token::Register(op_b))) = (tokens.get(1), tokens.get(2)) {
                    Some((3, Instruction
                    {
                        operation: Opcode::CMP,
                        destination: Register::R0,
                        operand1: *op_a,
                        operand2: *op_b,
                        immediate: 0,
                        condition: *condition,
                        alu: ALUFunction::ADD,
                    }))
                } else { None },
                Operation::CMPI => if let (Some(Token::Register(op_a)), Some(Token::Immediate(op_b))) = (tokens.get(1), tokens.get(2)) {
                    Some((3, Instruction
                    {
                        operation: Opcode::CMPI,
                        destination: Register::R0,
                        operand1: *op_a,
                        operand2: Register::R0,
                        immediate: *op_b,
                        condition: *condition,
                        alu: ALUFunction::ADD,
                    }))
                } else { None },
                Operation::HCF => Some((1, Instruction
                {
                    operation: Opcode::HCF,
                    destination: Register::R0,
                    operand1: Register::R0,
                    operand2: Register::R0,
                    immediate: 0,
                    condition: *condition,
                    alu: ALUFunction::ADD,
                })),
                _ => None,
            };
        }

        None
    }
}

impl Operation {
    pub fn get_alu_function(&self) -> ALUFunction {
        match self {
            Operation::NOP => ALUFunction::ADD,
            Operation::LW => ALUFunction::ADD,
            Operation::LWI => ALUFunction::ADD,
            Operation::SW => ALUFunction::ADD,
            Operation::SWI => ALUFunction::ADD,
            Operation::MW => ALUFunction::ADD,
            Operation::MWI => ALUFunction::ADD,
            Operation::JP => ALUFunction::ADD,
            Operation::JPI => ALUFunction::ADD,
            Operation::ADD => ALUFunction::ADD,
            Operation::ADC => ALUFunction::ADC,
            Operation::SUB => ALUFunction::SUB,
            Operation::SBB => ALUFunction::SBB,
            Operation::OR => ALUFunction::OR,
            Operation::NOR => ALUFunction::NOR,
            Operation::XOR => ALUFunction::XOR,
            Operation::AND => ALUFunction::AND,
            Operation::ADDI => ALUFunction::ADD,
            Operation::ADCI => ALUFunction::ADC,
            Operation::SUBI => ALUFunction::SUB,
            Operation::SBBI => ALUFunction::SBB,
            Operation::ORI => ALUFunction::OR,
            Operation::NORI => ALUFunction::NOR,
            Operation::XORI => ALUFunction::XOR,
            Operation::ANDI => ALUFunction::AND,
            Operation::ADDF => ALUFunction::ADD,
            Operation::ADCF => ALUFunction::ADC,
            Operation::SUBF => ALUFunction::SUB,
            Operation::SBBF => ALUFunction::SBB,
            Operation::ORF => ALUFunction::OR,
            Operation::NORF => ALUFunction::NOR,
            Operation::XORF => ALUFunction::XOR,
            Operation::ANDF => ALUFunction::AND,
            Operation::ADDFI => ALUFunction::ADD,
            Operation::ADCFI => ALUFunction::ADC,
            Operation::SUBFI => ALUFunction::SUB,
            Operation::SBBFI => ALUFunction::SBB,
            Operation::ORFI => ALUFunction::OR,
            Operation::NORFI => ALUFunction::NOR,
            Operation::XORFI => ALUFunction::XOR,
            Operation::ANDFI => ALUFunction::AND,
            Operation::CMP => ALUFunction::ADD,
            Operation::CMPI => ALUFunction::ADD,
            Operation::HCF => ALUFunction::ADD,
        }
    }
}