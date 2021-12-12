use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;

pub fn ceiling_log_base_power_of_2_naive_nz(x: &Natural, pow: u64) -> u64 {
    assert_ne!(*x, Natural::ZERO);
    assert_ne!(pow, 0);
    let mut result = 0;
    let mut p = Natural::ONE;
    while p < *x {
        result += 1;
        p <<= pow;
    }
    result
}
