use crate::natural::Natural;
use crate::test_util::natural::logic::{natural_op_bits, natural_op_limbs};

pub fn natural_or_alt_1(x: &Natural, y: &Natural) -> Natural {
    natural_op_bits(&|a, b| a || b, x, y)
}

pub fn natural_or_alt_2(x: &Natural, y: &Natural) -> Natural {
    natural_op_limbs(&|a, b| a | b, x, y)
}
