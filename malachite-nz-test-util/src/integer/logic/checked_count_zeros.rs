use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::{BitIterable, CountZeros};
use malachite_nz::integer::Integer;

pub fn integer_checked_count_zeros_alt_1(n: &Integer) -> Option<u64> {
    if *n < 0 {
        Some(u64::wrapping_from(n.bits().filter(|&b| !b).count()))
    } else {
        None
    }
}

pub fn integer_checked_count_zeros_alt_2(n: &Integer) -> Option<u64> {
    if *n < 0 {
        Some(n.twos_complement_limbs().map(CountZeros::count_zeros).sum())
    } else {
        None
    }
}
