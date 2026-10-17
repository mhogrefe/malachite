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
    /// Replaces row `i` of an [`UnsignedMatrix`] with the elements of an [`UnsignedVector`], taking
    /// the vector by reference.
    ///
    /// Indices start at 0. The dimension of `v` must equal the width of the matrix.
    /// [`set_row_owned`](UnsignedMatrix::set_row_owned) takes the vector by value instead.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m.set_row(1, &UnsignedVector::<u64>::from_str("(7, 8, 9)").unwrap());
    /// assert_eq!(m.to_string(), "[1, 2, 3; 7, 8, 9]");
    /// ```
    pub fn set_row(&mut self, i: usize, v: &UnsignedVector<T>) {
        let row = self.row_iter_mut(i).into_slice();
        assert_eq!(
            v.elements.len(),
            row.len(),
            "the vector's dimension must equal the width of the matrix"
        );
        row.copy_from_slice(&v.elements);
    }

    /// Replaces row `i` of an [`UnsignedMatrix`] with the elements of an [`UnsignedVector`], taking
    /// the vector by value.
    ///
    /// Indices start at 0. The dimension of `v` must equal the width of the matrix.
    /// [`set_row`](UnsignedMatrix::set_row) takes the vector by reference instead.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m.set_row_owned(1, UnsignedVector::<u64>::from_str("(7, 8, 9)").unwrap());
    /// assert_eq!(m.to_string(), "[1, 2, 3; 7, 8, 9]");
    /// ```
    pub fn set_row_owned(&mut self, i: usize, v: UnsignedVector<T>) {
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
