// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use core::iter::FusedIterator;
use core::mem;
use core::slice::{Iter, IterMut};

impl<T: PrimitiveUnsigned> UnsignedMatrix<T> {
    /// Returns a double-ended iterator over the rows of an [`UnsignedMatrix`], from top to bottom.
    ///
    /// Each row is an iterator over its elements, from left to right, as
    /// [`row_iter`](UnsignedMatrix::row_iter) returns, so visiting every element row by row is a
    /// double `for` loop with no indices. A matrix with no columns still has its rows, each empty.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::iterators::prefix_to_string;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(
    ///     prefix_to_string(m.rows().map(|row| prefix_to_string(row, 10)), 10),
    ///     "[[1, 2, 3], [4, 5, 6]]"
    /// );
    /// assert_eq!(
    ///     prefix_to_string(m.rows().rev().map(|row| prefix_to_string(row, 10)), 10),
    ///     "[[4, 5, 6], [1, 2, 3]]"
    /// );
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[]_{2,0}").unwrap();
    /// assert_eq!(m.rows().len(), 2);
    /// ```
    pub fn rows(&self) -> UnsignedMatrixRows<'_, T> {
        UnsignedMatrixRows {
            elements: &self.elements,
            width: self.width,
            remaining: self.height,
        }
    }

    /// Returns a double-ended iterator over the rows of an [`UnsignedMatrix`], from top to bottom,
    /// each given as an iterator over mutable references to its elements, from left to right.
    ///
    /// Each row is what [`row_iter_mut`](UnsignedMatrix::row_iter_mut) returns, but all of them can
    /// be held at once. A matrix with no columns still has its rows, each empty.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// // Multiply each row by its position, counting from 1.
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// for (row, k) in m.rows_mut().zip(1u32..) {
    ///     for x in row {
    ///         *x *= u64::from(k);
    ///     }
    /// }
    /// assert_eq!(m.to_string(), "[1, 2, 3; 8, 10, 12]");
    /// ```
    pub fn rows_mut(&mut self) -> UnsignedMatrixRowsMut<'_, T> {
        UnsignedMatrixRowsMut {
            elements: &mut self.elements,
            width: self.width,
            remaining: self.height,
        }
    }
}

/// A double-ended iterator over the rows of an [`UnsignedMatrix`], each given as an iterator over
/// its elements.
///
/// This `struct` is created by [`UnsignedMatrix::rows`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct UnsignedMatrixRows<'a, T> {
    // The rows not yet produced, from either end.
    elements: &'a [T],
    width: usize,
    // The number of rows not yet produced. It can't be derived from `elements` when the width is 0.
    remaining: usize,
}

impl<'a, T> Iterator for UnsignedMatrixRows<'a, T> {
    type Item = Iter<'a, T>;

    fn next(&mut self) -> Option<Iter<'a, T>> {
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

impl<'a, T> DoubleEndedIterator for UnsignedMatrixRows<'a, T> {
    fn next_back(&mut self) -> Option<Iter<'a, T>> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let (rest, row) = self.elements.split_at(self.elements.len() - self.width);
        self.elements = rest;
        Some(row.iter())
    }
}

impl<'a, T> ExactSizeIterator for UnsignedMatrixRows<'a, T> {}

impl<'a, T> FusedIterator for UnsignedMatrixRows<'a, T> {}

/// A double-ended iterator over the rows of an [`UnsignedMatrix`], each given as an iterator over
/// mutable references to its elements.
///
/// This `struct` is created by [`UnsignedMatrix::rows_mut`]; see its documentation for more.
#[derive(Debug)]
pub struct UnsignedMatrixRowsMut<'a, T> {
    // The rows not yet produced, from either end.
    elements: &'a mut [T],
    width: usize,
    // The number of rows not yet produced. It can't be derived from `elements` when the width is 0.
    remaining: usize,
}

impl<'a, T> Iterator for UnsignedMatrixRowsMut<'a, T> {
    type Item = IterMut<'a, T>;

    fn next(&mut self) -> Option<IterMut<'a, T>> {
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

impl<'a, T> DoubleEndedIterator for UnsignedMatrixRowsMut<'a, T> {
    fn next_back(&mut self) -> Option<IterMut<'a, T>> {
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

impl<'a, T> ExactSizeIterator for UnsignedMatrixRowsMut<'a, T> {}

impl<'a, T> FusedIterator for UnsignedMatrixRowsMut<'a, T> {}
