use malachite_base::num::conversion::traits::ExactFrom;
use num::{BigUint, One};

pub fn num_set_bit(x: &mut BigUint, index: u64) {
    *x = x.clone() | (BigUint::one() << usize::exact_from(index));
}
