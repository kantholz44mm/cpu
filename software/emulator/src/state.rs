use std::ops::Range;

use image::{ImageBuffer, Luma};
use isa::arch::{ControlFlags, DoubleWord, Instruction, QuadWord, Register, Word};
use crate::combinatorics::{alu, and, mux_doubleword, mux_word, xor};


#[derive(Clone, Copy)]
pub struct State {
    pub oprom: [DoubleWord; 128],
    pub progrom: [QuadWord; 65536],
    pub memory: [Word; 65536],
    pub registers: [Word; 8],
    pub pc: DoubleWord,
    pub halt: bool,
}

impl State {
    pub fn new(oprom_file: &str, progrom_file: &str) -> Result<Self, String> {
        let oprom = std::fs::read(oprom_file).map_err(|err| err.to_string())?;
        let progrom = std::fs::read(progrom_file).map_err(|err| err.to_string())?;

        if oprom.len() > 256 {
            Err(format!("OPROM must be at most 256 bytes but is {}.", oprom.len()))
        } else if progrom.len() > 262144 {
            Err(format!("PROGROM must be at most 262144 bytes but is {}.", oprom.len()))
        } else if oprom.len() % 2 != 0 {
            Err(format!("OPROM length must be divisible by 2 but isn't."))
        } else if progrom.len() % 4 != 0 {
            Err(format!("PROGROM length must be divisible by 4 but isn't."))
        } else {
            let oprom: Vec<DoubleWord> = oprom.array_chunks::<2>().map(|[h, l]| (*h as DoubleWord) | (*l as DoubleWord) << 8).collect();
            let progrom: Vec<QuadWord> = progrom.array_chunks::<4>().map(|[a, b, c, d]| {
                (*a as QuadWord) <<  0 |
                (*b as QuadWord) <<  8 |
                (*c as QuadWord) << 16 |
                (*d as QuadWord) << 24
            }).collect();

            let mut oprom_arr = [0; 128];
            let mut progrom_arr = [0; 65536];
            for i in 0..oprom.len() {
                oprom_arr[i] = oprom[i];
            }
            for i in 0..progrom.len() {
                progrom_arr[i] = progrom[i];
            }

            Ok(Self {
                oprom: oprom_arr,
                progrom: progrom_arr,
                memory: [0; 65536],
                registers: [0; 8],
                pc: 0,
                halt: false,
            })
        }
    }

    pub fn read_register(&self, register: Register) -> Word {
        self.registers[register as usize]
    }

    pub fn write_register(&mut self, register: Register, data: Word) {
        match register {
            Register::RZ => { /* R0 is read only */ },
            _ => { self.registers[register as usize] = data; }
        }
    }

    pub fn tick(&mut self) {

        if self.halt {
            return;
        }

        //fetch
        let instruction = Instruction::from(self.progrom[self.pc as usize]);
        let control_signals = ControlFlags::from(self.oprom[instruction.opcode as usize]);

        // propagate
        let imm8 = u8_to_bools((instruction.immediate & 0xFF) as u8);
        let imm16 = u16_to_bools(instruction.immediate);
        let read0 = u8_to_bools(self.read_register(instruction.ro1));
        let read1 = u8_to_bools(self.read_register(instruction.ro2));
        let readcin = u8_to_bools(self.read_register(Register::RF))[2];
        let readadr = u16_to_bools(self.read_register(Register::RL) as u16 | (self.read_register(Register::RH) as u16) << 8);

        let operand_b = mux_word(read1, imm8, control_signals.opsel);
        let address = mux_doubleword(readadr, imm16, control_signals.adrsel);
        let carryin = and(readcin, control_signals.cryen);
        let aluop = [((control_signals.aluop as u8 & 1) != 0),
                     ((control_signals.aluop as u8 & 2) != 0),
                     ((control_signals.aluop as u8 & 4) != 0)];

        let (alu_result, alu_flags) = alu(aluop, read0, operand_b, carryin);
        let mem_result = u8_to_bools(if control_signals.ioren { self.memory[bools_to_u16(address) as usize] } else { 0 });
        let result = mux_word(alu_result, mem_result, control_signals.ressel);
        let pcssel = and(control_signals.pcssel, xor(control_signals.zinv, alu_flags[0]));
        let nextpc = mux_doubleword(u16_to_bools(self.pc + 1), address, pcssel);

        // write back registers/memory
        if control_signals.regwen {
            self.write_register(instruction.rd, bools_to_u8(result));
        }

        if control_signals.fwen {
            let flags = [alu_flags[0], alu_flags[1], alu_flags[2], alu_flags[3], false, false, false, false];
            self.write_register(Register::RF, bools_to_u8(flags));
        }

        if control_signals.iowen {
            self.memory[bools_to_u16(address) as usize] = bools_to_u8(read1);
        }

        if control_signals.halt {
            self.halt = true;
        } else {
            // finally, update PC
            self.pc = bools_to_u16(nextpc);
        }
    }

    pub fn dump_memory_to_bitmap(&self, path: &str) {
        let img: ImageBuffer<Luma<u8>, &[u8]> = ImageBuffer::from_raw(256, 256, &self.memory[..]).unwrap();
        img.save(path).unwrap();
    }

    pub fn dump_memory(&self, range: Range<usize>, path: &str) {
        std::fs::write(path, &self.memory[range]).unwrap();
    }
}

pub fn u8_to_bools(data: u8) -> [bool; 8] {
    (0..8).map(|i| (data & (1 << i)) != 0).collect::<Vec<bool>>().try_into().unwrap()
}

pub fn bools_to_u8(data: [bool; 8]) -> u8 {
    data.iter().enumerate().fold(0, |acc, (i, &b)| acc | ((b as u8) << i))
}

pub fn u16_to_bools(data: u16) -> [bool; 16] {
    (0..16).map(|i| (data & (1 << i)) != 0).collect::<Vec<bool>>().try_into().unwrap()
}

pub fn bools_to_u16(data: [bool; 16]) -> u16 {
    data.iter().enumerate().fold(0, |acc, (i, &b)| acc | ((b as u16) << i))
}
