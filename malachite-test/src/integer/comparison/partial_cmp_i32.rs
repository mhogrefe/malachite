use num;
use std::cmp::Ordering;

pub fn num_partial_cmp_i32(x: &num::BigInt, i: i32) -> Option<Ordering> {
    x.partial_cmp(&num::BigInt::from(i))
}
