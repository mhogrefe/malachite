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
    /// Returns column `j` of an [`UnsignedMatrix`] as an [`UnsignedVector`].
    ///
    /// Indices start at 0. The vector's dimension is the height of the matrix, and its elements are
    /// copied from the matrix. To read the elements without copying them, iterating over the column
    /// with [`column_iter`](UnsignedMatrix::column_iter) may be preferable.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height()`.
    ///
    /// # Panics
    /// Panics if `j` is greater than or equal to the width of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(m.column(2).to_string(), "(3, 6)");
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[]_{0,3}").unwrap();
    /// assert_eq!(m.column(1).to_string(), "()");
    /// ```
    #[inline]
    pub fn column(&self, j: usize) -> UnsignedVector<T> {
        self.column_iter(j).copied().collect()
    }
}
