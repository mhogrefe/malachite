use crate::natural::arithmetic::div_round::double_cmp;
use crate::natural::exhaustive::exhaustive_natural_inclusive_range;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::DivExact;
use malachite_base::num::arithmetic::traits::Factorial;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use std::cmp::Ordering;

pub fn binomial_coefficient_naive(n: Natural, mut k: Natural) -> Natural {
    if k > n {
        return Natural::ZERO;
    }
    if k == 0u32 || n == k {
        return Natural::ONE;
    }
    if double_cmp(&k, &n) == Ordering::Greater {
        k = &n - &k;
    }
    let k_u64 = u64::exact_from(&k);
    exhaustive_natural_inclusive_range(&n - k + Natural::ONE, n)
        .product::<Natural>()
        .div_exact(Natural::factorial(k_u64))
}
