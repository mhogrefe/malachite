use crate::integer::Integer;
use malachite_base::num::basic::traits::{One, Zero};

pub fn integer_product_naive<I: Iterator<Item = Integer>>(xs: I) -> Integer {
    let mut p = Integer::ONE;
    for x in xs {
        if x == 0 {
            return Integer::ZERO;
        }
        p *= x;
    }
    p
}
