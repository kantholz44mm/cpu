
#[cfg(test)]
mod tests {
    use isa::arch::ALUOperation;
    use crate::{combinatorics::{add_sub_word, alu, and, decode_2, deselect_2, full_adder, mux, mux_word, nand, nor, not, or, or_and_xor_nand_word, shift_left_word, shift_right_word, shift_word, xor}, state::{bools_to_u8, u8_to_bools}};

    pub fn aluopcode_to_bools(opcode: ALUOperation) -> [bool; 3] {
        u8_to_bools(opcode as u8)[0..3].try_into().unwrap()
    }

    #[test]
    pub fn test_bools_conversion() {
        for i in 0..=u8::MAX {
            assert_eq!(bools_to_u8(u8_to_bools(i)), i);
        }
    }

    #[test]
    pub fn test_nand() {
        assert_eq!(nand(false, false), true );
        assert_eq!(nand(false, true ), true );
        assert_eq!(nand(true,  false), true );
        assert_eq!(nand(true,  true ), false);
    }

    #[test]
    pub fn test_xor() {
        assert_eq!(xor(false, false), false);
        assert_eq!(xor(false, true ), true );
        assert_eq!(xor(true,  false), true );
        assert_eq!(xor(true,  true ), false);
    }

    #[test]
    pub fn test_not() {
        assert_eq!(not(false), true);
        assert_eq!(not(true), false);
    }

    #[test]
    pub fn test_and() {
        assert_eq!(and(false, false), false);
        assert_eq!(and(false, true ), false);
        assert_eq!(and(true,  false), false);
        assert_eq!(and(true,  true ), true );
    }

    #[test]
    pub fn test_or() {
        assert_eq!(or(false, false), false);
        assert_eq!(or(false, true ), true );
        assert_eq!(or(true,  false), true );
        assert_eq!(or(true,  true ), true );
    }

    #[test]
    pub fn test_nor() {
        assert_eq!(nor(false, false), true );
        assert_eq!(nor(false, true ), false);
        assert_eq!(nor(true,  false), false);
        assert_eq!(nor(true,  true ), false);
    }

    #[test]
    pub fn test_mux() {
        assert_eq!(mux(false, true , false), false);
        assert_eq!(mux(false, true , true ), true );
        assert_eq!(mux(true , false, true ), false);
        assert_eq!(mux(true , false, false), true );

        assert_eq!(mux_word(u8_to_bools(42), u8_to_bools(192), false), u8_to_bools(42 ));
        assert_eq!(mux_word(u8_to_bools(42), u8_to_bools(192), true ), u8_to_bools(192));
        assert_eq!(mux_word(u8_to_bools(192), u8_to_bools(42), true ), u8_to_bools(42 ));
        assert_eq!(mux_word(u8_to_bools(192), u8_to_bools(42), false), u8_to_bools(192));
    }

    #[test]
    pub fn test_decode_2() {
        assert_eq!(decode_2([false, false]), [true , false, false, false]);
        assert_eq!(decode_2([true , false]), [false, true , false, false]);
        assert_eq!(decode_2([false, true ]), [false, false, true , false]);
        assert_eq!(decode_2([true , true ]), [false, false, false, true ]);
    }

    #[test]
    pub fn test_deselect_2() {
        assert_eq!(deselect_2([false, true , false, true ], [false, false, false, false]), false);
        assert_eq!(deselect_2([false, true , false, true ], [true , false, false, false]), false);
        assert_eq!(deselect_2([false, true , false, true ], [false, true , false, false]), true );
        assert_eq!(deselect_2([false, true , false, true ], [false, false, true , false]), false);
        assert_eq!(deselect_2([false, true , false, true ], [false, false, false, true ]), true );
    }

    #[test]
    pub fn test_or_nor_xor_and() {
        let a = 0b11001100;
        let b = 0b10101010;
        assert_eq!(or_and_xor_nand_word([false, false], u8_to_bools(a), u8_to_bools(b)), u8_to_bools(a | b));
        assert_eq!(or_and_xor_nand_word([true , false], u8_to_bools(a), u8_to_bools(b)), u8_to_bools(a & b));
        assert_eq!(or_and_xor_nand_word([false, true ], u8_to_bools(a), u8_to_bools(b)), u8_to_bools(a ^ b));
        assert_eq!(or_and_xor_nand_word([true , true ], u8_to_bools(a), u8_to_bools(b)), u8_to_bools(!(a & b)));
    }

    #[test]
    pub fn test_shifting() {
        let a = 0b10101010;
        for i in 0..8 {
            assert_eq!(shift_left_word(u8_to_bools(a), u8_to_bools(i), false), u8_to_bools(a << i));
            assert_eq!(shift_right_word(u8_to_bools(a), u8_to_bools(i), false), u8_to_bools(a >> i));

            assert_eq!(shift_word(u8_to_bools(a), u8_to_bools(i), false, false), (u8_to_bools(a << i), true ));
            assert_eq!(shift_word(u8_to_bools(a), u8_to_bools(i), true, false), (u8_to_bools(a >> i), false));
        }
    }

    #[test]
    pub fn test_full_adder() {
        assert_eq!(full_adder(false, false, false), (false, false));
        assert_eq!(full_adder(false, false, true ), (true , false));
        assert_eq!(full_adder(false, true , false), (true , false));
        assert_eq!(full_adder(false, true , true ), (false, true ));
        assert_eq!(full_adder(true , false, false), (true , false));
        assert_eq!(full_adder(true , false, true ), (false, true ));
        assert_eq!(full_adder(true , true , false), (false, true ));
        assert_eq!(full_adder(true , true , true ), (true , true ));
    }

    #[test]
    pub fn test_addition() {
        // unsigned
        assert_eq!(add_sub_word(false, false, u8_to_bools(0)  , u8_to_bools(0))  , (u8_to_bools(0)  , false, false)); // 0 + 0, no carry
        assert_eq!(add_sub_word(true , false, u8_to_bools(0)  , u8_to_bools(0))  , (u8_to_bools(1)  , false, false)); // 0 + 0, with carry
        assert_eq!(add_sub_word(false, false, u8_to_bools(255), u8_to_bools(0))  , (u8_to_bools(255), false, false)); // 255 + 0, no carry
        assert_eq!(add_sub_word(true , false, u8_to_bools(255), u8_to_bools(0))  , (u8_to_bools(0)  , true , false)); // 255 + 0, with carry
        assert_eq!(add_sub_word(false, false, u8_to_bools(192), u8_to_bools(168)), (u8_to_bools(104), true , true )); // 192 + 168, no carry
        assert_eq!(add_sub_word(true , false, u8_to_bools(192), u8_to_bools(168)), (u8_to_bools(105), true , true )); // 192 + 168, with carry
        assert_eq!(add_sub_word(false, false, u8_to_bools(127), u8_to_bools(0))  , (u8_to_bools(127), false, false)); // 127 + 0, no carry
        assert_eq!(add_sub_word(true , false, u8_to_bools(127), u8_to_bools(0))  , (u8_to_bools(128), false, true )); // 127 + 0, with carry
        assert_eq!(add_sub_word(false, false, u8_to_bools(42) , u8_to_bools(39)) , (u8_to_bools(81) , false, false)); // 42 + 39, no carry
        assert_eq!(add_sub_word(true , false, u8_to_bools(42) , u8_to_bools(39)) , (u8_to_bools(82) , false, false)); // 42 + 39, with carry

        //signed
        assert_eq!(add_sub_word(false, false, u8_to_bools(52) , u8_to_bools(-14 as i8 as u8)) , (u8_to_bools(38) , true , false)); // 52 + -14, no carry
        assert_eq!(add_sub_word(true , false, u8_to_bools(52) , u8_to_bools(-14 as i8 as u8)) , (u8_to_bools(39) , true , false)); // 52 + -14, with carry
        assert_eq!(add_sub_word(false, false, u8_to_bools(-30 as i8 as u8), u8_to_bools(-120 as i8 as u8)), (u8_to_bools(106), true , true)); // -30 + -120, no carry
        assert_eq!(add_sub_word(true , false, u8_to_bools(-30 as i8 as u8), u8_to_bools(-120 as i8 as u8)), (u8_to_bools(107), true , true)); // -30 + -120, with carry
    }

    #[test]
    pub fn test_subtraction() {
        // unsigned
        assert_eq!(add_sub_word(false, true, u8_to_bools(0)  , u8_to_bools(0))  , (u8_to_bools(0)  , false, false)); // 0 - 0, no borrow
        assert_eq!(add_sub_word(true , true, u8_to_bools(0)  , u8_to_bools(0))  , (u8_to_bools(255), true , false)); // 0 - 0, with borrow
        assert_eq!(add_sub_word(false, true, u8_to_bools(255), u8_to_bools(0))  , (u8_to_bools(255), false, false)); // 255 - 0, no borrow
        assert_eq!(add_sub_word(true , true, u8_to_bools(255), u8_to_bools(0))  , (u8_to_bools(254), false, false)); // 255 - 0, with borrow
        assert_eq!(add_sub_word(false, true, u8_to_bools(192), u8_to_bools(168)), (u8_to_bools(24) , false, false)); // 192 - 168, no borrow
        assert_eq!(add_sub_word(true , true, u8_to_bools(192), u8_to_bools(168)), (u8_to_bools(23) , false, false)); // 192 - 168, with borrow
        assert_eq!(add_sub_word(false, true, u8_to_bools(127), u8_to_bools(0))  , (u8_to_bools(127), false, false)); // 127 - 0, no borrow
        assert_eq!(add_sub_word(true , true, u8_to_bools(127), u8_to_bools(0))  , (u8_to_bools(126), false, false)); // 127 - 0, with borrow
        assert_eq!(add_sub_word(false, true, u8_to_bools(42) , u8_to_bools(39)) , (u8_to_bools(3)  , false, false)); // 42 - 39, no borrow
        assert_eq!(add_sub_word(true , true, u8_to_bools(42) , u8_to_bools(39)) , (u8_to_bools(2)  , false, false)); // 42 - 39, with borrow

        //signed
        assert_eq!(add_sub_word(false, true, u8_to_bools(52) , u8_to_bools(-14 as i8 as u8)), (u8_to_bools(66) , true , false)); // 52 - -14, no borrow
        assert_eq!(add_sub_word(true , true, u8_to_bools(52) , u8_to_bools(-14 as i8 as u8)), (u8_to_bools(65) , true , false)); // 52 - -14, with borrow
        assert_eq!(add_sub_word(false, true, u8_to_bools(-30 as i8 as u8), u8_to_bools(120)), (u8_to_bools(106), false, true )); // -30 - 120, no borrow
        assert_eq!(add_sub_word(true , true, u8_to_bools(-30 as i8 as u8), u8_to_bools(120)), (u8_to_bools(105), false, true )); // -30 - 120, with borrow
    }

    #[test]
    pub fn test_alu() {
        // addition and subtraction
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::Add), u8_to_bools(240), u8_to_bools(40), false), (u8_to_bools(24), [false, false, true, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::Add), u8_to_bools(52) , u8_to_bools(-62 as i8 as u8), false), (u8_to_bools(-10 as i8 as u8), [false, true, false, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::Subtract), u8_to_bools(52) , u8_to_bools(104), false), (u8_to_bools(-52 as i8 as u8), [false, true, true, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::Subtract), u8_to_bools(0)  , u8_to_bools(150), false), (u8_to_bools(106), [false, false, true, false]));

        // shifting
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::ShiftLeft), u8_to_bools(52) , u8_to_bools(1), false), (u8_to_bools(104), [false, false, false, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::ShiftLeft), u8_to_bools(255), u8_to_bools(1), false), (u8_to_bools(254), [false, true , true , false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::ShiftRight), u8_to_bools(52) , u8_to_bools(1), false), (u8_to_bools(26) , [false, false, false, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::ShiftRight), u8_to_bools(255), u8_to_bools(1), false), (u8_to_bools(127), [false, false, true , false]));

        assert_eq!(alu(aluopcode_to_bools(ALUOperation::ShiftLeft), u8_to_bools(52) , u8_to_bools(3), false), (u8_to_bools(160), [false, true , false, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::ShiftLeft), u8_to_bools(255), u8_to_bools(3), false), (u8_to_bools(248), [false, true , true , false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::ShiftRight), u8_to_bools(52) , u8_to_bools(3), false), (u8_to_bools(6)  , [false, false, false, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::ShiftRight), u8_to_bools(255), u8_to_bools(3), false), (u8_to_bools(31) , [false, false, true , false]));

        // bitwise operations
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::Or),  u8_to_bools(0b01010101) , u8_to_bools(0b10101010), false), (u8_to_bools(255), [false, true , false, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::Or),  u8_to_bools(0b00000000) , u8_to_bools(0b00000000), false), (u8_to_bools(0),   [true , false, false, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::And), u8_to_bools(0b01010101) , u8_to_bools(0b10101010), false), (u8_to_bools(0),   [true , false, false, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::And), u8_to_bools(0b10101010) , u8_to_bools(0b10101010), false), (u8_to_bools(170), [false, true , false, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::Xor), u8_to_bools(0b01010101) , u8_to_bools(0b10101010), false), (u8_to_bools(255), [false, true , false, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::Xor), u8_to_bools(0b00000000) , u8_to_bools(0b11111111), false), (u8_to_bools(255), [false, true , false, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::Nand),u8_to_bools(0b01010101) , u8_to_bools(0b10101010), false), (u8_to_bools(255), [false, true , false, false]));
        assert_eq!(alu(aluopcode_to_bools(ALUOperation::Nand),u8_to_bools(0b00000000) , u8_to_bools(0b00000000), false), (u8_to_bools(255), [false, true , false, false]));
    }
}