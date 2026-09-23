// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;

impl PartialEq<Rational> for RationalPolynomial {
    /// Determines whether a [`RationalPolynomial`] is equal to a [`Rational`].
    ///
    /// The polynomial is equal to the [`Rational`] when it is the constant polynomial with that
    /// value, so the zero polynomial is equal to 0 and nothing else, and no polynomial of positive
    /// degree is equal to any [`Rational`].
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
    /// See [here](super::partial_eq_rational#partial_eq).
    fn eq(&self, other: &Rational) -> bool {
        match self.numerator.coefficients_asc() {
            [] => *other == 0u32,
            // Both are in lowest terms, so they are equal exactly when their signs, numerators and
            // denominators are.
            [c] => {
                other.sign == (*c > 0u32)
                    && self.denominator == other.denominator
                    && *c.unsigned_abs_ref() == other.numerator
            }
            _ => false,
        }
    }
}

impl PartialEq<RationalPolynomial> for Rational {
    /// Determines whether a [`Rational`] is equal to a [`RationalPolynomial`].
    ///
    /// The [`Rational`] is equal to the polynomial when the polynomial is the constant polynomial
    /// with that value, so 0 is equal to the zero polynomial.
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
    /// See [here](super::partial_eq_rational#partial_eq).
    #[inline]
    fn eq(&self, other: &RationalPolynomial) -> bool {
        other == self
    }
}
