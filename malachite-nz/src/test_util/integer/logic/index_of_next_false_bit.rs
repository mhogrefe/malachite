use integer::Integer;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitIterable, SignificantBits};

pub fn integer_index_of_next_false_bit_alt(n: &Integer, u: u64) -> Option<u64> {
    if u >= n.significant_bits() {
        if *n >= 0 {
            Some(u)
        } else {
            None
        }
    } else {
        for (i, bit) in n.bits().enumerate().skip(usize::exact_from(u)) {
            if !bit {
                return Some(u64::exact_from(i));
            }
        }
        None
    }
}
