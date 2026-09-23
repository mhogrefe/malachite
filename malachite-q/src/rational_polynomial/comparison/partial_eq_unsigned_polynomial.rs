// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::integer::Integer;

impl<T: PrimitiveUnsigned> PartialEq<UnsignedPolynomial<T>> for RationalPolynomial
where
    Integer: PartialEq<T>,
{
    /// Determines whether a [`RationalPolynomial`] is equal to an [`UnsignedPolynomial`].
    ///
    /// The two are equal when they have the same coefficients, so the zero polynomials are equal
    /// and a [`RationalPolynomial`] with a non-integer or negative coefficient, or one too large
    /// for `T`, is equal to no [`UnsignedPolynomial<T>`].
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
        self.denominator == 1u32 && self.numerator == *other
    }
}

impl<T: PrimitiveUnsigned> PartialEq<RationalPolynomial> for UnsignedPolynomial<T>
where
    Integer: PartialEq<T>,
{
    /// Determines whether an [`UnsignedPolynomial`] is equal to a [`RationalPolynomial`].
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
    fn eq(&self, other: &RationalPolynomial) -> bool {
        other == self
    }
}
