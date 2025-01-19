#[cfg(test)]
mod tests {
    use isa::arch::{Instruction, Opcode, Register};
    use crate::{lexer::Token, parser::parse_line};


    #[test]
    pub fn test_parse_empty() {
        assert_eq!(parse_line(&[]), Ok(None));
    }

    #[test]
    pub fn test_parse_operation_without_operands() {
        let expected_instruction = Instruction {
            opcode: Opcode::HCF,
            rd: Register::RZ,
            ro1: Register::RZ,
            ro2: Register::RZ,
            immediate: 0,
        };
        assert_eq!(parse_line(&[Token::Operation(Opcode::HCF)]), Ok(Some(expected_instruction)));
    }

    #[test]
    pub fn test_parse_operation_with_three_register_operands() {
        let expected_instruction = Instruction {
            opcode: Opcode::ADD,
            rd: Register::R1,
            ro1: Register::R2,
            ro2: Register::R3,
            immediate: 0,
        };

        let tokens = [
            Token::Operation(Opcode::ADD),
            Token::Register(Register::R1),
            Token::Symbol(','),
            Token::Register(Register::R2),
            Token::Symbol(','),
            Token::Register(Register::R3),
        ];
        assert_eq!(parse_line(&tokens), Ok(Some(expected_instruction)));
    }
}