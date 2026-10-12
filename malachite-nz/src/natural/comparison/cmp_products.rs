// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::cmp::Ordering::{self, *};
use malachite_base::num::comparison::traits::OrdProducts;
use malachite_base::num::logic::traits::SignificantBits;

// The comparison of two products from the comparisons of their factors in one pairing, when those
// decide it: if both agree, the products compare the same way, and if one is an equality, the other
// decides.
const fn from_pairing(x: Ordering, y: Ordering) -> Option<Ordering> {
    match (x, y) {
        (Equal, o) | (o, Equal) => Some(o),
        (Less, Less) => Some(Less),
        (Greater, Greater) => Some(Greater),
        _ => None,
    }
}

impl OrdProducts for Natural {
    /// Compares the products of two pairs of [`Natural`]s, computing the products only as a last
    /// resort.
    ///
    /// $$
    /// f(a, b, c, d) = \operatorname{cmp}(ab, cd).
    /// $$
    ///
    /// The products are not computed when one of these decides the comparison:
    /// - a zero factor on either side;
    /// - the bit lengths: with $s$ the sum of the bit lengths of a pair, $2^{s - 2} \leq ab < 2^s$,
    ///   so a difference of at least 2 between the two sums decides;
    /// - the pairing of $a$ with $c$ and $b$ with $d$: if both comparisons agree, the products
    ///   compare the same way, and if one is an equality, the other decides;
    /// - the same for the pairing of $a$ with $d$ and $b$ with $c$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the largest number of significant
    /// bits of `a`, `b`, `c`, and `d`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::comparison::traits::OrdProducts;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = |x: u32| Natural::from(x);
    /// assert_eq!(Natural::cmp_products(&n(3), &n(7), &n(7), &n(3)), Equal);
    /// assert_eq!(Natural::cmp_products(&n(3), &n(7), &n(4), &n(5)), Greater);
    /// assert_eq!(Natural::cmp_products(&n(3), &n(7), &n(2), &n(11)), Less);
    /// ```
    ///
    /// This is equivalent to `compareProducts` from `AzNat/CompareProducts.lean`, Azurite.
    fn cmp_products(a: &Self, b: &Self, c: &Self, d: &Self) -> Ordering {
        let left_zero = *a == 0u32 || *b == 0u32;
        let right_zero = *c == 0u32 || *d == 0u32;
        if left_zero || right_zero {
            return right_zero.cmp(&left_zero);
        }
        let left_bits = a.significant_bits() + b.significant_bits();
        let right_bits = c.significant_bits() + d.significant_bits();
        if right_bits + 2 <= left_bits {
            return Greater;
        } else if left_bits + 2 <= right_bits {
            return Less;
        }
        if let Some(o) = from_pairing(a.cmp(c), b.cmp(d)) {
            return o;
        }
        if let Some(o) = from_pairing(a.cmp(d), b.cmp(c)) {
            return o;
        }
        (a * b).cmp(&(c * d))
    }
}
