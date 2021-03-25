use crate::integer::logic::{integer_op_bits, integer_op_limbs};
use malachite_nz::integer::Integer;

pub fn integer_or_alt_1(x: &Integer, y: &Integer) -> Integer {
    integer_op_bits(&|a, b| a || b, x, y)
}

pub fn integer_or_alt_2(x: &Integer, y: &Integer) -> Integer {
    integer_op_limbs(&|a, b| a | b, x, y)
}
