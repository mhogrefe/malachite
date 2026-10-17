// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use core::ops::Range;
use core::slice::{Iter, IterMut};

impl<T: PrimitiveUnsigned> UnsignedMatrix<T> {
    // The range of positions in `elements` that row `i` occupies, panicking if `i` is out of range.
    fn row_range(&self, i: usize) -> Range<usize> {
        assert!(
            i < self.height,
            "row index {i} is out of range for a matrix with {} rows",
            self.height
        );
        i * self.width..(i + 1) * self.width
    }

    /// Returns an iterator over the elements in row `i` of an [`UnsignedMatrix`], from left to
    /// right.
    ///
    /// Indices start at 0. The iterator yields one element for each column, and it is double-ended
    /// and knows its length.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to the height of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::iterators::prefix_to_string;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(prefix_to_string(m.row_iter(1), 10), "[4, 5, 6]");
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[]_{2,0}").unwrap();
    /// assert_eq!(prefix_to_string(m.row_iter(1), 10), "[]");
    /// ```
    #[inline]
    pub fn row_iter(&self, i: usize) -> Iter<'_, T> {
        self.elements[self.row_range(i)].iter()
    }

    /// Returns an iterator over mutable references to the elements in row `i` of an
    /// [`UnsignedMatrix`], from left to right.
    ///
    /// Indices start at 0. The iterator yields one element for each column, and it is double-ended
    /// and knows its length.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to the height of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// for x in m.row_iter_mut(1) {
    ///     *x += 1;
    /// }
    /// assert_eq!(m.to_string(), "[1, 2, 3; 5, 6, 7]");
    /// ```
    #[inline]
    pub fn row_iter_mut(&mut self, i: usize) -> IterMut<'_, T> {
        let range = self.row_range(i);
        self.elements[range].iter_mut()
    }
}
