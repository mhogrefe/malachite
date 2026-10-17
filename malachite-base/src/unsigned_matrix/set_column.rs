// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> UnsignedMatrix<T> {
    /// Replaces column `j` of an [`UnsignedMatrix`] with the elements of an [`UnsignedVector`],
    /// taking the vector by reference.
    ///
    /// Indices start at 0. The dimension of `v` must equal the height of the matrix.
    /// [`set_column_owned`](UnsignedMatrix::set_column_owned) takes the vector by value instead.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m.set_column(2, &UnsignedVector::<u64>::from_str("(7, 8)").unwrap());
    /// assert_eq!(m.to_string(), "[1, 2, 7; 4, 5, 8]");
    /// ```
    pub fn set_column(&mut self, j: usize, v: &UnsignedVector<T>) {
        let column = self.column_iter_mut(j);
        assert_eq!(
            v.elements.len(),
            column.len(),
            "the vector's dimension must equal the height of the matrix"
        );
        for (x, &y) in column.zip(&v.elements) {
            *x = y;
        }
    }

    /// Replaces column `j` of an [`UnsignedMatrix`] with the elements of an [`UnsignedVector`],
    /// taking the vector by value.
    ///
    /// Indices start at 0. The dimension of `v` must equal the height of the matrix.
    /// [`set_column`](UnsignedMatrix::set_column) takes the vector by reference instead.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m.set_column_owned(2, UnsignedVector::<u64>::from_str("(7, 8)").unwrap());
    /// assert_eq!(m.to_string(), "[1, 2, 7; 4, 5, 8]");
    /// ```
    pub fn set_column_owned(&mut self, j: usize, v: UnsignedVector<T>) {
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
