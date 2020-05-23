use num::{BigInt, Integer, Zero};

pub fn num_divisible_by(x: &BigInt, y: &BigInt) -> bool {
    *x == BigInt::zero() || *y != BigInt::zero() && x.is_multiple_of(y)
}
