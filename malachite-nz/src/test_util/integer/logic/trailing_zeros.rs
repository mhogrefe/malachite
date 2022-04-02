use integer::Integer;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitIterable;

pub fn integer_trailing_zeros_alt(n: &Integer) -> Option<u64> {
    if *n == 0 {
        None
    } else {
        Some(u64::wrapping_from(n.bits().take_while(|&b| !b).count()))
    }
}
