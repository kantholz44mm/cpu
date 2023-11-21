use crate::instructions::Instruction;
use crate::instructions::Register::*;
use crate::instructions::Token;

mod instructions;
mod lexer;

fn main() {

    let input = "something: MW r0 r1";
    let tokens: Vec<&str> = input.split([',', '\n', '\t', '\r', ' ']).collect();

    println!("lexing {} tokens", tokens.len());

    for token in tokens.iter() {
        if let Some(parsed) = Token::parse(token) {
            println!("parsed token: {:?}", parsed);
        } else {
            println!("invalid/unknown token: {}", token);
        }
    }
}
