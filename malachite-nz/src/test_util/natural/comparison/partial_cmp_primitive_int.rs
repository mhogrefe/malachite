use num::BigUint;
use std::cmp::Ordering;

pub fn num_partial_cmp_unsigned<T>(x: &BigUint, u: T) -> Option<Ordering>
where
    BigUint: From<T>,
{
    x.partial_cmp(&BigUint::from(u))
}
