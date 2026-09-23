// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural_polynomial::NaturalPolynomial;

impl PartialEq<Integer> for NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] is equal to an [`Integer`].
    ///
    /// The polynomial is equal to the [`Integer`] when it is the constant polynomial with that
    /// value, so the zero polynomial is equal to 0 and nothing else, no polynomial is equal to a
    /// negative [`Integer`], and no polynomial of positive degree is equal to any [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.coefficient(0).significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::partial_eq_integer#partial_eq).
    fn eq(&self, other: &Integer) -> bool {
        match self.coefficients.as_slice() {
            [] => *other == 0u32,
            [c] => other == c,
            _ => false,
        }
    }
}

impl PartialEq<NaturalPolynomial> for Integer {
    /// Determines whether an [`Integer`] is equal to a [`NaturalPolynomial`].
    ///
    /// The [`Integer`] is equal to the polynomial when the polynomial is the constant polynomial
    /// with that value, so 0 is equal to the zero polynomial and a negative [`Integer`] is equal to
    /// no polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.coefficient(0).significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::partial_eq_integer#partial_eq).
    #[inline]
    fn eq(&self, other: &NaturalPolynomial) -> bool {
        other == self
    }
}
