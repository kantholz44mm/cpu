use core::panic;
use std::{env, fs::File, io::{self, Read, Write}, os};
use archas::{arch::compile_microcode, assembler::assemble_program, lexer::lex_program, parser::parse_program};

fn assemble_microcode(filename: &str) {
    let mut file = File::create(filename).unwrap();
    file.write_all(&compile_microcode()).unwrap();
}

fn assemble_sourcecode(inputfile: &str, outputfile: &str) {
    let mut source_code = String::new();
    let mut file = File::open(inputfile).unwrap();
    file.read_to_string(&mut source_code).unwrap();

    // lexical analysis
    let tokens = match lex_program(&source_code) {
        Ok(tokens) => tokens,
        Err(after) => panic!("Lexical error at position: {}:\n{}", after, &source_code[after..]),
    };

    // parsing
    let program = match parse_program(tokens) {
        Ok(program) => program,
        Err(error_line) => panic!("Parse error in line: {}", error_line + 1),
    };

    // assembling
    let instructions = match assemble_program(program) {
        Ok(ins) => ins,
        Err(error) => panic!("Assembly error: {:?}", error),
    };

    // dump into binary file
    let mut program_file = File::create(outputfile).unwrap();
    for instruction in instructions {
        let bytes = instruction.encode().to_le_bytes();
        program_file.write(&bytes).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("--assemble-microcode")) {
        assemble_microcode("microcode.bin");
    } else {
        let (source, target): (&str, &str) = match args.as_slice() {
            [_, source] => (source, "a.out"),
            [_, source, target] => (source, target),
            _ => panic!("Usage: archas --assemble-microcode\n       archas <inputfile> [outputfile]")
        };
        assemble_sourcecode(source, target);
    }
}