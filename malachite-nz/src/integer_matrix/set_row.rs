// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_vector::IntegerVector;

impl IntegerMatrix {
    /// Replaces row `i` of an [`IntegerMatrix`] with the elements of an [`IntegerVector`], taking
    /// the vector by reference.
    ///
    /// Indices start at 0. The dimension of `v` must equal the width of the matrix.
    /// [`set_row_owned`](IntegerMatrix::set_row_owned) takes the vector by value, which avoids
    /// cloning its elements.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `v.elements.len()` plus the total
    /// number of bits in the elements of `v`.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to the height of the matrix, or if the dimension of
    /// `v` is not equal to its width.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut m = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m.set_row(1, &IntegerVector::from_str("(7, 8, 9)").unwrap());
    /// assert_eq!(m.to_string(), "[1, 2, 3; 7, 8, 9]");
    /// ```
    pub fn set_row(&mut self, i: u64, v: &IntegerVector) {
        let row = self.row_iter_mut(i).into_slice();
        assert_eq!(
            v.elements.len(),
            row.len(),
            "the vector's dimension must equal the width of the matrix"
        );
        row.clone_from_slice(&v.elements);
    }

    /// Replaces row `i` of an [`IntegerMatrix`] with the elements of an [`IntegerVector`], taking
    /// the vector by value.
    ///
    /// Indices start at 0. The dimension of `v` must equal the width of the matrix.
    /// [`set_row`](IntegerMatrix::set_row) takes the vector by reference instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.width()`.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to the height of the matrix, or if the dimension of
    /// `v` is not equal to its width.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut m = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m.set_row_owned(1, IntegerVector::from_str("(7, 8, 9)").unwrap());
    /// assert_eq!(m.to_string(), "[1, 2, 3; 7, 8, 9]");
    /// ```
    pub fn set_row_owned(&mut self, i: u64, v: IntegerVector) {
        let row = self.row_iter_mut(i).into_slice();
        assert_eq!(
            v.elements.len(),
            row.len(),
            "the vector's dimension must equal the width of the matrix"
        );
        for (x, y) in row.iter_mut().zip(v.elements) {
            *x = y;
        }
    }
}
