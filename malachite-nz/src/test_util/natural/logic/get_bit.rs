use malachite_base::num::conversion::traits::ExactFrom;
use num::{BigUint, One, Zero};

pub fn num_get_bit(x: &BigUint, index: u64) -> bool {
    x & (BigUint::one() << usize::exact_from(index)) != BigUint::zero()
}
