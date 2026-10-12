// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::comparison::traits::OrdProduct;
use malachite_base::num::logic::traits::SignificantBits;

impl OrdProduct for Natural {
    /// Compares the product of two [`Natural`]s with a third, computing the product only as a last
    /// resort.
    ///
    /// $$
    /// f(a, b, c) = \operatorname{cmp}(ab, c).
    /// $$
    ///
    /// The product is not computed when one of these decides the comparison:
    /// - a zero factor, or $c = 0$;
    /// - a factor of 1, which leaves a single comparison;
    /// - a factor that is at least $c$, since the other factor is then at least 2 and $ab \geq 2c$;
    /// - the bit lengths: with $s$ the sum of the bit lengths of $a$ and $b$, $2^{s - 2} \leq ab <
    ///   2^s$, so $ab > c$ if $c$ has at most $s - 2$ bits and $ab < c$ if it has at least $s + 1$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(a.significant_bits(),
    /// b.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::comparison::traits::OrdProduct;
    /// use malachite_nz::natural::Natural;
    ///
    /// let three = Natural::from(3u32);
    /// let seven = Natural::from(7u32);
    /// assert_eq!(
    ///     Natural::cmp_product(&three, &seven, &Natural::from(21u32)),
    ///     Equal
    /// );
    /// assert_eq!(
    ///     Natural::cmp_product(&three, &seven, &Natural::from(20u32)),
    ///     Greater
    /// );
    /// assert_eq!(
    ///     Natural::cmp_product(&three, &seven, &Natural::from(100u32)),
    ///     Less
    /// );
    /// ```
    ///
    /// This is equivalent to `compareProduct` from `AzNat/CompareProduct.lean`, Azurite.
    fn cmp_product(a: &Self, b: &Self, c: &Self) -> Ordering {
        match (a, b) {
            (&Self::ZERO, _) | (_, &Self::ZERO) => {
                if *c == 0u32 {
                    Equal
                } else {
                    Less
                }
            }
            _ if *c == 0u32 => Greater,
            (&Self::ONE, _) => b.cmp(c),
            (_, &Self::ONE) => a.cmp(c),
            _ if c <= a || c <= b => Greater,
            _ => {
                let s = a.significant_bits() + b.significant_bits();
                let c_bits = c.significant_bits();
                if c_bits + 2 <= s {
                    Greater
                } else if s < c_bits {
                    Less
                } else {
                    (a * b).cmp(c)
                }
            }
        }
    }
}
