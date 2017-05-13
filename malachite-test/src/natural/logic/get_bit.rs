use num::{BigUint, One, Zero};

pub fn num_get_bit(x: &BigUint, index: u64) -> bool {
    x & (BigUint::one() << index as usize) != BigUint::zero()
}
