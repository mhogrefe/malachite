use crate::natural::arithmetic::div_round::double_cmp;
use crate::natural::exhaustive::exhaustive_natural_inclusive_range;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{DivExact, Factorial, Gcd, Parity};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use std::cmp::Ordering;

pub fn binomial_coefficient_naive_1(n: Natural, mut k: Natural) -> Natural {
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

pub fn binomial_coefficient_naive_2(n: Natural, mut k: Natural) -> Natural {
    if k > n {
        return Natural::ZERO;
    }
    if k == 0u32 || n == k {
        return Natural::ONE;
    }
    if double_cmp(&k, &n) == Ordering::Greater {
        k = &n - &k;
    }
    if k == 1u32 {
        n
    } else if k == 2u32 {
        (&n >> 1) * (if n.even() { n - Natural::ONE } else { n })
    } else {
        let mut product = n - &k + Natural::ONE;
        let mut numerator = product.clone();
        for i in exhaustive_natural_inclusive_range(Natural::TWO, k) {
            numerator += Natural::ONE;
            let gcd = (&numerator).gcd(&i);
            product /= i.div_exact(&gcd);
            product *= (&numerator).div_exact(gcd);
        }
        product
    }
}
