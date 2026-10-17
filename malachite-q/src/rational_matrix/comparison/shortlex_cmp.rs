// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::{ShortlexRationalMatrix, ShortlexRationalMatrixRef};
use core::cmp::Ordering;

impl Ord for ShortlexRationalMatrixRef<'_> {
    /// Compares two [`ShortlexRationalMatrixRef`]s.
    ///
    /// The order is shortlex: the matrices are compared first by their number of elements, then by
    /// height, then by width, and then, in case of a tie, lexicographically, by their elements in
    /// row-major order. This is a total order, and its equality agrees with
    /// [`RationalMatrix`](crate::rational_matrix::RationalMatrix) equality.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two matrices'
    /// total number of bits, summed over their elements. Matrices of different shapes are compared
    /// in constant time.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::{RationalMatrix, ShortlexRationalMatrixRef};
    ///
    /// let five = RationalMatrix::from_str("[5]").unwrap();
    /// let row = RationalMatrix::from_str("[1, 2]").unwrap();
    /// let column = RationalMatrix::from_str("[1; 2]").unwrap();
    /// let other_column = RationalMatrix::from_str("[1; 3]").unwrap();
    ///
    /// // The number of elements decides first, whatever the elements are.
    /// assert!(ShortlexRationalMatrixRef(&five) < ShortlexRationalMatrixRef(&row));
    /// // Then the height, even when the elements are the same.
    /// assert!(ShortlexRationalMatrixRef(&row) < ShortlexRationalMatrixRef(&column));
    /// // At equal shapes, the first differing element decides.
    /// assert!(ShortlexRationalMatrixRef(&column) < ShortlexRationalMatrixRef(&other_column));
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let (m, n) = (self.0, other.0);
        // Once the shapes agree, comparing the `Vec`s is the comparison at the first element where
        // the two differ.
        m.elements
            .len()
            .cmp(&n.elements.len())
            .then_with(|| m.height.cmp(&n.height))
            .then_with(|| m.width.cmp(&n.width))
            .then_with(|| m.elements.cmp(&n.elements))
    }
}

impl PartialOrd for ShortlexRationalMatrixRef<'_> {
    /// Compares two [`ShortlexRationalMatrixRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShortlexRationalMatrix {
    /// Compares two [`ShortlexRationalMatrix`]s.
    ///
    /// See the [`Ord`] implementation for [`ShortlexRationalMatrixRef`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two matrices'
    /// total number of bits, summed over their elements. Matrices of different shapes are compared
    /// in constant time.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::{RationalMatrix, ShortlexRationalMatrix};
    /// use std::collections::BTreeSet;
    ///
    /// let column = RationalMatrix::from_str("[1; 2]").unwrap();
    /// let row = RationalMatrix::from_str("[1, 2]").unwrap();
    /// let five = RationalMatrix::from_str("[5]").unwrap();
    ///
    /// // The wrapper makes matrices usable in ordered collections.
    /// let set: BTreeSet<_> = [column, row, five]
    ///     .into_iter()
    ///     .map(ShortlexRationalMatrix)
    ///     .collect();
    /// assert_eq!(
    ///     set.iter().map(|m| m.to_string()).collect::<Vec<_>>(),
    ///     ["[5]", "[1, 2]", "[1; 2]"]
    /// );
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(&other.as_ref())
    }
}

impl PartialOrd for ShortlexRationalMatrix {
    /// Compares two [`ShortlexRationalMatrix`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
