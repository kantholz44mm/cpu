use emulator::state::State;
use isa::arch::NUM_OPCODES;

fn main() {
    let mut state = State::new();
    state.load_oprom("../assembler/assembly/microcode.bin").unwrap();
    state.load_program("../assembler/assembly/main.bin").unwrap();
    state.main_memory.fill(0);

}
