use num::bigint::Sign;
use num::BigRational;
use std::cmp::Ordering;

pub fn num_sign(x: &BigRational) -> Ordering {
    match x.numer().sign() {
        Sign::NoSign => Ordering::Equal,
        Sign::Plus => Ordering::Greater,
        Sign::Minus => Ordering::Less,
    }
}
