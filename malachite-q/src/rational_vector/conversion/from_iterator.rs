// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;

impl FromIterator<Rational> for RationalVector {
    /// Collects an iterator of [`Rational`]s into a [`RationalVector`].
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v: RationalVector = (1u32..=3).map(|d| Rational::from_unsigneds(1, d)).collect();
    /// assert_eq!(v.to_string(), "(1, 1/2, 1/3)");
    /// let v: RationalVector = core::iter::empty().collect();
    /// assert_eq!(v.to_string(), "()");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = Rational>>(xs: I) -> Self {
        Self {
            elements: xs.into_iter().collect(),
        }
    }
}
