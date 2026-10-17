// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::{ShortlexIntegerMatrix, ShortlexIntegerMatrixRef};
use core::cmp::Ordering;

impl Ord for ShortlexIntegerMatrixRef<'_> {
    /// Compares two [`ShortlexIntegerMatrixRef`]s.
    ///
    /// The order is shortlex: the matrices are compared first by their number of elements, then by
    /// height, then by width, and then, in case of a tie, lexicographically, by their elements in
    /// row-major order. This is a total order, and its equality agrees with
    /// [`IntegerMatrix`](crate::integer_matrix::IntegerMatrix) equality.
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
    /// use malachite_nz::integer_matrix::{IntegerMatrix, ShortlexIntegerMatrixRef};
    ///
    /// let five = IntegerMatrix::from_str("[5]").unwrap();
    /// let row = IntegerMatrix::from_str("[1, 2]").unwrap();
    /// let column = IntegerMatrix::from_str("[1; 2]").unwrap();
    /// let other_column = IntegerMatrix::from_str("[1; 3]").unwrap();
    ///
    /// // The number of elements decides first, whatever the elements are.
    /// assert!(ShortlexIntegerMatrixRef(&five) < ShortlexIntegerMatrixRef(&row));
    /// // Then the height, even when the elements are the same.
    /// assert!(ShortlexIntegerMatrixRef(&row) < ShortlexIntegerMatrixRef(&column));
    /// // At equal shapes, the first differing element decides.
    /// assert!(ShortlexIntegerMatrixRef(&column) < ShortlexIntegerMatrixRef(&other_column));
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

impl PartialOrd for ShortlexIntegerMatrixRef<'_> {
    /// Compares two [`ShortlexIntegerMatrixRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShortlexIntegerMatrix {
    /// Compares two [`ShortlexIntegerMatrix`]s.
    ///
    /// See the [`Ord`] implementation for [`ShortlexIntegerMatrixRef`] for details.
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
    /// use malachite_nz::integer_matrix::{IntegerMatrix, ShortlexIntegerMatrix};
    /// use std::collections::BTreeSet;
    ///
    /// let column = IntegerMatrix::from_str("[1; 2]").unwrap();
    /// let row = IntegerMatrix::from_str("[1, 2]").unwrap();
    /// let five = IntegerMatrix::from_str("[5]").unwrap();
    ///
    /// // The wrapper makes matrices usable in ordered collections.
    /// let set: BTreeSet<_> = [column, row, five]
    ///     .into_iter()
    ///     .map(ShortlexIntegerMatrix)
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

impl PartialOrd for ShortlexIntegerMatrix {
    /// Compares two [`ShortlexIntegerMatrix`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
