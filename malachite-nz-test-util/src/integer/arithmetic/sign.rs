use num::bigint::Sign;
use num::BigInt;
use std::cmp::Ordering;

pub fn num_sign(x: &BigInt) -> Ordering {
    match x.sign() {
        Sign::NoSign => Ordering::Equal,
        Sign::Plus => Ordering::Greater,
        Sign::Minus => Ordering::Less,
    }
}
