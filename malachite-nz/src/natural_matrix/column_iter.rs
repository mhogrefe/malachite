// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use core::iter::StepBy;
use core::slice::{Iter, IterMut};

impl NaturalMatrix {
    // Panics if `j` is not a column index.
    fn check_column_index(&self, j: usize) {
        assert!(
            j < self.width,
            "column index {j} is out of range for a matrix with {} columns",
            self.width
        );
    }

    /// Returns an iterator over the elements in column `j` of a [`NaturalMatrix`], from top to
    /// bottom.
    ///
    /// Indices start at 0. The iterator yields one element for each row, and it is double-ended and
    /// knows its length.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `j` is greater than or equal to the width of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::iterators::prefix_to_string;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(prefix_to_string(m.column_iter(2), 10), "[3, 6]");
    ///
    /// let m = NaturalMatrix::from_str("[]_{0,3}").unwrap();
    /// assert_eq!(prefix_to_string(m.column_iter(1), 10), "[]");
    /// ```
    #[inline]
    pub fn column_iter(&self, j: usize) -> StepBy<Iter<'_, Natural>> {
        self.check_column_index(j);
        // With no rows there are no elements, so `j..` would be out of range.
        self.elements
            .get(j..)
            .unwrap_or_default()
            .iter()
            .step_by(self.width)
    }

    /// Returns an iterator over mutable references to the elements in column `j` of a
    /// [`NaturalMatrix`], from top to bottom.
    ///
    /// Indices start at 0. The iterator yields one element for each row, and it is double-ended and
    /// knows its length.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `j` is greater than or equal to the width of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// for x in m.column_iter_mut(2) {
    ///     *x += Natural::ONE;
    /// }
    /// assert_eq!(m.to_string(), "[1, 2, 4; 4, 5, 7]");
    /// ```
    #[inline]
    pub fn column_iter_mut(&mut self, j: usize) -> StepBy<IterMut<'_, Natural>> {
        self.check_column_index(j);
        let width = self.width;
        self.elements
            .get_mut(j..)
            .unwrap_or_default()
            .iter_mut()
            .step_by(width)
    }
}
