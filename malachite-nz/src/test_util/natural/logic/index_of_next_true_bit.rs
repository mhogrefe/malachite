use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::BitIterable;
use crate::natural::Natural;

pub fn natural_index_of_next_true_bit_alt(n: &Natural, u: u64) -> Option<u64> {
    for (i, bit) in n.bits().enumerate().skip(usize::exact_from(u)) {
        if bit {
            return Some(u64::wrapping_from(i));
        }
    }
    None
}
