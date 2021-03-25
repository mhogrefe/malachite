use crate::natural::logic::{natural_op_bits, natural_op_limbs};
use malachite_nz::natural::Natural;

pub fn natural_and_alt_1(x: &Natural, y: &Natural) -> Natural {
    natural_op_bits(&|a, b| a && b, x, y)
}

pub fn natural_and_alt_2(x: &Natural, y: &Natural) -> Natural {
    natural_op_limbs(&|a, b| a & b, x, y)
}
