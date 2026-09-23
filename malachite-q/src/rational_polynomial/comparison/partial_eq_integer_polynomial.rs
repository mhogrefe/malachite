// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_nz::integer_polynomial::IntegerPolynomial;

impl PartialEq<IntegerPolynomial> for RationalPolynomial {
    /// Determines whether a [`RationalPolynomial`] is equal to an [`IntegerPolynomial`].
    ///
    /// The two are equal when they have the same coefficients, so the zero polynomials are equal
    /// and a [`RationalPolynomial`] with a non-integer coefficient is equal to no
    /// [`IntegerPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two polynomials'
    /// total number of bits, summed over their coefficients. Polynomials of different degrees are
    /// compared in constant time.
    ///
    /// # Examples
    /// See [here](super::partial_eq_integer_polynomial#partial_eq).
    fn eq(&self, other: &IntegerPolynomial) -> bool {
        self.denominator == 1u32 && self.numerator == *other
    }
}

impl PartialEq<RationalPolynomial> for IntegerPolynomial {
    /// Determines whether an [`IntegerPolynomial`] is equal to a [`RationalPolynomial`].
    ///
    /// The two are equal when they have the same coefficients, so the zero polynomials are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two polynomials'
    /// total number of bits, summed over their coefficients. Polynomials of different degrees are
    /// compared in constant time.
    ///
    /// # Examples
    /// See [here](super::partial_eq_integer_polynomial#partial_eq).
    #[inline]
    fn eq(&self, other: &RationalPolynomial) -> bool {
        other == self
    }
}
