use num;
use std::cmp::Ordering;

pub fn num_partial_cmp_u32(x: &num::BigUint, u: u32) -> Option<Ordering> {
    x.partial_cmp(&num::BigUint::from(u))
}
