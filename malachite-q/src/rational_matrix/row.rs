// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use crate::rational_vector::RationalVector;

impl RationalMatrix {
    /// Returns row `i` of a [`RationalMatrix`] as a [`RationalVector`].
    ///
    /// Indices start at 0. The vector's dimension is the width of the matrix, and its elements are
    /// copied from the matrix. To read the elements without copying them, iterating over the row
    /// with [`row_iter`](RationalMatrix::row_iter) may be preferable.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// elements of the row.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to the height of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let m = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(m.row(1).to_string(), "(4, 5, 6)");
    ///
    /// let m = RationalMatrix::from_str("[]_{2,0}").unwrap();
    /// assert_eq!(m.row(1).to_string(), "()");
    /// ```
    #[inline]
    pub fn row(&self, i: u64) -> RationalVector {
        RationalVector {
            elements: self.row_iter(i).as_slice().to_vec(),
        }
    }
}
