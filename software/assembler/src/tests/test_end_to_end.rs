#[cfg(test)]
mod tests {
    use isa::arch::QuadWord;

    use crate::{lexer::lex, parser::parse};

    #[test]
    pub fn test_assemble_no_instructions() {
        let program_text = "";
        let tokens = lex(&program_text).unwrap();
        let instructions = parse(tokens).unwrap();
        let assembly: Vec<u8> = instructions.iter().flat_map(|ins| QuadWord::from(*ins).to_le_bytes()).collect();
        assert!(assembly.len() == 0);
    }

    #[test]
    pub fn test_assemble_single_nop() {
        let program_text = "NOP\n";
        let tokens = lex(&program_text).unwrap();
        let instructions = parse(tokens).unwrap();
        let assembly: Vec<u8> = instructions.iter().flat_map(|ins| QuadWord::from(*ins).to_le_bytes()).collect();
        assert_eq!(assembly, vec![0, 0, 0, 0]);
    }
}