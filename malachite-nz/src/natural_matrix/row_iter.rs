// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use core::ops::Range;
use core::slice::{Iter, IterMut};

impl NaturalMatrix {
    // The range of positions in `elements` that row `i` occupies, panicking if `i` is out of range.
    fn row_range(&self, i: usize) -> Range<usize> {
        assert!(
            i < self.height,
            "row index {i} is out of range for a matrix with {} rows",
            self.height
        );
        i * self.width..(i + 1) * self.width
    }

    /// Returns an iterator over the elements in row `i` of a [`NaturalMatrix`], from left to right.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(prefix_to_string(m.row_iter(1), 10), "[4, 5, 6]");
    ///
    /// let m = NaturalMatrix::from_str("[]_{2,0}").unwrap();
    /// assert_eq!(prefix_to_string(m.row_iter(1), 10), "[]");
    /// ```
    #[inline]
    pub fn row_iter(&self, i: usize) -> Iter<'_, Natural> {
        self.elements[self.row_range(i)].iter()
    }

    /// Returns an iterator over mutable references to the elements in row `i` of a
    /// [`NaturalMatrix`], from left to right.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// for x in m.row_iter_mut(1) {
    ///     *x += Natural::ONE;
    /// }
    /// assert_eq!(m.to_string(), "[1, 2, 3; 5, 6, 7]");
    /// ```
    #[inline]
    pub fn row_iter_mut(&mut self, i: usize) -> IterMut<'_, Natural> {
        let range = self.row_range(i);
        self.elements[range].iter_mut()
    }
}
