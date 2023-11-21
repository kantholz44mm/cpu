use crate::instructions::Instruction;
use crate::instructions::Register::*;
use crate::instructions::Token;

mod instructions;
mod lexer;

fn main() {

    let input = "LW r0 r3:r5";
    let tokens: Vec<Token> = input.split([',', '\n', '\t', '\r', ' ']).map(|t| Token::parse(t).unwrap()).collect();
    println!("{:?}", tokens);

    println!("lexing {} tokens", tokens.len());
    println!("parsing line yields: {:?}", Instruction::from_tokens(&tokens));
}
