// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use crate::rational_polynomial::RationalPolynomial;

impl PartialEq<GaussianRational> for RationalPolynomial {
    /// Determines whether a [`RationalPolynomial`] is equal to a [`GaussianRational`].
    ///
    /// The polynomial is equal to the [`GaussianRational`] when it is the constant polynomial with
    /// that value, so the zero polynomial is equal to 0 and nothing else, no polynomial of positive
    /// degree is equal to any [`GaussianRational`], and no polynomial is equal to a
    /// [`GaussianRational`] with a nonzero imaginary part.
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
    /// See [here](super::partial_eq_gaussian_rational#partial_eq).
    fn eq(&self, other: &GaussianRational) -> bool {
        other.imaginary == 0u32 && *self == other.real
    }
}

impl PartialEq<RationalPolynomial> for GaussianRational {
    /// Determines whether a [`GaussianRational`] is equal to a [`RationalPolynomial`].
    ///
    /// The [`GaussianRational`] is equal to the polynomial when the polynomial is the constant
    /// polynomial with that value, so 0 is equal to the zero polynomial, and a [`GaussianRational`]
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
    /// See [here](super::partial_eq_gaussian_rational#partial_eq).
    #[inline]
    fn eq(&self, other: &RationalPolynomial) -> bool {
        other == self
    }
}
