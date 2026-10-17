// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use alloc::vec::{IntoIter, Vec};
use core::iter::{FusedIterator, StepBy};
use core::slice::{Iter, IterMut};

impl NaturalMatrix {
    /// Returns a double-ended iterator over the columns of a [`NaturalMatrix`], from left to right.
    ///
    /// Each column is an iterator over its elements, from top to bottom, as
    /// [`column_iter`](NaturalMatrix::column_iter) returns, so visiting every element column by
    /// column is a double `for` loop with no indices. A matrix with no rows still has its columns,
    /// each empty.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::iterators::prefix_to_string;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(
    ///     prefix_to_string(m.columns().map(|column| prefix_to_string(column, 10)), 10),
    ///     "[[1, 4], [2, 5], [3, 6]]"
    /// );
    /// assert_eq!(
    ///     prefix_to_string(
    ///         m.columns().rev().map(|column| prefix_to_string(column, 10)),
    ///         10
    ///     ),
    ///     "[[3, 6], [2, 5], [1, 4]]"
    /// );
    ///
    /// let m = NaturalMatrix::from_str("[]_{0,3}").unwrap();
    /// assert_eq!(m.columns().len(), 3);
    /// ```
    pub fn columns(&self) -> NaturalMatrixColumns<'_> {
        NaturalMatrixColumns {
            elements: &self.elements,
            width: self.width,
            start: 0,
            end: self.width,
        }
    }

    /// Returns a double-ended iterator over the columns of a [`NaturalMatrix`], from left to right,
    /// each given as an iterator over mutable references to its elements, from top to bottom.
    ///
    /// Each column yields what [`column_iter_mut`](NaturalMatrix::column_iter_mut) does, but all of
    /// them can be held at once. A matrix with no rows still has its columns, each empty.
    ///
    /// The elements of a column are spread through `elements`, one per row, so the columns can't
    /// all be borrowed as strides at once. Instead, the iterator holds an iterator over each row,
    /// and gathers each column's references into a [`Vec`] when it produces that column, which
    /// takes time and memory proportional to the height.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // Multiply each column by its position, counting from 1.
    /// let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// for (column, k) in m.columns_mut().zip(1u32..) {
    ///     for x in column {
    ///         *x *= Natural::from(k);
    ///     }
    /// }
    /// assert_eq!(m.to_string(), "[1, 4, 9; 4, 10, 18]");
    /// ```
    pub fn columns_mut(&mut self) -> NaturalMatrixColumnsMut<'_> {
        let rows = if self.width == 0 {
            Vec::new()
        } else {
            self.elements
                .chunks_exact_mut(self.width)
                .map(<[Natural]>::iter_mut)
                .collect()
        };
        NaturalMatrixColumnsMut {
            rows,
            remaining: self.width,
        }
    }
}

/// A double-ended iterator over the columns of a [`NaturalMatrix`], each given as an iterator over
/// its elements.
///
/// This `struct` is created by [`NaturalMatrix::columns`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct NaturalMatrixColumns<'a> {
    elements: &'a [Natural],
    width: usize,
    // The columns not yet produced are those with indices in `start..end`.
    start: usize,
    end: usize,
}

impl<'a> NaturalMatrixColumns<'a> {
    fn column(&self, j: usize) -> StepBy<Iter<'a, Natural>> {
        // With no rows there are no elements, so `j..` would be out of range.
        self.elements
            .get(j..)
            .unwrap_or_default()
            .iter()
            .step_by(self.width)
    }
}

impl<'a> Iterator for NaturalMatrixColumns<'a> {
    type Item = StepBy<Iter<'a, Natural>>;

    fn next(&mut self) -> Option<StepBy<Iter<'a, Natural>>> {
        if self.start == self.end {
            return None;
        }
        self.start += 1;
        Some(self.column(self.start - 1))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end - self.start;
        (remaining, Some(remaining))
    }
}

impl<'a> DoubleEndedIterator for NaturalMatrixColumns<'a> {
    fn next_back(&mut self) -> Option<StepBy<Iter<'a, Natural>>> {
        if self.start == self.end {
            return None;
        }
        self.end -= 1;
        Some(self.column(self.end))
    }
}

impl<'a> ExactSizeIterator for NaturalMatrixColumns<'a> {}

impl<'a> FusedIterator for NaturalMatrixColumns<'a> {}

/// A double-ended iterator over the columns of a [`NaturalMatrix`], each given as an iterator over
/// mutable references to its elements.
///
/// This `struct` is created by [`NaturalMatrix::columns_mut`]; see its documentation for more.
#[derive(Debug)]
pub struct NaturalMatrixColumnsMut<'a> {
    // One iterator over each row. The next column takes one element from the front of each, and the
    // last column one from the back.
    rows: Vec<IterMut<'a, Natural>>,
    // The number of columns not yet produced. It can't be derived from `rows` when the height is 0.
    remaining: usize,
}

impl<'a> Iterator for NaturalMatrixColumnsMut<'a> {
    type Item = IntoIter<&'a mut Natural>;

    fn next(&mut self) -> Option<IntoIter<&'a mut Natural>> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        Some(
            self.rows
                .iter_mut()
                .map(|row| row.next().unwrap())
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a> DoubleEndedIterator for NaturalMatrixColumnsMut<'a> {
    fn next_back(&mut self) -> Option<IntoIter<&'a mut Natural>> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        Some(
            self.rows
                .iter_mut()
                .map(|row| row.next_back().unwrap())
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }
}

impl<'a> ExactSizeIterator for NaturalMatrixColumnsMut<'a> {}

impl<'a> FusedIterator for NaturalMatrixColumnsMut<'a> {}
