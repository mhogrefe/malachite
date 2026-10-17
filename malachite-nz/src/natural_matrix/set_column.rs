// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_matrix::NaturalMatrix;
use crate::natural_vector::NaturalVector;

impl NaturalMatrix {
    /// Replaces column `j` of a [`NaturalMatrix`] with the elements of a [`NaturalVector`], taking
    /// the vector by reference.
    ///
    /// Indices start at 0. The dimension of `v` must equal the height of the matrix.
    /// [`set_column_owned`](NaturalMatrix::set_column_owned) takes the vector by value, which
    /// avoids cloning its elements.
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
    /// Panics if `j` is greater than or equal to the width of the matrix, or if the dimension of
    /// `v` is not equal to its height.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m.set_column(2, &NaturalVector::from_str("(7, 8)").unwrap());
    /// assert_eq!(m.to_string(), "[1, 2, 7; 4, 5, 8]");
    /// ```
    pub fn set_column(&mut self, j: u64, v: &NaturalVector) {
        let column = self.column_iter_mut(j);
        assert_eq!(
            v.elements.len(),
            column.len(),
            "the vector's dimension must equal the height of the matrix"
        );
        for (x, y) in column.zip(&v.elements) {
            x.clone_from(y);
        }
    }

    /// Replaces column `j` of a [`NaturalMatrix`] with the elements of a [`NaturalVector`], taking
    /// the vector by value.
    ///
    /// Indices start at 0. The dimension of `v` must equal the height of the matrix.
    /// [`set_column`](NaturalMatrix::set_column) takes the vector by reference instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height()`.
    ///
    /// # Panics
    /// Panics if `j` is greater than or equal to the width of the matrix, or if the dimension of
    /// `v` is not equal to its height.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m.set_column_owned(2, NaturalVector::from_str("(7, 8)").unwrap());
    /// assert_eq!(m.to_string(), "[1, 2, 7; 4, 5, 8]");
    /// ```
    pub fn set_column_owned(&mut self, j: u64, v: NaturalVector) {
        let column = self.column_iter_mut(j);
        assert_eq!(
            v.elements.len(),
            column.len(),
            "the vector's dimension must equal the height of the matrix"
        );
        for (x, y) in column.zip(v.elements) {
            *x = y;
        }
    }
}
