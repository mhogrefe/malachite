// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::special::rising_factorial_polynomial::ogf_bsplit_final;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{AddMulAssign, DivExactAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::number_theory::traits::{
    BinomialCoefficient, Factorial, UnsignedStirlingNumberOfTheFirstKind,
};

// The second-order Eulerian numbers <<d, j>> for j < d, where d is positive, by the recurrence <<m,
// j>> = (j + 1)<<m - 1, j>> + (2m - 1 - j)<<m - 1, j - 1>>.
pub(crate) fn second_order_eulerian_numbers(d: u64) -> Vec<Natural> {
    let mut row = vec![Natural::ZERO; usize::exact_from(d)];
    row[0] = Natural::ONE;
    for m in 2..=d {
        for j in (1..usize::exact_from(m)).rev() {
            let uj = u64::exact_from(j);
            let carry = &row[j - 1] * Natural::from((m << 1) - 1 - uj);
            row[j] *= Natural::from(uj + 1);
            row[j] += carry;
        }
    }
    row
}

// sum_{i < len} w_i C(n + i, 2 len), where w = `weights`, with the binomial coefficients found from
// the largest down by exact division. The weights must be nonempty.
pub(crate) fn weighted_binomial_sum(n: u64, weights: &[Natural]) -> Natural {
    let len = weights.len();
    let r = u64::exact_from(len) << 1;
    let mut top = Natural::from(n) + Natural::from(len - 1);
    let mut binomial = Natural::binomial_coefficient(top.clone(), Natural::from(r));
    let mut sum = Natural::ZERO;
    for i in (0..len).rev() {
        if binomial == 0u32 {
            break;
        }
        sum.add_mul_assign(&weights[i], &binomial);
        if i != 0 {
            // C(t - 1, r) = C(t, r)(t - r)/t, and t >= r since C(t, r) is nonzero
            binomial *= &top - Natural::from(r);
            binomial.div_exact_assign(&top);
            top -= Natural::ONE;
        }
    }
    sum
}

// [n, n - d] = sum_{j < d} <<d, j>> C(n + j, 2d), for 0 < d < n.
fn stirling_1u_small_difference(n: u64, d: u64) -> Natural {
    weighted_binomial_sum(n, &second_order_eulerian_numbers(d))
}

// This is `arith_stirling_number_1u` from `arith/stirling1.c`, FLINT 3.6.0, except that the
// exponential generating function is replaced by the second-order Eulerian numbers. Their cost,
// about (n - k)^3 log(n - k), does not grow with n otherwise, and, measured in Malachite, they beat
// both ordinary generating functions when n - k <= min(0.3n, 3.3n^(2/3)); the second bound keeps
// their cost within O(n^2 log n) for large n. That covers every k > 0.87n for which FLINT uses the
// exponential generating function, which was slower than the reversed generating function
// everywhere it was measured.
crate_test_fn! {unsigned_stirling_number_of_the_first_kind(n: u64, k: u64) -> Natural {
    if k >= n || k == 0 {
        Natural::from(u32::from(n == k))
    } else if k == 1 {
        Natural::factorial(n - 1)
    } else if (n - k) as f64 <= (0.3 * n as f64).min(3.3 * libm::cbrt(n as f64 * n as f64)) {
        stirling_1u_small_difference(n, n - k)
    } else if k < n >> 1 {
        ogf_bsplit_final(1, n, usize::exact_from(k), false)
    } else {
        ogf_bsplit_final(1, n, usize::exact_from(n - k + 1), true)
    }
}}

crate_test_fn! {stirling_1u_by_small_difference(n: u64, k: u64) -> Natural {
    stirling_1u_small_difference(n, n - k)
}}

crate_test_fn! {stirling_1u_by_ogf(n: u64, k: u64) -> Natural {
    ogf_bsplit_final(1, n, usize::exact_from(k), false)
}}

crate_test_fn! {stirling_1u_by_reversed_ogf(n: u64, k: u64) -> Natural {
    ogf_bsplit_final(1, n, usize::exact_from(n - k + 1), true)
}}

impl UnsignedStirlingNumberOfTheFirstKind for Natural {
    /// Computes the unsigned Stirling number of the first kind $\left[{n \atop k}\right]$: the
    /// number of permutations of $n$ elements with $k$ cycles.
    ///
    /// $\left[{n \atop k}\right]$ is the coefficient of $x^k$ in the rising factorial $x(x + 1)
    /// \cdots (x + n - 1)$. When $n - k$ is small, it is found from the second-order Eulerian
    /// numbers, as $\sum_j \left\langle\!\left\langle {n - k \atop j} \right\rangle\!\right\rangle
    /// \binom{k + j}{2(n - k)}$, in time that hardly depends on $n$; otherwise only the needed part
    /// of the product $(1 + x)(2 + x) \cdots (n - 1 + x)$, or of $(1 + x)(1 + 2x) \cdots (1 + (n -
    /// 1)x)$, is expanded, by binary splitting.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::number_theory::traits::UnsignedStirlingNumberOfTheFirstKind;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::unsigned_stirling_number_of_the_first_kind(0, 0), 1);
    /// assert_eq!(Natural::unsigned_stirling_number_of_the_first_kind(5, 0), 0);
    /// assert_eq!(Natural::unsigned_stirling_number_of_the_first_kind(3, 5), 0);
    /// assert_eq!(
    ///     Natural::unsigned_stirling_number_of_the_first_kind(5, 2),
    ///     50
    /// );
    /// assert_eq!(
    ///     Natural::unsigned_stirling_number_of_the_first_kind(10, 5),
    ///     269325
    /// );
    /// assert_eq!(
    ///     Natural::unsigned_stirling_number_of_the_first_kind(42, 8).to_string(),
    ///     "34710540274511227194886186414162202385600068321280"
    /// );
    /// assert_eq!(
    ///     Natural::unsigned_stirling_number_of_the_first_kind(u64::MAX, u64::MAX - 2).to_string(),
    ///     "14474011154664524422192363201981531115617445066701938474182899447357142728705"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_stirling_number_1u` from `arith/stirling1.c`, FLINT 3.6.0,
    /// except that the second-order Eulerian numbers replace its exponential generating function.
    #[inline]
    fn unsigned_stirling_number_of_the_first_kind(n: u64, k: u64) -> Self {
        unsigned_stirling_number_of_the_first_kind(n, k)
    }
}
