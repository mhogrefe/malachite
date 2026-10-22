// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014, 2015 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::determinant_bound::determinant_bound_helper;
use crate::natural::Natural;

impl IntegerMatrix {
    /// Returns an upper bound for the absolute values of the determinants of all the square
    /// submatrices of an [`IntegerMatrix`] of any shape, from Hadamard's inequality.
    ///
    /// The bound is computed as by [`determinant_bound`](IntegerMatrix::determinant_bound), except
    /// that zero rows and columns are left out of the products of norms. Since every other row and
    /// column has a norm of at least 1, the bound then holds for the matrix obtained by deleting
    /// any rows and columns. The norms and their products are computed with floating-point
    /// magnitudes whose 30-bit mantissas are always rounded up, so the bound is rigorous but may
    /// exceed the exact Hadamard bound slightly. The bound is positive, and it is 1 for a matrix
    /// with no elements. It may be far from optimal when the height and width are very different.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw)$
    ///
    /// $M(h, w) = O(h + w)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is `self.height()`, and $w$ is
    /// `self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3]").unwrap();
    /// assert_eq!(a.submatrix_determinant_bound(), 4u32);
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 0, 0]").unwrap();
    /// assert_eq!(a.submatrix_determinant_bound(), 3u32);
    ///
    /// let a = IntegerMatrix::from_str("[]_{2,0}").unwrap();
    /// assert_eq!(a.submatrix_determinant_bound(), 1u32);
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_det_bound_submatrix` from `fmpz_mat/det_bound.c`, FLINT
    /// 3.6.0.
    #[inline]
    pub fn submatrix_determinant_bound(&self) -> Natural {
        determinant_bound_helper(self, false)
    }
}
