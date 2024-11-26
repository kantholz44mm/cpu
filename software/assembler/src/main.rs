use std::path::Path;
use isa::arch::Opcode;
use lexer::lex;
use parser::parse;
use preprocessor::preprocess;

mod lexer;
mod parser;
mod preprocessor;

fn assemble_microcode() -> Vec<u8> {
    let instructions: [Opcode; 16] = [
        Opcode::ADC,
        Opcode::SBB,
        Opcode::SHL,
        Opcode::SHR,
        Opcode::OR,
        Opcode::NOR,
        Opcode::XOR,
        Opcode::AND,
        Opcode::LW,
        Opcode::SW,
        Opcode::BZ,
        Opcode::BNZ,
        Opcode::RES0,
        Opcode::RES1,
        Opcode::LA,
        Opcode::HCF,
    ];
    instructions.iter().flat_map(|opcode| opcode.control_flags().encode().to_le_bytes()).collect()
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&String::from("--microcode")) {
        let assembly = assemble_microcode();
        std::fs::write("microcode.bin", &assembly).map_err(|err| err.to_string())?;

        println!("Assembled microcode, binary size: {} B", assembly.len());
        Ok(())
    } else if args.len() < 2 {
        Err(String::from("missing arguments."))
    } else {
        let input_file = Path::new(args.last().unwrap());
        let output_file = input_file.with_extension("bin");
        let input = preprocess(input_file)?;

        if args.contains(&String::from("--dump_preprocessed")) {
            let preprocessed_file = input_file.with_extension("preprocessed.s");
            std::fs::write(preprocessed_file, &input).map_err(|err| err.to_string())?;
        }

        let tokens = lex(&input)?;
        let instructions = parse(tokens)?;
        let assembly: Vec<u8> = instructions.iter().flat_map(|ins| ins.encode().to_le_bytes()).collect();
    
        std::fs::write(output_file, &assembly).map_err(|err| err.to_string())?;
    
        println!("Assembled {} instructions, binary size: {} B", instructions.len(), assembly.len());
        Ok(())
    }
}
