// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;

impl RationalVector {
    /// Builds a [`RationalVector`] from a vector of [`Integer`](malachite_nz::integer::Integer)
    /// numerators and a single [`Natural`] denominator, dividing each numerator by it.
    ///
    /// Each element is reduced to lowest terms, so the numerators and denominator need not be. This
    /// is the inverse of
    /// [`to_numerators_and_denominator`](RationalVector::to_numerators_and_denominator).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators plus the dimension times the number of bits of the denominator.
    ///
    /// # Panics
    /// Panics if `denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_vector::IntegerVector;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let ns = IntegerVector::from_str("(3, -4, 30)").unwrap();
    /// assert_eq!(
    ///     RationalVector::from_numerators_and_denominator(&ns, &Natural::from(6u32)).to_string(),
    ///     "(1/2, -2/3, 5)"
    /// );
    /// ```
    pub fn from_numerators_and_denominator(
        numerators: &IntegerVector,
        denominator: &Natural,
    ) -> Self {
        assert_ne!(*denominator, 0u32, "the denominator is zero");
        Self {
            elements: numerators
                .elements
                .iter()
                .map(|n| {
                    Rational::from_sign_and_naturals_ref(
                        *n >= 0u32,
                        n.unsigned_abs_ref(),
                        denominator,
                    )
                })
                .collect(),
        }
    }
}
