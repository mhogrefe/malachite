use num;
use num::bigint::Sign;
use std::cmp::Ordering;

pub fn num_sign(x: &num::BigInt) -> Ordering {
    match x.sign() {
        Sign::NoSign => Ordering::Equal,
        Sign::Plus => Ordering::Greater,
        Sign::Minus => Ordering::Less,
    }
}
