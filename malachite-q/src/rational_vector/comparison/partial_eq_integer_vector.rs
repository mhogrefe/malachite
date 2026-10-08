// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::rational_vector::RationalVector;
use malachite_nz::integer_vector::IntegerVector;

impl PartialEq<IntegerVector> for RationalVector {
    /// Determines whether a [`RationalVector`] is equal to an [`IntegerVector`].
    ///
    /// The two are equal when they have the same dimension and equal elements in each position. So
    /// the 0-dimensional vectors are equal, and a [`RationalVector`] with an element that is not an
    /// integer is equal to no [`IntegerVector`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`. Vectors of
    /// different dimensions are compared in constant time.
    ///
    /// # Examples
    /// See [here](super::partial_eq_integer_vector#partial_eq).
    fn eq(&self, other: &IntegerVector) -> bool {
        self.elements.len() == other.elements.len()
            && self
                .elements
                .iter()
                .zip(&other.elements)
                .all(|(x, y)| x == y)
    }
}

impl PartialEq<RationalVector> for IntegerVector {
    /// Determines whether an [`IntegerVector`] is equal to a [`RationalVector`].
    ///
    /// The two are equal when they have the same dimension and equal elements in each position, so
    /// the 0-dimensional vectors are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`. Vectors of
    /// different dimensions are compared in constant time.
    ///
    /// # Examples
    /// See [here](super::partial_eq_integer_vector#partial_eq).
    #[inline]
    fn eq(&self, other: &RationalVector) -> bool {
        other == self
    }
}
