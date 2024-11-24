use emulator::state::State;

fn main() {
    let mut state = State::new();
    state.load_oprom("../assembler/assembly/microcode.bin").unwrap();
    state.load_program("../assembler/assembly/main.bin").unwrap();
    state.main_memory.fill(0);

    state.halted = false;

    while !state.halted {
        state.tick();
        println!("executed tick");
    }
}
