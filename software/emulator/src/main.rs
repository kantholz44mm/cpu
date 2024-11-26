use std::time::Instant;
use emulator::state::State;
use isa::arch::{ADDRESS_RANGE, NUM_REGISTERS};

fn main() {

    let mut state = State::new();
    state.load_oprom("../assembler/assembly/microcode.bin").unwrap();
    state.load_program("../assembler/assembly/graphics.bin").unwrap();
    state.main_memory.fill(0);

    state.halted = false;

    let mut instructions_executed = 0;
    let start_timestamp = Instant::now();

    while !state.halted {
        const REGISTER_NAMES: [&'static str; NUM_REGISTERS] = [ "RZ", "R1", "R2", "R3", "R4", "RF", "RL", "RH" ];
        for i in 0..NUM_REGISTERS {
            print!("{}: {:02.X} ", REGISTER_NAMES[i], state.registers[i]);
        }
        println!(" PC: {:04.X}", state.program_counter);
        println!("instruction[{}]: {:?}", state.program_counter, state.program_memory[state.program_counter as usize]);
        state.tick();
        instructions_executed += 1;
    }

    let runtime = Instant::now() - start_timestamp;
    println!("halted after executing {} instructions.", instructions_executed);
    println!("halted at PC = {}", state.program_counter);
    println!("ran for {:?} @ ~{} instructions/second", runtime, instructions_executed as f32 / runtime.as_secs_f32());

    state.dump_ram_as_luma_bitmap(0..(ADDRESS_RANGE as usize), 256, "memory_dump.bmp");
}
