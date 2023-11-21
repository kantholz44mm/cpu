use core::panic;
use std::fs::File;
use std::io::Write;
use std::io::stdin;

use instructions::ControlLines;
use instructions::DoubleWord;

use crate::instructions::Instruction;
use crate::instructions::Register::*;
use crate::instructions::Token;

mod instructions;
mod lexer;

fn main() {

    let mut outfile = File::create("compiled_binary.arx").unwrap();
    let tokens: Vec<Token> = "MWI r0, 55".split([',', '\n', '\t', '\r', ' ']).filter(|t| !t.is_empty()).map(|t| Token::parse(t).expect(format!("discovered invalid token: {}", t).as_str())).collect();
    println!("{:?}", tokens);

    println!("lexing {} tokens", tokens.len());
    if let Some(instruction) = Instruction::from_tokens(&tokens) {
        println!("instruction({}): {:08x?}", instruction.encode().to_le_bytes().len(), instruction);
        outfile.write_all(&instruction.encode().to_le_bytes()).unwrap();
    }
}
