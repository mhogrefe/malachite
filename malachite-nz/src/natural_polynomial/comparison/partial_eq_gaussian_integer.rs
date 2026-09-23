// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::natural_polynomial::NaturalPolynomial;

impl PartialEq<GaussianInteger> for NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] is equal to a [`GaussianInteger`].
    ///
    /// The polynomial is equal to the [`GaussianInteger`] when it is the constant polynomial with
    /// that value, so the zero polynomial is equal to 0 and nothing else, no polynomial is equal to
    /// a [`GaussianInteger`] with a nonzero imaginary part or a negative real part, and no
    /// polynomial of positive degree is equal to any [`GaussianInteger`].
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
        match self.coefficients.as_slice() {
            [] => *other == 0u32,
            [c] => other == c,
            _ => false,
        }
    }
}

impl PartialEq<NaturalPolynomial> for GaussianInteger {
    /// Determines whether a [`GaussianInteger`] is equal to a [`NaturalPolynomial`].
    ///
    /// The [`GaussianInteger`] is equal to the polynomial when the polynomial is the constant
    /// polynomial with that value, so 0 is equal to the zero polynomial, and a [`GaussianInteger`]
    /// with a nonzero imaginary part or a negative real part is equal to no polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.real.significant_bits(), other.coefficient(0).significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::partial_eq_gaussian_integer#partial_eq).
    #[inline]
    fn eq(&self, other: &NaturalPolynomial) -> bool {
        other == self
    }
}
