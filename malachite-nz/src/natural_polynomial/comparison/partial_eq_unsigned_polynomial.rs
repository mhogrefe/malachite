// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

impl<T: PrimitiveUnsigned> PartialEq<UnsignedPolynomial<T>> for NaturalPolynomial
where
    Natural: PartialEq<T>,
{
    /// Determines whether a [`NaturalPolynomial`] is equal to an [`UnsignedPolynomial`].
    ///
    /// The two are equal when they have the same coefficients, which, since neither stores trailing
    /// zeros, means the same number of coefficients and equal coefficients in each position. So the
    /// zero polynomials are equal, and a [`NaturalPolynomial`] with a coefficient too large for `T`
    /// is equal to no [`UnsignedPolynomial<T>`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    /// Polynomials of different degrees are compared in constant time.
    ///
    /// # Examples
    /// See [here](super::partial_eq_unsigned_polynomial#partial_eq).
    fn eq(&self, other: &UnsignedPolynomial<T>) -> bool {
        let other = other.coefficients_asc();
        self.coefficients.len() == other.len()
            && self.coefficients.iter().zip(other).all(|(x, y)| x == y)
    }
}

impl<T: PrimitiveUnsigned> PartialEq<NaturalPolynomial> for UnsignedPolynomial<T>
where
    Natural: PartialEq<T>,
{
    /// Determines whether an [`UnsignedPolynomial`] is equal to a [`NaturalPolynomial`].
    ///
    /// The two are equal when they have the same coefficients, so the zero polynomials are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    /// Polynomials of different degrees are compared in constant time.
    ///
    /// # Examples
    /// See [here](super::partial_eq_unsigned_polynomial#partial_eq).
    #[inline]
    fn eq(&self, other: &NaturalPolynomial) -> bool {
        other == self
    }
}
