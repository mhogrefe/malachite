use crate::integer::Integer;
use crate::test_util::integer::logic::{integer_op_bits, integer_op_limbs};

pub fn integer_xor_alt_1(x: &Integer, y: &Integer) -> Integer {
    integer_op_bits(&|a, b| a ^ b, x, y)
}

pub fn integer_xor_alt_2(x: &Integer, y: &Integer) -> Integer {
    integer_op_limbs(&|a, b| a ^ b, x, y)
}
