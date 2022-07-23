use crate::natural::Natural;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitIterable;

pub fn natural_trailing_zeros_alt(n: &Natural) -> Option<u64> {
    if *n == 0 {
        None
    } else {
        Some(u64::wrapping_from(n.bits().take_while(|&b| !b).count()))
    }
}
