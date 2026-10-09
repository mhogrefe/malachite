// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> FromIterator<T> for UnsignedVector<T> {
    /// Collects an iterator of elements into an [`UnsignedVector`].
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
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v: UnsignedVector<u8> = (1..=3).collect();
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// let v: UnsignedVector<u8> = core::iter::empty().collect();
    /// assert_eq!(v.to_string(), "()");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(xs: I) -> Self {
        Self {
            elements: xs.into_iter().collect(),
        }
    }
}
