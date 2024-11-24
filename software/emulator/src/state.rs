use std::io::{self, Read};

use isa::arch::{ControlFlags, DoubleWord, Instruction, QuadWord, Word, ADDRESS_RANGE, NUM_OPCODES, NUM_REGISTERS};

pub struct State {
    pub program_counter: DoubleWord,
    pub registers: [Word; NUM_REGISTERS],
    pub oprom: [ControlFlags; NUM_OPCODES],
    pub main_memory: [Word; ADDRESS_RANGE],
    pub program_memory: [Instruction; ADDRESS_RANGE],
    pub halted: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            registers: [0; NUM_REGISTERS],
            oprom: [Default::default(); NUM_OPCODES],
            main_memory: [0; ADDRESS_RANGE],
            program_memory: [Instruction::default(); ADDRESS_RANGE],
            halted: true,
        }
    }

    pub fn load_oprom(&mut self, file: &str) -> io::Result<()> {
        let contents = std::fs::read(file)?;
        let length = contents.len();

        if let Ok(control_flags) = <[Word; NUM_OPCODES]>::try_from(contents) {
            self.oprom = control_flags.map(|word| ControlFlags::decode(word));
            Ok(())
        } else {
            Err(std::io::Error::new(io::ErrorKind::InvalidData, format!("expected {} bytes but got {}", NUM_OPCODES, length)))
        }
    }

    pub fn load_program(&mut self, file: &str) -> io::Result<()> {
        let contents = std::fs::read(file)?;
        
        if contents.len() % 4 != 0 {
            let message = "non-integer number of instructions";
            Err(std::io::Error::new(io::ErrorKind::InvalidData, message))
        } else if contents.len() / 4 > ADDRESS_RANGE {
            let message = format!("address range is {} but file has {} instructions", ADDRESS_RANGE, contents.len() / 4);
            Err(std::io::Error::new(io::ErrorKind::FileTooLarge, message))
        } else {
            let instructions: Vec<Instruction> = contents.chunks_exact(4).map(|words| {
                let quadword = (words[0] as QuadWord) << 0
                             | (words[1] as QuadWord) << 8
                             | (words[2] as QuadWord) << 16
                             | (words[3] as QuadWord) << 24;
                Instruction::decode(quadword)
            }).collect();
            self.program_memory.fill(Instruction::default());
            self.program_memory[..instructions.len()].copy_from_slice(&instructions);
            Ok(())
        }
    }

    pub fn reset(&mut self) {
        self.registers.fill(0);
        self.program_counter = 0;
    }

    pub fn tick(&mut self) {
        if self.halted {
            return;
        }


    }
}