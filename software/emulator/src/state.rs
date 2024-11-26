use std::ops::Range;
use std::{io::{self, Read}, ops::RangeBounds};

use image::{ImageBuffer, Luma};
use isa::arch::{ControlFlags, DoubleWord, FlagWriteMode, Instruction, Opcode, OperandSelect, QuadWord, Register, Word, ADDRESS_RANGE, NUM_OPCODES, NUM_REGISTERS};

use crate::combinatorics::{alu, bools_to_u8, opcode_to_bools, u8_to_bools};

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

        // fetch & "decode"
        let instruction = self.program_memory[self.program_counter as usize];
        let control_lines = self.oprom[instruction.opcode as usize];

        // propagate to register bank readout
        let read_0 = self.registers[instruction.ro1 as usize];
        let read_1 = self.registers[instruction.ro2 as usize];
        let read_flag = self.registers[Register::RF as usize];
        let read_addr = (self.registers[Register::RH as usize] as DoubleWord) << 8
                        | (self.registers[Register::RL as usize] as DoubleWord) << 0;

        let alu_operand_b = match instruction.operand {
            OperandSelect::OperandRegister => read_1,
            OperandSelect::OperandImmediate => instruction.immediate as u8,
        };

        let (alu_result, z, n, c, o) = alu(opcode_to_bools(instruction.opcode), u8_to_bools(read_0), u8_to_bools(alu_operand_b), (read_flag >> 2) & 1 != 0);

        let address = match instruction.operand {
            OperandSelect::OperandRegister => read_addr,
            OperandSelect::OperandImmediate => instruction.immediate,
        };

        if control_lines.regwen {
            if instruction.opcode.is_arithmetic() {
                if instruction.rd != Register::RZ {
                    self.registers[instruction.rd as usize] = bools_to_u8(alu_result);
                }
            } else {
                if instruction.rd != Register::RZ {
                    self.registers[instruction.rd as usize] = self.main_memory[address as usize];
                }
            }
        }

        if control_lines.adrwen {
            self.registers[Register::RL as usize] = ((instruction.immediate >> 0) & 0xFF) as Word;
            self.registers[Register::RH as usize] = ((instruction.immediate >> 8) & 0xFF) as Word;
        }

        if control_lines.iowen {
            self.main_memory[address as usize] = read_1;
        }

        if instruction.flags == FlagWriteMode::WriteFlags {
            self.registers[Register::RF as usize] = bools_to_u8([z, n, c, o, false, false, false, false]);
        }

        if instruction.opcode == Opcode::HCF {
            self.halted = true;
        }

        // increment pc
        let override_pc = control_lines.pcssel && ((read_1 == 0) != ((instruction.opcode as u8 & 1) == 1));
        let override_address = match instruction.operand {
            OperandSelect::OperandRegister => read_addr,
            OperandSelect::OperandImmediate => instruction.immediate,
        };

        if override_pc {
            self.program_counter = override_address;
        } else {
            self.program_counter += 1;
        }
    }

    pub fn dump_ram_as_luma_bitmap(&self, range: Range<usize>, width: usize, path: &str) {
        let num_bytes = range.end - range.start;
        let height = (num_bytes as f32 / width as f32).ceil() as usize;
        let mut luma_pixels = vec![0; width * height];

        luma_pixels[0..num_bytes].copy_from_slice(&self.main_memory[range]);
        let image = ImageBuffer::<Luma<u8>, _>::from_raw(width as u32, height as u32, luma_pixels).unwrap();
        image.save(path).unwrap();
    }
}