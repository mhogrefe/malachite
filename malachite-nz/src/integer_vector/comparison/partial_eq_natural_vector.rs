// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::integer_vector::IntegerVector;
use crate::natural_vector::NaturalVector;

impl PartialEq<NaturalVector> for IntegerVector {
    /// Determines whether an [`IntegerVector`] is equal to a [`NaturalVector`].
    ///
    /// The two are equal when they have the same dimension and equal elements in each position. So
    /// the 0-dimensional vectors are equal, and an [`IntegerVector`] with a negative element is
    /// equal to no [`NaturalVector`].
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
    /// See [here](super::partial_eq_natural_vector#partial_eq).
    fn eq(&self, other: &NaturalVector) -> bool {
        self.elements.len() == other.elements.len()
            && self
                .elements
                .iter()
                .zip(&other.elements)
                .all(|(x, y)| x == y)
    }
}

impl PartialEq<IntegerVector> for NaturalVector {
    /// Determines whether a [`NaturalVector`] is equal to an [`IntegerVector`].
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
    /// See [here](super::partial_eq_natural_vector#partial_eq).
    #[inline]
    fn eq(&self, other: &IntegerVector) -> bool {
        other == self
    }
}
