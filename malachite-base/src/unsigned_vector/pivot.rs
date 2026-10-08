// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> UnsignedVector<T> {
    /// Returns the pivot of an [`UnsignedVector`]: its first nonzero element.
    ///
    /// This is the element that leads the vector when it is a row of a matrix in echelon form. It
    /// returns a copy of it, or `None` if every element is zero, which includes the 0-dimensional
    /// vector.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("(0, 0, 3, 0, 5)")
    ///         .unwrap()
    ///         .pivot(),
    ///     Some(3)
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("(0, 0)").unwrap().pivot(),
    ///     None
    /// );
    /// assert_eq!(UnsignedVector::<u32>::from_str("()").unwrap().pivot(), None);
    /// ```
    #[inline]
    pub fn pivot(&self) -> Option<T> {
        self.elements.iter().find(|&&x| x != T::ZERO).copied()
    }

    /// Returns the index of the pivot of an [`UnsignedVector`]: the position of its first nonzero
    /// element.
    ///
    /// Indices start at 0, as they do for [`Index`](core::ops::Index). Returns `None` if every
    /// element is zero, which includes the 0-dimensional vector. When it returns `Some(i)`,
    /// [`pivot`](Self::pivot) is the element at `i`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("(0, 0, 3, 0, 5)")
    ///         .unwrap()
    ///         .pivot_index(),
    ///     Some(2)
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("(0, 0)")
    ///         .unwrap()
    ///         .pivot_index(),
    ///     None
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("()").unwrap().pivot_index(),
    ///     None
    /// );
    /// ```
    #[inline]
    pub fn pivot_index(&self) -> Option<u64> {
        self.elements
            .iter()
            .position(|&x| x != T::ZERO)
            .map(u64::exact_from)
    }
}
