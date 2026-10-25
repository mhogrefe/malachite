// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use malachite_base::matrix::Matrix;
use malachite_nz::integer_matrix::IntegerMatrix;

impl PartialEq<IntegerMatrix> for RationalMatrix {
    /// Determines whether a [`RationalMatrix`] is equal to an [`IntegerMatrix`].
    ///
    /// The two are equal when they have the same height and width and equal elements in each
    /// position. So two matrices with no elements are equal only when their shapes match, and a
    /// [`RationalMatrix`] with an element that is not an integer is equal to no [`IntegerMatrix`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    /// Matrices of different shapes are compared in constant time.
    ///
    /// # Examples
    /// See [here](super::partial_eq_integer_matrix#partial_eq).
    fn eq(&self, other: &IntegerMatrix) -> bool {
        self.height() == other.height()
            && self.width() == other.width()
            && self
                .elements
                .iter()
                .zip(other.elements_ref())
                .all(|(x, y)| x == y)
    }
}

impl PartialEq<RationalMatrix> for IntegerMatrix {
    /// Determines whether an [`IntegerMatrix`] is equal to a [`RationalMatrix`].
    ///
    /// The two are equal when they have the same height and width and equal elements in each
    /// position, so two matrices with no elements are equal only when their shapes match.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    /// Matrices of different shapes are compared in constant time.
    ///
    /// # Examples
    /// See [here](super::partial_eq_integer_matrix#partial_eq).
    #[inline]
    fn eq(&self, other: &RationalMatrix) -> bool {
        other == self
    }
}
