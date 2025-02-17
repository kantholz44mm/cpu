
pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn xor(a: bool, b: bool) -> bool {
    nand(nand(a, nand(a, b)), nand(b, nand(a, b)))
}

pub fn and(a: bool, b: bool) -> bool {
    nand(nand(a, b), nand(a, b))
}

pub fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

pub fn not(a: bool) -> bool {
    nand(a, a)
}

pub fn nor(a: bool, b: bool) -> bool {
    not(or(a, b))
}

pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    nand(nand(sel, b), nand(a, nand(sel, sel)))
}

pub fn mux_word(a: [bool; 8], b: [bool; 8], sel: bool) -> [bool; 8] {
    [
        mux(a[0], b[0], sel),
        mux(a[1], b[1], sel),
        mux(a[2], b[2], sel),
        mux(a[3], b[3], sel),
        mux(a[4], b[4], sel),
        mux(a[5], b[5], sel),
        mux(a[6], b[6], sel),
        mux(a[7], b[7], sel),
    ]
}

pub fn mux_doubleword(a: [bool; 16], b: [bool; 16], sel: bool) -> [bool; 16] {
    [
        mux(a[0], b[0], sel),
        mux(a[1], b[1], sel),
        mux(a[2], b[2], sel),
        mux(a[3], b[3], sel),
        mux(a[4], b[4], sel),
        mux(a[5], b[5], sel),
        mux(a[6], b[6], sel),
        mux(a[7], b[7], sel),
        mux(a[8], b[8], sel),
        mux(a[9], b[9], sel),
        mux(a[10], b[10], sel),
        mux(a[11], b[11], sel),
        mux(a[12], b[12], sel),
        mux(a[13], b[13], sel),
        mux(a[14], b[14], sel),
        mux(a[15], b[15], sel),
    ]
}

pub fn decode_2(sel: [bool; 2]) -> [bool; 4] {
    [
        and(not(sel[0]), not(sel[1])),
        and(    sel[0] , not(sel[1])),
        and(not(sel[0]),     sel[1] ),
        and(    sel[0] ,     sel[1] ),
    ]
}

pub fn deselect_2(a: [bool; 4], b: [bool; 4]) -> bool {
    or(nand(nand(a[0], b[0]), nand(a[1], b[1])), nand(nand(a[2], b[2]), nand(a[3], b[3])))
}

pub fn full_adder(a: bool, b: bool, cin: bool) -> (bool, bool) {
    let cout = nand(nand(xor(a, b), cin), nand(a, b));
    let sum = xor(xor(a, b), cin);
    (sum, cout)
}

pub fn add_sub_word(cin: bool, sub: bool, a: [bool; 8], b: [bool; 8]) -> ([bool; 8], bool, bool) {

    let mut sum = [false; 8];
    let mut carry = [false; 9];

    carry[0] = xor(sub, cin);

    for i in 0..8 {
        (sum[i], carry[i + 1]) = full_adder(a[i], xor(sub, b[i]), carry[i]);
    }

    let cout = xor(sub, carry[8]);
    let overflow = xor(carry[7], carry[8]);

    (sum, cout, overflow)
}

pub fn or_and_xor_nand(a: bool, b: bool) -> [bool; 4] {
    [
        or  (a, b),
        and (a, b),
        xor (a, b),
        nand(a, b)
    ]
}

pub fn or_and_xor_nand_word(sel: [bool; 2], a: [bool; 8], b: [bool; 8]) -> [bool; 8] {
    [
        deselect_2(or_and_xor_nand(a[0], b[0]), decode_2(sel)),
        deselect_2(or_and_xor_nand(a[1], b[1]), decode_2(sel)),
        deselect_2(or_and_xor_nand(a[2], b[2]), decode_2(sel)),
        deselect_2(or_and_xor_nand(a[3], b[3]), decode_2(sel)),
        deselect_2(or_and_xor_nand(a[4], b[4]), decode_2(sel)),
        deselect_2(or_and_xor_nand(a[5], b[5]), decode_2(sel)),
        deselect_2(or_and_xor_nand(a[6], b[6]), decode_2(sel)),
        deselect_2(or_and_xor_nand(a[7], b[7]), decode_2(sel)),
    ]
}

pub fn shift_left_word(a: [bool; 8], by: [bool; 8], sib: bool) -> [bool; 8] {
    let a = mux_word(a, [sib, a[0], a[1], a[2], a[3], a[4], a[5], a[6]], by[0]);
    let a = mux_word(a, [sib,  sib, a[0], a[1], a[2], a[3], a[4], a[5]], by[1]);
    let a = mux_word(a, [sib,  sib,  sib,  sib, a[0], a[1], a[2], a[3]], by[2]);
    a
}

pub fn shift_right_word(a: [bool; 8], by: [bool; 8], sib: bool) -> [bool; 8] {
    let a = mux_word(a, [a[1], a[2], a[3], a[4], a[5], a[6], a[7], sib], by[0]);
    let a = mux_word(a, [a[2], a[3], a[4], a[5], a[6], a[7],  sib, sib], by[1]);
    let a = mux_word(a, [a[4], a[5], a[6], a[7],  sib,  sib,  sib, sib], by[2]);
    a
}

pub fn shift_word(a: [bool; 8], by: [bool; 8], dir: bool, sib: bool) -> ([bool; 8], bool) {
    (mux_word(shift_left_word(a, by, sib), shift_right_word(a, by, sib), dir), mux(a[7], a[0], dir))
}

pub fn alu(op: [bool; 3], a: [bool; 8], b: [bool; 8], cin: bool) -> ([bool; 8], [bool; 4]) {
    let (add_sub_result, add_sub_cout, add_sub_overflow) = add_sub_word(cin, op[0], a, b);
    let or_nor_xor_and_result = or_and_xor_nand_word(op[0..2].try_into().unwrap(), a, b);
    let (shift_result, shift_out) = shift_word(a, b, op[0], cin);

    let result = mux_word(mux_word(add_sub_result, shift_result, op[1]), or_nor_xor_and_result, op[2]);
    let zero = not(or(or(or(or(or(or(or(result[0], result[1]), result[2]), result[3]), result[4]), result[5]), result[6]), result[7]));
    let negative = result[7];
    let carry = mux(mux(add_sub_cout, shift_out, op[1]), false, op[2]);
    let overflow = mux(add_sub_overflow, false, or(op[1], op[2]));

    (result, [zero, negative, carry, overflow])
}
