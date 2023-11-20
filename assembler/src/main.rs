use crate::instructions::Instruction;
use crate::instructions::Register::*;

mod instructions;

fn main() {

    let ins = Instruction::MWI { dest: RP, datum: 55};
    println!("{:0x}", ins.encode());
}
