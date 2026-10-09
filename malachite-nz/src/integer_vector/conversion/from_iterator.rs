// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;

impl FromIterator<Integer> for IntegerVector {
    /// Collects an iterator of [`Integer`]s into an [`IntegerVector`].
    ///
    /// The vector's dimension is the number of elements the iterator produces, and every finite
    /// iterator gives a valid vector.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements produced, not
    /// counting the cost of producing them.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v: IntegerVector = (-1..=1).map(Integer::from).collect();
    /// assert_eq!(v.to_string(), "(-1, 0, 1)");
    /// let v: IntegerVector = core::iter::empty().collect();
    /// assert_eq!(v.to_string(), "()");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = Integer>>(xs: I) -> Self {
        Self {
            elements: xs.into_iter().collect(),
        }
    }
}
