use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToBytes;

pub type Word = u8;
pub type DoubleWord = u16;
pub type QuadWord = u32;

const NUM_INSTRUCTIONS: usize = 16;

#[derive(Debug, Clone, Copy, Default)]
pub struct Instruction {
    pub op: Operation,
    pub cond: Condition,
    pub dest: Register,
    pub op1: Register,
    pub op2: Register,
    pub imm: DoubleWord,
}

#[derive(Debug, Clone, Copy)]
pub struct ControlFlags {
    pub rwen: bool,
    pub fwen: bool,
    pub ioren: bool,
    pub iowen: bool,
    pub bssel: bool,
    pub assel: bool,
    pub rssel: bool,
    pub pcssel: bool,
    pub prwen: bool
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, FromPrimitive, ToPrimitive, Eq)]
#[repr(u8)]
pub enum ALUFunction {
    ADD       = 0x0,
    ADC       = 0x1,
    SUB       = 0x2,
    SBB       = 0x3,
    OR        = 0x4,
    NOR       = 0x5,
    XOR       = 0x6,
    AND       = 0x7,
    SHL       = 0x8,
    SHR       = 0x9,
    Reserved0 = 0xA,
    Reserved1 = 0xB,
    Reserved2 = 0xC,
    Reserved3 = 0xD,
    Reserved4 = 0xE,
    Reserved5 = 0xF,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
#[repr(u8)]
pub enum Operation {
    NOP,
    LW,
    LWI,
    SW,
    SWI,
    JP,
    JPI,
    ALU(ALUFunction),
    ALUI(ALUFunction),
    CMP(ALUFunction),
    CMPI(ALUFunction),
    WPR,
    WPRI,
    HCF,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, FromPrimitive, ToPrimitive, Eq)]
#[repr(u8)]
pub enum Condition {
    Always          = 0x0,
    CarryOrBorrow   = 0x1,
    Equality        = 0x2,
    Zero            = 0x3,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, FromPrimitive, ToPrimitive, Eq)]
#[repr(u8)]
pub enum Register {
    R0 = 0x0,
    R1 = 0x1,
    R2 = 0x2,
    R3 = 0x3,
}

impl Instruction {
    pub fn encode(&self) -> QuadWord {
        let aluop = match self.op {
            Operation::ALU(aluop)  |
            Operation::ALUI(aluop) |
            Operation::CMP(aluop)  |
            Operation::CMPI(aluop) => aluop,
            _ => ALUFunction::ADD
        };

        0 as QuadWord
        | (self.imm as QuadWord)            << 0
        | (self.op2 as QuadWord)            << 16
        | (self.op1 as QuadWord)            << 18
        | (self.dest as QuadWord)           << 20
        | (aluop as QuadWord)               << 22
        | (self.cond as QuadWord)           << 26
        | (self.op.opcode() as QuadWord)    << 28
    }
}

impl ControlFlags {
    pub fn encode(&self) -> DoubleWord {
        0 as DoubleWord
        | (self.rwen as DoubleWord)   << 0
        | (self.fwen as DoubleWord)   << 1
        | (self.ioren as DoubleWord)  << 2
        | (self.iowen as DoubleWord)  << 3
        | (self.bssel as DoubleWord)  << 4
        | (self.assel as DoubleWord)  << 5
        | (self.rssel as DoubleWord)  << 6
        | (self.pcssel as DoubleWord) << 7
        | (self.prwen as DoubleWord)  << 8
    }
}

impl Operation {
    pub fn control_flags(&self) -> ControlFlags {
        match self {
            Operation::NOP      => ControlFlags { rwen: false, fwen: false, ioren: false, iowen: false, bssel: false, assel: false, rssel: true , pcssel: false, prwen: false },
            Operation::LW       => ControlFlags { rwen: true , fwen: false, ioren: true , iowen: false, bssel: false, assel: false, rssel: true , pcssel: false, prwen: false },
            Operation::LWI      => ControlFlags { rwen: true , fwen: false, ioren: true , iowen: false, bssel: false, assel: true , rssel: true , pcssel: false, prwen: false },
            Operation::SW       => ControlFlags { rwen: false, fwen: false, ioren: false, iowen: true , bssel: false, assel: false, rssel: true , pcssel: false, prwen: false },
            Operation::SWI      => ControlFlags { rwen: false, fwen: false, ioren: false, iowen: true , bssel: false, assel: true , rssel: true , pcssel: false, prwen: false },
            Operation::JP       => ControlFlags { rwen: false, fwen: false, ioren: false, iowen: false, bssel: false, assel: false, rssel: true , pcssel: true , prwen: false },
            Operation::JPI      => ControlFlags { rwen: false, fwen: false, ioren: false, iowen: false, bssel: false, assel: true , rssel: false, pcssel: true , prwen: false },
            Operation::ALU(_)   => ControlFlags { rwen: true , fwen: true , ioren: false, iowen: false, bssel: true , assel: false, rssel: false, pcssel: false, prwen: false },
            Operation::ALUI(_)  => ControlFlags { rwen: true , fwen: true , ioren: false, iowen: false, bssel: false, assel: false, rssel: false, pcssel: false, prwen: false },
            Operation::CMP(_)   => ControlFlags { rwen: false, fwen: true , ioren: false, iowen: false, bssel: true , assel: false, rssel: false, pcssel: false, prwen: false },
            Operation::CMPI(_)  => ControlFlags { rwen: false, fwen: true , ioren: false, iowen: false, bssel: false, assel: false, rssel: false, pcssel: false, prwen: false },
            Operation::WPR      => ControlFlags { rwen: false, fwen: false, ioren: false, iowen: false, bssel: true , assel: false, rssel: false, pcssel: false, prwen: true  },
            Operation::WPRI     => ControlFlags { rwen: false, fwen: false, ioren: false, iowen: false, bssel: false, assel: false, rssel: false, pcssel: false, prwen: true  },
            Operation::HCF      => ControlFlags { rwen: false, fwen: false, ioren: false, iowen: false, bssel: false, assel: false, rssel: false, pcssel: false, prwen: false },
        }
    }

    pub fn opcode(&self) -> Word {
        match self {
            Operation::NOP      => 0x0,
            Operation::LW       => 0x1,
            Operation::LWI      => 0x2,
            Operation::SW       => 0x3,
            Operation::SWI      => 0x4,
            Operation::JP       => 0x5,
            Operation::JPI      => 0x6,
            Operation::ALU(_)   => 0x7,
            Operation::ALUI(_)  => 0x8,
            Operation::CMP(_)   => 0x9,
            Operation::CMPI(_)  => 0xA,
            Operation::WPR      => 0xB,
            Operation::WPRI     => 0xC,
            Operation::HCF      => 0xF,
        }
    }
}

pub fn compile_microcode() -> Vec<u8> {
    [
        Operation::NOP,
        Operation::LW,
        Operation::LWI,
        Operation::SW,
        Operation::SWI,
        Operation::JP,
        Operation::JPI,
        Operation::ALU(ALUFunction::ADD),
        Operation::ALUI(ALUFunction::ADD),
        Operation::CMP(ALUFunction::ADD),
        Operation::CMPI(ALUFunction::ADD),
        Operation::WPR,
        Operation::WPRI,
        Operation::NOP, // reserved
        Operation::NOP, // reserved
        Operation::HCF,
    ].iter().flat_map(|opcode| opcode.control_flags().encode().to_le_bytes()).collect()
}

impl Default for Operation {
    fn default() -> Self {
        Self::NOP
    }
}

impl Default for Condition {
    fn default() -> Self {
        Self::Always
    }
}

impl Default for Register {
    fn default() -> Self {
        Self::R0
    }
}

impl Default for ALUFunction {
    fn default() -> Self {
        Self::ADD
    }
}