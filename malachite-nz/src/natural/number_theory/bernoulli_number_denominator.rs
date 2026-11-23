// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::factorization::traits::Primes;
use malachite_base::num::modular::traits::DivisibleBy;

// The largest even index whose denominator is in `BERNOULLI_DENOMINATORS`.
const BERNOULLI_DENOMINATOR_MAX_SMALL: u64 = 178;

// The denominators of B_0, B_2, B_4, ..., B_178. This is __bernoulli_denom_small from
// arith/bernoulli_number_denom.c, FLINT 3.6.0.
const BERNOULLI_DENOMINATORS: [u32; 90] = [
    1, 6, 30, 42, 30, 66, 2730, 6, 510, 798, 330, 138, 2730, 6, 870, 14322, 510, 6, 1919190, 6,
    13530, 1806, 690, 282, 46410, 66, 1590, 798, 870, 354, 56786730, 6, 510, 64722, 30, 4686,
    140100870, 6, 30, 3318, 230010, 498, 3404310, 6, 61410, 272118, 1410, 6, 4501770, 6, 33330,
    4326, 1590, 642, 209191710, 1518, 1671270, 42, 1770, 6, 2328255930, 6, 30, 4357878, 510, 8646,
    4206930, 6, 4110, 274386, 679470, 6, 2381714790, 6, 4470, 2162622, 30, 138, 1794590070, 6,
    230010, 130074, 2490, 1002, 3404310, 66, 5190, 2478, 1043970, 1074,
];

impl Natural {
    /// Returns the denominator of the Bernoulli number $B_n$, in lowest terms.
    ///
    /// By the von Staudt–Clausen theorem, the denominator of $B_n$ for an even $n \geq 2$ is the
    /// product of the primes $p$ for which $p - 1$ divides $n$. The denominator of $B_0 = 1$ is 1,
    /// that of $B_1 = -1/2$ is 2, and that of $B_n = 0$ for an odd $n \geq 3$ is 1.
    ///
    /// $$
    /// f(n) = \prod_{\substack{p \text{ prime} \\ (p - 1) \mid n}} p.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n / \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::bernoulli_number_denominator(0), 1);
    /// assert_eq!(Natural::bernoulli_number_denominator(1), 2);
    /// assert_eq!(Natural::bernoulli_number_denominator(3), 1);
    /// assert_eq!(Natural::bernoulli_number_denominator(12), 2730);
    /// assert_eq!(
    ///     Natural::bernoulli_number_denominator(1000).to_string(),
    ///     "342999030"
    /// );
    /// ```
    ///
    /// This is arith_bernoulli_number_denom from arith/bernoulli_number_denom.c, FLINT 3.6.0.
    pub fn bernoulli_number_denominator(n: u64) -> Self {
        if n.odd() {
            Self::from(1 + u32::from(n == 1))
        } else if n <= BERNOULLI_DENOMINATOR_MAX_SMALL {
            Self::from(BERNOULLI_DENOMINATORS[usize::wrapping_from(n >> 1)])
        } else {
            // 2 and 3 divide every even n
            let mut den = const { Self::const_from(6) };
            for p in u64::primes_less_than_or_equal_to(&(n + 1)).skip(2) {
                if n.divisible_by(p - 1) {
                    den *= Self::from(p);
                }
            }
            den
        }
    }
}
