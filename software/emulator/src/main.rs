use emulator::state::State;
use isa::arch::Instruction;

fn main() {
    let mut state = State::new("../assembler/assembly/microcode.bin", "../assembler/assembly/main.bin").unwrap();

    while !state.halt {
        let current_instruction = Instruction::from(state.progrom[state.pc as usize]);
        println!("executing: {:?}", current_instruction);
        state.tick();
    }
}
