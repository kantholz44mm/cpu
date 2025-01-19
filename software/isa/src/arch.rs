use std::fmt::Debug;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use strum_macros::{EnumIter, EnumString};

pub type Word = u8;
pub type DoubleWord = u16;
pub type QuadWord = u32;

pub const NUM_REGISTERS: usize = std::mem::variant_count::<Register>();
pub const NUM_OPCODES: usize = std::mem::variant_count::<Opcode>();
pub const ADDRESS_RANGE: usize = DoubleWord::MAX as usize;

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum ALUOperation {
    Add,
    Subtract,
    ShiftLeft,
    ShiftRight,
    Or,
    And,
    Xor,
    Nand
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub rd: Register,
    pub ro1: Register,
    pub ro2: Register,
    pub immediate: DoubleWord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ControlFlags {
    pub regwen: bool,
    pub ioren: bool,
    pub iowen: bool,
    pub pcssel: bool,
    pub halt: bool,
    pub opsel: bool,
    pub fwen: bool,
    pub adrsel: bool,
    pub ressel: bool,
    pub zinv: bool,
    pub aluop: ALUOperation,
    pub cryen: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, FromPrimitive, ToPrimitive, EnumIter, EnumString)]
#[repr(u8)]
#[strum(ascii_case_insensitive)]
pub enum Opcode {
    NOP,
    ADD,
    ADDI,
    ADDNF,
    ADDINF,
    ADC,
    ADCI,
    ADCNF,
    ADCINF,
    SUB,
    SUBI,
    SUBNF,
    SUBINF,
    SBC,
    SBCI,
    SBCNF,
    SBCINF,
    SHL,
    SHLI,
    SHLNF,
    SHLINF,
    SHR,
    SHRI,
    SHRNF,
    SHRINF,
    OR,
    ORI,
    ORNF,
    ORINF,
    AND,
    ANDI,
    ANDNF,
    ANDINF,
    XOR,
    XORI,
    XORNF,
    XORINF,
    NAND,
    NANDI,
    NANDNF,
    NANDINF,
    LW,
    LWI,
    SW,
    SWI,
    BZ,
    BZI,
    BNZ,
    BNZI,
    HCF,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, FromPrimitive, ToPrimitive, Eq)]
#[repr(u8)]
pub enum Register {
    RZ,
    R1,
    R2,
    R3,
    R4,
    RF,
    RL,
    RH
}

#[derive(Clone, Copy, PartialEq, PartialOrd, FromPrimitive, ToPrimitive, Eq)]
#[repr(u8)]
pub enum OperandList {
    Empty,
    RdRo1Ro2,
    RdRo1Imm8,
    RdBracketedRHRL,
    RdBracketedImm16,
    BracketedRHRLRo2,
    BracketedImm16Ro2,
}

impl Instruction {
    pub fn encode(&self) -> QuadWord {
        0 as QuadWord
        | (self.immediate as QuadWord)      << 0
        | (self.ro2 as QuadWord)            << 16
        | (self.ro1 as QuadWord)            << 19
        | (self.rd as QuadWord)             << 22
        | (self.opcode as QuadWord)         << 25
    }

    pub fn decode(word: QuadWord) -> Self {
        unsafe { 
            Self {
                opcode:     std::mem::transmute(((word >> 25) & 0x7F) as Word),
                rd:         std::mem::transmute(((word >> 22) &  0x7) as Word),
                ro1:        std::mem::transmute(((word >> 19) &  0x7) as Word),
                ro2:        std::mem::transmute(((word >> 16) &  0x7) as Word),
                immediate:  word as DoubleWord
            }
        }
    }
}

impl ControlFlags {
    pub fn encode(&self) -> DoubleWord {
        0 as DoubleWord
        | (self.regwen as DoubleWord) << 0
        | (self.ioren  as DoubleWord) << 1
        | (self.iowen  as DoubleWord) << 2
        | (self.pcssel as DoubleWord) << 3
        | (self.halt   as DoubleWord) << 4
        | (self.opsel  as DoubleWord) << 5
        | (self.fwen   as DoubleWord) << 6
        | (self.adrsel as DoubleWord) << 7
        | (self.ressel as DoubleWord) << 8
        | (self.zinv   as DoubleWord) << 9
        | (self.aluop  as DoubleWord) << 10
        | (self.cryen  as DoubleWord) << 13
    }

    pub fn decode(word: DoubleWord) -> Self {
        unsafe { 
            Self {
                regwen: std::mem::transmute(((word >> 0  ) & 0x1) as Word),
                ioren:  std::mem::transmute(((word >> 1  ) & 0x1) as Word),
                iowen:  std::mem::transmute(((word >> 2  ) & 0x1) as Word),
                pcssel: std::mem::transmute(((word >> 3  ) & 0x1) as Word),
                halt:   std::mem::transmute(((word >> 4  ) & 0x1) as Word),
                opsel:  std::mem::transmute(((word >> 5  ) & 0x1) as Word),
                fwen:   std::mem::transmute(((word >> 6  ) & 0x1) as Word),
                adrsel: std::mem::transmute(((word >> 7  ) & 0x1) as Word),
                ressel: std::mem::transmute(((word >> 8  ) & 0x1) as Word),
                zinv:   std::mem::transmute(((word >> 9  ) & 0x1) as Word),
                aluop:  std::mem::transmute(((word >> 10 ) & 0x7) as Word),
                cryen:  std::mem::transmute(((word >> 13 ) & 0x1) as Word),
            }
        }
    }
}

impl Opcode {
    pub fn control_flags(&self) -> ControlFlags {
        match self {
            Opcode::NOP     => ControlFlags { regwen: false, ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: false },
            Opcode::ADD     => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: false },
            Opcode::ADDI    => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: false },
            Opcode::ADDNF   => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: false },
            Opcode::ADDINF  => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: false },
            Opcode::ADC     => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: true  },
            Opcode::ADCI    => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: true  },
            Opcode::ADCNF   => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: true  },
            Opcode::ADCINF  => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: true  },
            Opcode::SUB     => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Subtract,   cryen: false },
            Opcode::SUBI    => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Subtract,   cryen: false },
            Opcode::SUBNF   => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Subtract,   cryen: false },
            Opcode::SUBINF  => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Subtract,   cryen: false },
            Opcode::SBC     => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Subtract,   cryen: true  },
            Opcode::SBCI    => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Subtract,   cryen: true  },
            Opcode::SBCNF   => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Subtract,   cryen: true  },
            Opcode::SBCINF  => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Subtract,   cryen: true  },
            Opcode::SHL     => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::ShiftLeft,  cryen: false },
            Opcode::SHLI    => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::ShiftLeft,  cryen: false },
            Opcode::SHLNF   => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::ShiftLeft,  cryen: false },
            Opcode::SHLINF  => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::ShiftLeft,  cryen: false },
            Opcode::SHR     => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::ShiftRight, cryen: false },
            Opcode::SHRI    => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::ShiftRight, cryen: false },
            Opcode::SHRNF   => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::ShiftRight, cryen: false },
            Opcode::SHRINF  => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::ShiftRight, cryen: false },
            Opcode::OR      => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Or,         cryen: false },
            Opcode::ORI     => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Or,         cryen: false },
            Opcode::ORNF    => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Or,         cryen: false },
            Opcode::ORINF   => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Or,         cryen: false },
            Opcode::AND     => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::And,        cryen: false },
            Opcode::ANDI    => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::And,        cryen: false },
            Opcode::ANDNF   => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::And,        cryen: false },
            Opcode::ANDINF  => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::And,        cryen: false },
            Opcode::XOR     => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Xor,        cryen: false },
            Opcode::XORI    => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Xor,        cryen: false },
            Opcode::XORNF   => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Xor,        cryen: false },
            Opcode::XORINF  => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Xor,        cryen: false },
            Opcode::NAND    => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Nand,       cryen: false },
            Opcode::NANDI   => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: true , adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Nand,       cryen: false },
            Opcode::NANDNF  => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Nand,       cryen: false },
            Opcode::NANDINF => ControlFlags { regwen: true , ioren: false, iowen: false, pcssel: false, halt: false, opsel: true , fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Nand,       cryen: false },
            Opcode::LW      => ControlFlags { regwen: true , ioren: true , iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: true , zinv: false, aluop: ALUOperation::Add,        cryen: false },
            Opcode::LWI     => ControlFlags { regwen: true , ioren: true , iowen: false, pcssel: false, halt: false, opsel: false, fwen: false, adrsel: true , ressel: true , zinv: false, aluop: ALUOperation::Add,        cryen: false },
            Opcode::SW      => ControlFlags { regwen: false, ioren: false, iowen: true , pcssel: false, halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: false },
            Opcode::SWI     => ControlFlags { regwen: false, ioren: false, iowen: true , pcssel: false, halt: false, opsel: false, fwen: false, adrsel: true , ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: false },
            Opcode::BZ      => ControlFlags { regwen: false, ioren: false, iowen: false, pcssel: true , halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: false },
            Opcode::BZI     => ControlFlags { regwen: false, ioren: false, iowen: false, pcssel: true , halt: false, opsel: false, fwen: false, adrsel: true , ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: false },
            Opcode::BNZ     => ControlFlags { regwen: false, ioren: false, iowen: false, pcssel: true , halt: false, opsel: false, fwen: false, adrsel: false, ressel: false, zinv: true , aluop: ALUOperation::Add,        cryen: false },
            Opcode::BNZI    => ControlFlags { regwen: false, ioren: false, iowen: false, pcssel: true , halt: false, opsel: false, fwen: false, adrsel: true , ressel: false, zinv: true , aluop: ALUOperation::Add,        cryen: false },
            Opcode::HCF     => ControlFlags { regwen: false, ioren: false, iowen: false, pcssel: false, halt: true , opsel: false, fwen: false, adrsel: false, ressel: false, zinv: false, aluop: ALUOperation::Add,        cryen: false },
        }
    }

    pub fn operandlist_type(&self) -> OperandList {
        match self {
            Opcode::NOP     => OperandList::Empty,
            Opcode::ADD     => OperandList::RdRo1Ro2,
            Opcode::ADDI    => OperandList::RdRo1Imm8,
            Opcode::ADDNF   => OperandList::RdRo1Ro2,
            Opcode::ADDINF  => OperandList::RdRo1Imm8,
            Opcode::ADC     => OperandList::RdRo1Ro2,
            Opcode::ADCI    => OperandList::RdRo1Imm8,
            Opcode::ADCNF   => OperandList::RdRo1Ro2,
            Opcode::ADCINF  => OperandList::RdRo1Imm8,
            Opcode::SUB     => OperandList::RdRo1Ro2,
            Opcode::SUBI    => OperandList::RdRo1Imm8,
            Opcode::SUBNF   => OperandList::RdRo1Ro2,
            Opcode::SUBINF  => OperandList::RdRo1Imm8,
            Opcode::SBC     => OperandList::RdRo1Ro2,
            Opcode::SBCI    => OperandList::RdRo1Imm8,
            Opcode::SBCNF   => OperandList::RdRo1Ro2,
            Opcode::SBCINF  => OperandList::RdRo1Imm8,
            Opcode::SHL     => OperandList::RdRo1Ro2,
            Opcode::SHLI    => OperandList::RdRo1Imm8,
            Opcode::SHLNF   => OperandList::RdRo1Ro2,
            Opcode::SHLINF  => OperandList::RdRo1Imm8,
            Opcode::SHR     => OperandList::RdRo1Ro2,
            Opcode::SHRI    => OperandList::RdRo1Imm8,
            Opcode::SHRNF   => OperandList::RdRo1Ro2,
            Opcode::SHRINF  => OperandList::RdRo1Imm8,
            Opcode::OR      => OperandList::RdRo1Ro2,
            Opcode::ORI     => OperandList::RdRo1Imm8,
            Opcode::ORNF    => OperandList::RdRo1Ro2,
            Opcode::ORINF   => OperandList::RdRo1Imm8,
            Opcode::AND     => OperandList::RdRo1Ro2,
            Opcode::ANDI    => OperandList::RdRo1Imm8,
            Opcode::ANDNF   => OperandList::RdRo1Ro2,
            Opcode::ANDINF  => OperandList::RdRo1Imm8,
            Opcode::XOR     => OperandList::RdRo1Ro2,
            Opcode::XORI    => OperandList::RdRo1Imm8,
            Opcode::XORNF   => OperandList::RdRo1Ro2,
            Opcode::XORINF  => OperandList::RdRo1Imm8,
            Opcode::NAND    => OperandList::RdRo1Ro2,
            Opcode::NANDI   => OperandList::RdRo1Imm8,
            Opcode::NANDNF  => OperandList::RdRo1Ro2,
            Opcode::NANDINF => OperandList::RdRo1Imm8,
            Opcode::LW      => OperandList::RdBracketedRHRL,
            Opcode::LWI     => OperandList::RdBracketedImm16,
            Opcode::SW      => OperandList::BracketedRHRLRo2,
            Opcode::SWI     => OperandList::BracketedImm16Ro2,
            Opcode::BZ      => OperandList::BracketedRHRLRo2,
            Opcode::BZI     => OperandList::BracketedImm16Ro2,
            Opcode::BNZ     => OperandList::BracketedRHRLRo2,
            Opcode::BNZI    => OperandList::BracketedImm16Ro2,
            Opcode::HCF     => OperandList::Empty,
        }
    }
}

impl TryFrom<QuadWord> for Opcode {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match FromPrimitive::from_u32(value) {
            Some(opcode) => Ok(opcode),
            None => Err(format!("unknown opcode: {value}")),
        }
    }
}

impl TryFrom<QuadWord> for Register {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match FromPrimitive::from_u32(value) {
            Some(register) => Ok(register),
            None => Err(format!("unknown register index: {value}")),
        }
    }
}

impl Default for Opcode {
    fn default() -> Self {
        Self::ADC
    }
}

impl Default for Register {
    fn default() -> Self {
        Self::RZ
    }
}

impl Default for ALUOperation {
    fn default() -> Self {
        Self::Add
    }
}

impl Debug for OperandList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "/"),
            Self::RdRo1Ro2 => write!(f, "rd, ro1, ro2"),
            Self::RdRo1Imm8 => write!(f, "rd, ro1, imm8"),
            Self::RdBracketedRHRL => write!(f, "rd, [RH:RL]"),
            Self::RdBracketedImm16 => write!(f, "rd, [imm16]"),
            Self::BracketedRHRLRo2 => write!(f, "[RH:RL], ro2"),
            Self::BracketedImm16Ro2 => write!(f, "[imm16], ro2"),
        }
    }
}