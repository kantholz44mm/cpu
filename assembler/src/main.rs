use std::{fs::File, io::{self, Read, Write}};
use assembler::{arch::{compile_microcode, Instruction}, lexer::{lexer, Token}, parser::parse_line};

fn read_source_lines(filename: &str) -> Vec<String> {
    let mut source_code = String::new();
    io::stdin().read_to_string(&mut source_code).unwrap();

    source_code.split('\n')
               .map(|l| if let Some(comment_start) = l.find(';') {&l[..comment_start]} else {l})
               .filter(|l| !l.is_empty())
               .map(|l| l.to_string())
               .collect()
}

fn main() {
    let mut microcode_file = File::create("microcode.bin").unwrap();
    microcode_file.write_all(&compile_microcode()).unwrap();
    microcode_file.flush().unwrap();

    let mut instructions = Vec::new();
    
    for line in read_source_lines("assembly.rs") {
        let lexemes = line.split(&['\t', ' ']).filter(|lex| !lex.is_empty());
        let tokens: Vec<Option<Token>> = lexemes.map(|lex| lexer(lex)).collect();

        if tokens.iter().any(|token| token.is_none()) {
            panic!("Invalid token in line: {}", &line);
        }

        let tokens = tokens.iter().map(|token| token.unwrap()).collect();
        if let Some(instruction) = parse_line(tokens) {
            println!("{:?}", instruction);
            instructions.push(instruction);
        } else {
            panic!("Invalid instruction: {}", line);
        }
    }

    let mut program_file = File::create("program.bin").unwrap();
    for instruction in instructions {
        let bytes = instruction.encode().to_le_bytes();
        program_file.write(&bytes).unwrap();
    }
}