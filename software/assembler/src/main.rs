#![feature(variant_count)]

use std::path::Path;
use isa::arch::{DoubleWord, Opcode, QuadWord};
use lexer::lex;
use parser::parse;
use preprocessor::preprocess;
use strum::IntoEnumIterator;

mod lexer;
mod parser;
mod preprocessor;
mod expression;

fn assemble_microcode() -> Vec<u8> {
    Opcode::iter().flat_map(|opcode| DoubleWord::from(opcode.control_flags()).to_le_bytes()).collect()
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&String::from("--microcode")) {
        let assembly = assemble_microcode();
        std::fs::write("microcode.bin", &assembly).map_err(|err| err.to_string())?;

        println!("Assembled microcode for {} opcodes, binary size: {} B", std::mem::variant_count::<Opcode>(), assembly.len());
        Ok(())
    } else if args.len() < 2 {
        Err(String::from("missing arguments."))
    } else {
        let input_file = Path::new(args.last().unwrap());
        let output_file = input_file.with_extension("bin");
        let input = preprocess(input_file)?;

        if args.contains(&String::from("--preprocess")) {
            let preprocessed_file = input_file.with_extension("preprocessed.s");
            std::fs::write(preprocessed_file, &input).map_err(|err| err.to_string())?;
        }

        let tokens = lex(&input)?;
        let instructions = parse(tokens)?;
        let assembly: Vec<u8> = instructions.iter().flat_map(|ins| QuadWord::from(*ins).to_le_bytes()).collect();
    
        std::fs::write(output_file, &assembly).map_err(|err| err.to_string())?;
    
        println!("Assembled {} instructions, binary size: {} B", instructions.len(), assembly.len());
        Ok(())
    }
}
