use num_derive::{FromPrimitive, ToPrimitive};

pub type Word = u8;
pub type DoubleWord = u16;
pub type QuadWord = u32;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OperandSelect {
    OperandRegister,
    OperandImmediate,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum FlagWriteMode {
    DontWriteFlags,
    WriteFlags,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand: OperandSelect,
    pub flags: FlagWriteMode,
    pub rd: Register,
    pub ro1: Register,
    pub ro2: Register,
    pub immediate: DoubleWord,
}

#[derive(Debug, Clone, Copy)]
pub struct ControlFlags {
    pub regwen: bool,
    pub adrwen: bool,
    pub ioren: bool,
    pub iowen: bool,
    pub pcssel: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
#[repr(u8)]
pub enum Opcode {
    ADC,
    SBB,
    SHL,
    SHR,
    
    OR,
    NOR,
    XOR,
    AND,

    LW,
    SW,
    BZ,
    BNZ,

    JZ,
    JNZ,
    LA,
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

impl Instruction {
    pub fn encode(&self) -> QuadWord {
        0 as QuadWord
        | (self.immediate as QuadWord)      << 0
        | (self.ro2 as QuadWord)            << 16
        | (self.ro1 as QuadWord)            << 19
        | (self.rd as QuadWord)             << 22
        | (self.flags as QuadWord)          << 26
        | (self.operand as QuadWord)        << 27
        | (self.opcode as QuadWord)         << 28
    }
}

impl ControlFlags {
    pub fn encode(&self) -> Word {
        0 as Word
        | (self.regwen as Word) << 0
        | (self.adrwen as Word) << 1
        | (self.ioren as Word)  << 2
        | (self.iowen as Word)  << 3
        | (self.pcssel as Word) << 4
    }
}

impl Opcode {
    pub fn control_flags(&self) -> ControlFlags {
        match self {
            Opcode::ADC  => ControlFlags { regwen: true , adrwen: false, ioren: false, iowen: false, pcssel: false },
            Opcode::SBB  => ControlFlags { regwen: true , adrwen: false, ioren: false, iowen: false, pcssel: false },
            Opcode::SHL  => ControlFlags { regwen: true , adrwen: false, ioren: false, iowen: false, pcssel: false },
            Opcode::SHR  => ControlFlags { regwen: true , adrwen: false, ioren: false, iowen: false, pcssel: false },
            Opcode::OR   => ControlFlags { regwen: true , adrwen: false, ioren: false, iowen: false, pcssel: false },
            Opcode::NOR  => ControlFlags { regwen: true , adrwen: false, ioren: false, iowen: false, pcssel: false },
            Opcode::XOR  => ControlFlags { regwen: true , adrwen: false, ioren: false, iowen: false, pcssel: false },
            Opcode::AND  => ControlFlags { regwen: true , adrwen: false, ioren: false, iowen: false, pcssel: false },
            Opcode::LW   => ControlFlags { regwen: true , adrwen: false, ioren: true , iowen: false, pcssel: false },
            Opcode::SW   => ControlFlags { regwen: false, adrwen: false, ioren: false, iowen: true , pcssel: false },
            Opcode::BZ   => ControlFlags { regwen: false, adrwen: false, ioren: false, iowen: false, pcssel: true  },
            Opcode::BNZ  => ControlFlags { regwen: false, adrwen: false, ioren: false, iowen: false, pcssel: true  },
            Opcode::JZ   => ControlFlags { regwen: false, adrwen: false, ioren: false, iowen: false, pcssel: true  },
            Opcode::JNZ  => ControlFlags { regwen: false, adrwen: false, ioren: false, iowen: false, pcssel: true  },
            Opcode::LA   => ControlFlags { regwen: false, adrwen: true , ioren: false, iowen: false, pcssel: false },
            Opcode::HCF  => ControlFlags { regwen: false, adrwen: false, ioren: false, iowen: false, pcssel: false },
        }
    }

    pub fn is_arithmetic(&self) -> bool {
        match self {
            Opcode::ADC |
            Opcode::SBB |
            Opcode::OR  |
            Opcode::NOR |
            Opcode::XOR |
            Opcode::AND |
            Opcode::SHL |
            Opcode::SHR => true,
            _ => false
        }
    }

    pub fn is_branch(&self) -> bool {
        match self {
            Opcode::BZ |
            Opcode::BNZ => true,
            _ => false
        }
    }

    pub fn is_jump(&self) -> bool {
        match self {
            Opcode::JZ |
            Opcode::JNZ => true,
            _ => false
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

impl Default for OperandSelect {
    fn default() -> Self {
        Self::OperandRegister
    }
}

impl Default for FlagWriteMode {
    fn default() -> Self {
        Self::DontWriteFlags
    }
}