use core::panic;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::stdin;

use instructions::ControlLines;
use instructions::DoubleWord;
use instructions::Opcode;
use strum::IntoEnumIterator;

use crate::instructions::Instruction;
use crate::instructions::Register::*;
use crate::instructions::Token;

mod instructions;
mod lexer;

type Line = Vec<Token>;

fn compile_microcode() -> Vec<u16> {
    let mut encoded = vec![];
    for opcode in Opcode::iter() {
        encoded.push(opcode.get_control_lines().encode().to_be());
    }
    return encoded;
}

fn main() {

    let mut microcode_file = File::create("microcode.bin").unwrap();
    for microcode in compile_microcode() {
        microcode_file.write(&microcode.to_le_bytes()).unwrap();
    }

    let mut outfile = File::create("compiled_binary.arx").unwrap();
    let mut infile = File::open("assembly.s").unwrap();
    let mut assembly_content = String::new();
    infile.read_to_string(&mut assembly_content).unwrap();
    let tokens: Vec<Token> = assembly_content.split([',', '\n', '\t', '\r', ' ']).filter(|t| !t.is_empty()).map(|t| Token::parse(t).expect(format!("discovered invalid token: {}", t).as_str())).collect();
    let mut current_token_index = 0;

    while let Some((num_tokens, instruction)) = Instruction::from_tokens(&tokens[current_token_index..]) {
        println!("instruction({}): {:08x?}", instruction.encode().to_le_bytes().len(), instruction);
        outfile.write_all(&instruction.encode().to_le_bytes()).unwrap();
        current_token_index += num_tokens;
    }

    if current_token_index < tokens.len() {
        println!("error: {}/{} here: {:?}", current_token_index, tokens.len(), tokens[current_token_index]);
    }
}
