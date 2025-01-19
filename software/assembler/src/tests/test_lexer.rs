#[cfg(test)]
mod tests {
    use isa::arch::Opcode;

    use crate::lexer::{lex_operation, Token};


    #[test]
    pub fn test_empty_line() {
        assert_eq!(lex_operation(""), None);
        assert_eq!(lex_operation("ADDI"), Some((Token::Operation(Opcode::ADDI), 4)));
        assert_eq!(lex_operation("ADDI R0, R0, R0"), Some((Token::Operation(Opcode::ADDI), 4)));
    }

    #[test]
    pub fn test_basic_mnemonic() {
        assert_eq!(lex_operation("ADDI"), Some((Token::Operation(Opcode::ADDI), 4)));
        assert_eq!(lex_operation("ADDI R0, R0, R0"), Some((Token::Operation(Opcode::ADDI), 4)));
    }
}