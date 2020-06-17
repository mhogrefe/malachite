use std::cmp::Ordering;

use num::BigUint;

pub fn num_partial_cmp_unsigned<T>(x: &BigUint, u: T) -> Option<Ordering>
where
    BigUint: From<T>,
{
    x.partial_cmp(&BigUint::from(u))
}
