// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;

impl PartialEq<Natural> for IntegerPolynomial {
    /// Determines whether an [`IntegerPolynomial`] is equal to a [`Natural`].
    ///
    /// The polynomial is equal to the [`Natural`] when it is the constant polynomial with that
    /// value, so the zero polynomial is equal to 0 and nothing else, no polynomial with a negative
    /// constant term is equal to any [`Natural`], and no polynomial of positive degree is equal to
    /// any [`Natural`].
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
    /// See [here](super::partial_eq_natural#partial_eq).
    fn eq(&self, other: &Natural) -> bool {
        match self.coefficients.as_slice() {
            [] => *other == 0u32,
            [c] => c == other,
            _ => false,
        }
    }
}

impl PartialEq<IntegerPolynomial> for Natural {
    /// Determines whether a [`Natural`] is equal to an [`IntegerPolynomial`].
    ///
    /// The [`Natural`] is equal to the polynomial when the polynomial is the constant polynomial
    /// with that value, so 0 is equal to the zero polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.coefficient(0).significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::partial_eq_natural#partial_eq).
    #[inline]
    fn eq(&self, other: &IntegerPolynomial) -> bool {
        other == self
    }
}
