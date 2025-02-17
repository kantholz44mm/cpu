#[cfg(test)]
mod tests {
    use isa::arch::Opcode;

    use crate::lexer::{lex, lex_number, lex_operation, Token};


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

    #[test]
    pub fn test_numbers() {
        assert_eq!(lex_number("0"), Some((Token::Number(0), 1)));
        assert_eq!(lex_number("5"), Some((Token::Number(5), 1)));
        assert_eq!(lex_number("52"), Some((Token::Number(52), 2)));
        assert_eq!(lex_number("-5"), Some((Token::Number(-5), 2)));
        assert_eq!(lex_number("+5"), Some((Token::Number( 5), 2)));
        assert_eq!(lex_number("-52"), Some((Token::Number(-52), 3)));
        assert_eq!(lex_number("+52"), Some((Token::Number( 52), 3)));

        assert_eq!(lex_number("0x0"), Some((Token::Number(0x0), 3)));
        assert_eq!(lex_number("0x5"), Some((Token::Number(0x5), 3)));
        assert_eq!(lex_number("0x52"), Some((Token::Number(0x52), 4)));
        assert_eq!(lex_number("-0x5"), Some((Token::Number(-0x5), 4)));
        assert_eq!(lex_number("+0x5"), Some((Token::Number( 0x5), 4)));
        assert_eq!(lex_number("-0x52"), Some((Token::Number(-0x52), 5)));
        assert_eq!(lex_number("+0x52"), Some((Token::Number( 0x52), 5)));

        assert_eq!(lex_number("0b0"), Some((Token::Number(0), 3)));
        assert_eq!(lex_number("0b1"), Some((Token::Number(1), 3)));
        assert_eq!(lex_number("0b10"), Some((Token::Number(2), 4)));
        assert_eq!(lex_number("-0b101"), Some((Token::Number(-5), 6)));
        assert_eq!(lex_number("+0b101"), Some((Token::Number( 5), 6)));
    }
    
    #[test]
    pub fn test_invalid_numbers() {
        assert_eq!(lex_number(""), None);
        assert_eq!(lex_number("a"), None);
        assert_eq!(lex_number("0b2"), None);
        assert_eq!(lex_number("0xG"), None);
    }
}
