// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;

impl FromIterator<Natural> for NaturalVector {
    /// Collects an iterator of [`Natural`]s into a [`NaturalVector`].
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v: NaturalVector = (1u32..=3).map(Natural::from).collect();
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// let v: NaturalVector = core::iter::empty().collect();
    /// assert_eq!(v.to_string(), "()");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = Natural>>(xs: I) -> Self {
        Self {
            elements: xs.into_iter().collect(),
        }
    }
}
