use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitIterable, CountOnes};
use crate::natural::Natural;

pub fn natural_count_ones_alt_1(n: &Natural) -> u64 {
    u64::exact_from(n.bits().filter(|&b| b).count())
}

pub fn natural_count_ones_alt_2(n: &Natural) -> u64 {
    n.limbs().map(CountOnes::count_ones).sum()
}
