use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::{BitIterable, CountOnes};
use malachite_nz::integer::Integer;

pub fn integer_checked_count_ones_alt_1(n: &Integer) -> Option<u64> {
    if *n >= 0 {
        Some(u64::wrapping_from(n.bits().filter(|&b| b).count()))
    } else {
        None
    }
}

pub fn integer_checked_count_ones_alt_2(n: &Integer) -> Option<u64> {
    if *n >= 0 {
        Some(n.twos_complement_limbs().map(CountOnes::count_ones).sum())
    } else {
        None
    }
}
