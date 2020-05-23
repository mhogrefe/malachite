use num::BigInt;

pub fn num_partial_eq_primitive<T>(x: &BigInt, i: T) -> bool
where
    BigInt: From<T>,
{
    *x == BigInt::from(i)
}
