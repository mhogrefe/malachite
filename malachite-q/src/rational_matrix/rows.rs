// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use core::iter::FusedIterator;
use core::mem;
use core::slice::{Iter, IterMut};

impl RationalMatrix {
    /// Returns a double-ended iterator over the rows of a [`RationalMatrix`], from top to bottom.
    ///
    /// Each row is an iterator over its elements, from left to right, as
    /// [`row_iter`](RationalMatrix::row_iter) returns, so visiting every element row by row is a
    /// double `for` loop with no indices. A matrix with no columns still has its rows, each empty.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::iterators::prefix_to_string;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let m = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(
    ///     prefix_to_string(m.rows().map(|row| prefix_to_string(row, 10)), 10),
    ///     "[[1, 2, 3], [4, 5, 6]]"
    /// );
    /// assert_eq!(
    ///     prefix_to_string(m.rows().rev().map(|row| prefix_to_string(row, 10)), 10),
    ///     "[[4, 5, 6], [1, 2, 3]]"
    /// );
    ///
    /// let m = RationalMatrix::from_str("[]_{2,0}").unwrap();
    /// assert_eq!(m.rows().len(), 2);
    /// ```
    pub fn rows(&self) -> RationalMatrixRows<'_> {
        RationalMatrixRows {
            elements: &self.elements,
            width: self.width,
            remaining: self.height,
        }
    }

    /// Returns a double-ended iterator over the rows of a [`RationalMatrix`], from top to bottom,
    /// each given as an iterator over mutable references to its elements, from left to right.
    ///
    /// Each row is what [`row_iter_mut`](RationalMatrix::row_iter_mut) returns, but all of them can
    /// be held at once. A matrix with no columns still has its rows, each empty.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// // Multiply each row by its position, counting from 1.
    /// let mut m = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// for (row, k) in m.rows_mut().zip(1u32..) {
    ///     for x in row {
    ///         *x *= Rational::from(k);
    ///     }
    /// }
    /// assert_eq!(m.to_string(), "[1, 2, 3; 8, 10, 12]");
    /// ```
    pub fn rows_mut(&mut self) -> RationalMatrixRowsMut<'_> {
        RationalMatrixRowsMut {
            elements: &mut self.elements,
            width: self.width,
            remaining: self.height,
        }
    }
}

/// A double-ended iterator over the rows of a [`RationalMatrix`], each given as an iterator over
/// its elements.
///
/// This `struct` is created by [`RationalMatrix::rows`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RationalMatrixRows<'a> {
    // The rows not yet produced, from either end.
    elements: &'a [Rational],
    width: usize,
    // The number of rows not yet produced. It can't be derived from `elements` when the width is 0.
    remaining: usize,
}

impl<'a> Iterator for RationalMatrixRows<'a> {
    type Item = Iter<'a, Rational>;

    fn next(&mut self) -> Option<Iter<'a, Rational>> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let (row, rest) = self.elements.split_at(self.width);
        self.elements = rest;
        Some(row.iter())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a> DoubleEndedIterator for RationalMatrixRows<'a> {
    fn next_back(&mut self) -> Option<Iter<'a, Rational>> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let (rest, row) = self.elements.split_at(self.elements.len() - self.width);
        self.elements = rest;
        Some(row.iter())
    }
}

impl<'a> ExactSizeIterator for RationalMatrixRows<'a> {}

impl<'a> FusedIterator for RationalMatrixRows<'a> {}

/// A double-ended iterator over the rows of a [`RationalMatrix`], each given as an iterator over
/// mutable references to its elements.
///
/// This `struct` is created by [`RationalMatrix::rows_mut`]; see its documentation for more.
#[derive(Debug)]
pub struct RationalMatrixRowsMut<'a> {
    // The rows not yet produced, from either end.
    elements: &'a mut [Rational],
    width: usize,
    // The number of rows not yet produced. It can't be derived from `elements` when the width is 0.
    remaining: usize,
}

impl<'a> Iterator for RationalMatrixRowsMut<'a> {
    type Item = IterMut<'a, Rational>;

    fn next(&mut self) -> Option<IterMut<'a, Rational>> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let (row, rest) = mem::take(&mut self.elements).split_at_mut(self.width);
        self.elements = rest;
        Some(row.iter_mut())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a> DoubleEndedIterator for RationalMatrixRowsMut<'a> {
    fn next_back(&mut self) -> Option<IterMut<'a, Rational>> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let elements = mem::take(&mut self.elements);
        let (rest, row) = elements.split_at_mut(elements.len() - self.width);
        self.elements = rest;
        Some(row.iter_mut())
    }
}

impl<'a> ExactSizeIterator for RationalMatrixRowsMut<'a> {}

impl<'a> FusedIterator for RationalMatrixRowsMut<'a> {}
