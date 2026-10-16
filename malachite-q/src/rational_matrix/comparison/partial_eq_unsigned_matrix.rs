// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::unsigned_matrix::UnsignedMatrix;

impl<T: PrimitiveUnsigned> PartialEq<UnsignedMatrix<T>> for RationalMatrix
where
    Rational: PartialEq<T>,
{
    /// Determines whether a [`RationalMatrix`] is equal to an [`UnsignedMatrix`].
    ///
    /// The two are equal when they have the same height and width and equal elements in each
    /// position. So two matrices with no elements are equal only when their shapes match, and a
    /// [`RationalMatrix`] with an element that is negative, is not an integer, or is too large for
    /// `T` is equal to no [`UnsignedMatrix<T>`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`. Matrices of
    /// different shapes are compared in constant time.
    ///
    /// # Examples
    /// See [here](super::partial_eq_unsigned_matrix#partial_eq).
    fn eq(&self, other: &UnsignedMatrix<T>) -> bool {
        self.height == other.height()
            && self.width == other.width()
            && self
                .elements
                .iter()
                .zip(&other.elements)
                .all(|(x, y)| x == y)
    }
}

impl<T: PrimitiveUnsigned> PartialEq<RationalMatrix> for UnsignedMatrix<T>
where
    Rational: PartialEq<T>,
{
    /// Determines whether an [`UnsignedMatrix`] is equal to a [`RationalMatrix`].
    ///
    /// The two are equal when they have the same height and width and equal elements in each
    /// position, so two matrices with no elements are equal only when their shapes match.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`. Matrices of
    /// different shapes are compared in constant time.
    ///
    /// # Examples
    /// See [here](super::partial_eq_unsigned_matrix#partial_eq).
    #[inline]
    fn eq(&self, other: &RationalMatrix) -> bool {
        other == self
    }
}
