// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_nz::gaussian_integer::GaussianInteger;

impl PartialEq<GaussianInteger> for RationalPolynomial {
    /// Determines whether a [`RationalPolynomial`] is equal to a [`GaussianInteger`].
    ///
    /// The polynomial is equal to the [`GaussianInteger`] when it is the constant polynomial with
    /// that value, so the zero polynomial is equal to 0 and nothing else, no polynomial with a
    /// non-integer coefficient, or of positive degree, is equal to any [`GaussianInteger`], and no
    /// polynomial is equal to a [`GaussianInteger`] with a nonzero imaginary part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.coefficient(0).significant_bits(), other.real.significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::partial_eq_gaussian_integer#partial_eq).
    fn eq(&self, other: &GaussianInteger) -> bool {
        self.denominator == 1u32 && self.numerator == *other
    }
}

impl PartialEq<RationalPolynomial> for GaussianInteger {
    /// Determines whether a [`GaussianInteger`] is equal to a [`RationalPolynomial`].
    ///
    /// The [`GaussianInteger`] is equal to the polynomial when the polynomial is the constant
    /// polynomial with that value, so 0 is equal to the zero polynomial, and a [`GaussianInteger`]
    /// with a nonzero imaginary part is equal to no polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.real.significant_bits(),
    /// other.coefficient(0).significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::partial_eq_gaussian_integer#partial_eq).
    #[inline]
    fn eq(&self, other: &RationalPolynomial) -> bool {
        other == self
    }
}
