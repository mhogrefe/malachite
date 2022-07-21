use malachite_base::num::arithmetic::traits::SquareAssign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::BitIterable;
use crate::natural::Natural;

pub fn natural_pow_naive(n: &Natural, exp: u64) -> Natural {
    let mut result = Natural::ONE;
    for _ in 0..exp {
        result *= n;
    }
    result
}

pub fn natural_pow_simple_binary(n: &Natural, exp: u64) -> Natural {
    let mut result = Natural::ONE;
    for bit in exp.bits().rev() {
        result.square_assign();
        if bit {
            result *= n;
        }
    }
    result
}
