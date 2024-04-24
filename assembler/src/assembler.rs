use crate::{arch::{Instruction, Operation}, parser::{Operand, Program, ProgramInstruction}};

#[derive(Debug)]
pub enum AssemblyError {
    InvalidOperandList(usize),
    UnknownLabel(usize, String),
}

/// given a program, verify constraints and assemble into a flat list
/// of instructions. Resolves labels, verifies operand types, etc.
pub fn assemble_program(mut program: Program) -> Result<Vec<Instruction>, AssemblyError> {

    // resolve label addresses
    for (i, instruction) in program.instructions.iter_mut().enumerate() {
        for operand in instruction.operands.iter_mut() {
            if let Operand::AddressLabel(label) = operand {
                if let Some(address) = program.labels.get(label) {
                    *operand = Operand::AddressImmediate(*address);
                } else {
                    return Err(AssemblyError::UnknownLabel(i, label.clone()));
                }
            }
        }
    }

    let mut instructions = Vec::new();
    // construct actual instruction encodings,
    // check operand lists as well.
    for (i, ProgramInstruction { condition, operation, operands }) in program.instructions.iter().enumerate() {
        let mut instruction = match (operation, operands.as_slice()) {
            (Operation::NOP, []) => Instruction { 
                ..Default::default()
            },
            (Operation::LW, [Operand::Register(rd), Operand::AddressRegister(ro1)]) => Instruction {
                dest: *rd, op1: *ro1, ..Default::default()
            },
            (Operation::LWI, [Operand::Register(rd), Operand::AddressImmediate(imm16)]) => Instruction {
                dest: *rd, imm: *imm16, ..Default::default()
            },
            (Operation::SW, [Operand::AddressRegister(ro1), Operand::Register(ro2)]) => Instruction {
                op1: *ro1, op2: *ro2, ..Default::default()
            },
            (Operation::SWI, [Operand::AddressImmediate(imm16), Operand::Register(ro2)]) => Instruction {
                imm: *imm16, op2: *ro2, ..Default::default()
            },
            (Operation::JP, [Operand::AddressRegister(ro1)]) => Instruction {
                op1: *ro1, ..Default::default()
            },
            (Operation::JPI, [Operand::AddressImmediate(imm16)]) => Instruction {
                imm: *imm16, ..Default::default()
            },
            (Operation::ALU(_), [Operand::Register(rd), Operand::Register(ro1), Operand::Register(ro2)]) => Instruction {
                dest: *rd, op1: *ro1, op2: *ro2, ..Default::default()
            },
            (Operation::ALUI(_), [Operand::Register(rd), Operand::Register(ro1), Operand::Immediate(imm8)]) => Instruction {
                dest: *rd, op1: *ro1, imm: *imm8, ..Default::default()
            },
            (Operation::ALUF(_), [Operand::Register(rd), Operand::Register(ro1), Operand::Register(ro2)]) => Instruction {
                dest: *rd, op1: *ro1, op2: *ro2, ..Default::default()
            },
            (Operation::ALUFI(_), [Operand::Register(rd), Operand::Register(ro1), Operand::Immediate(imm8)]) => Instruction {
                dest: *rd, op1: *ro1, imm: *imm8, ..Default::default()
            },
            (Operation::CMP(_), [Operand::Register(ro1), Operand::Register(ro2)]) => Instruction {
                op1: *ro1, op2: *ro2, ..Default::default()
            },
            (Operation::CMPI(_), [Operand::Register(ro1), Operand::Immediate(imm8)]) => Instruction {
                op1: *ro1, imm: *imm8, ..Default::default()
            },
            (Operation::WPR, [Operand::Register(ro2)]) => Instruction {
                op2: *ro2, ..Default::default()
            },
            (Operation::WPRI, [Operand::Immediate(imm8)]) => Instruction {
                imm: *imm8, ..Default::default()
            },
            (Operation::HCF, []) => Instruction { 
                ..Default::default()
            },
            _ => { return Err(AssemblyError::InvalidOperandList(i)) }
        };

        instruction.op = *operation;
        instruction.cond = *condition;
        instructions.push(instruction);
    }

    Ok(instructions)
}