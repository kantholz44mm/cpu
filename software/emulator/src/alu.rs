use isa::arch::{Opcode, Word};

pub fn alu(op: Opcode, a: Word, b: Word, cin: u8) -> (Word, Word) {
    match op {
        Opcode::ADC | Opcode::LW    => {
            (0, 0)
        },
        Opcode::SBB | Opcode::SW    => {
            (0, 0)
        },
        Opcode::SHL | Opcode::BZ    => {
            (0, 0)
        },
        Opcode::SHR | Opcode::BNZ   => {
            (0, 0)
        },
        Opcode::OR | Opcode::RES0   => {
            (0, 0)
        },
        Opcode::NOR | Opcode::RES1  => {
            (0, 0)
        },
        Opcode::XOR | Opcode::LA    => {
            (0, 0)
        },
        Opcode::AND | Opcode::HCF   => {
            (0, 0)
        },
    }
}