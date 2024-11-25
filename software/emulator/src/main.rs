use emulator::{combinatorics::{self, bools_to_u8, mux_word, u8_to_bools}, state::State};
use isa::arch::Opcode;

fn main() {

    let mut state = State::new();
    state.load_oprom("../assembler/assembly/microcode.bin").unwrap();
    state.load_program("../assembler/assembly/main.bin").unwrap();
    state.main_memory.fill(0);

    state.halted = false;

    let mut instructions_executed = 0;

    while !state.halted {
        state.tick();
        instructions_executed += 1;
    }

    println!("halted after executing {} instructions.", instructions_executed);
    println!("halted at PC = {}", state.program_counter);

    for i in 0..32 {
        let upper_byte = state.main_memory[0xFF00 + i * 2 + 0];
        let lower_byte = state.main_memory[0xFF00 + i * 2 + 1];
        let num16 = lower_byte as u16 | ((upper_byte as u16) << 8);
        println!("{num16}");
    }
}
