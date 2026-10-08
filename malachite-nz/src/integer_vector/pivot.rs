// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use malachite_base::num::conversion::traits::ExactFrom;

impl IntegerVector {
    /// Returns the pivot of an [`IntegerVector`]: its first nonzero element.
    ///
    /// This is the element that leads the vector when it is a row of a matrix in echelon form. It
    /// returns a reference to it, or `None` if every element is zero, which includes the
    /// 0-dimensional vector.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(0, 0, -3, 0, 5)").unwrap().pivot(),
    ///     Some(&Integer::from(-3))
    /// );
    /// assert_eq!(IntegerVector::from_str("(0, 0)").unwrap().pivot(), None);
    /// assert_eq!(IntegerVector::from_str("()").unwrap().pivot(), None);
    /// ```
    #[inline]
    pub fn pivot(&self) -> Option<&Integer> {
        self.elements.iter().find(|x| **x != 0u32)
    }

    /// Returns the index of the pivot of an [`IntegerVector`]: the position of its first nonzero
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(0, 0, -3, 0, 5)")
    ///         .unwrap()
    ///         .pivot_index(),
    ///     Some(2)
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_str("(0, 0)").unwrap().pivot_index(),
    ///     None
    /// );
    /// assert_eq!(IntegerVector::from_str("()").unwrap().pivot_index(), None);
    /// ```
    #[inline]
    pub fn pivot_index(&self) -> Option<u64> {
        self.elements
            .iter()
            .position(|x| *x != 0u32)
            .map(u64::exact_from)
    }
}
