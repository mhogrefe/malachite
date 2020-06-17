use num::BigUint;

pub fn num_partial_eq_unsigned<T>(x: &BigUint, u: T) -> bool
where
    BigUint: From<T>,
{
    *x == BigUint::from(u)
}
