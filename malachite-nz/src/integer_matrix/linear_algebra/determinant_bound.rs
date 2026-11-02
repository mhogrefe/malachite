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
use crate::integer_matrix::arithmetic::mag::Mag;
use crate::natural::Natural;
use alloc::vec;
use malachite_base::num::basic::traits::One;

// An upper bound for the absolute value of the determinant of every square submatrix of `a`, from
// Hadamard's inequality: the square root of the smaller of the product of the squared Euclidean
// norms of the rows and the product of those of the columns. If `include_zero_norms` is false, zero
// rows and columns are left out of the products.
//
// This is `fmpz_mat_det_bound_inner` from `fmpz_mat/det_bound.c`, FLINT 3.6.0.
pub(crate) fn determinant_bound_helper(a: &IntegerMatrix, include_zero_norms: bool) -> Natural {
    let (height, width) = (a.height, a.width);
    if height == 0 || width == 0 {
        return Natural::ONE;
    }
    let mut norms = vec![Mag::ZERO; height + width];
    let (row_norms, column_norms) = norms.split_at_mut(height);
    for (i, row) in a.elements.chunks_exact(width).enumerate() {
        for (j, x) in row.iter().enumerate() {
            let t = Mag::from_natural(x.unsigned_abs_ref());
            row_norms[i].add_mul_assign(t, t);
            column_norms[j].add_mul_assign(t, t);
        }
    }
    let product = |norms: &[Mag]| {
        norms
            .iter()
            .filter(|n| include_zero_norms || !n.is_zero())
            .fold(Mag::ONE, |p, &n| Mag::mul(p, n))
    };
    Mag::min(product(row_norms), product(column_norms))
        .sqrt()
        .ceiling()
}

impl IntegerMatrix {
    /// Returns an upper bound for the absolute value of the determinant of a square
    /// [`IntegerMatrix`], from Hadamard's inequality.
    ///
    /// Hadamard's inequality says that $|\det A|$ is at most the product of the Euclidean norms of
    /// the rows of $A$, and also at most the product of the Euclidean norms of its columns; the
    /// bound is the smaller of the two. The norms and their products are computed with
    /// floating-point magnitudes whose 30-bit mantissas are always rounded up, so the bound is
    /// rigorous but may exceed the exact Hadamard bound slightly. The bound is 0 if the matrix has
    /// a zero row or column, and 1 for the $0 \times 0$ matrix.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height()`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.determinant_bound(), 12u32);
    ///
    /// // The determinant is 25, which is also the exact Hadamard bound.
    /// let a = IntegerMatrix::from_str("[3, -4; 4, 3]").unwrap();
    /// assert_eq!(a.determinant_bound(), 26u32);
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 0, 0]").unwrap();
    /// assert_eq!(a.determinant_bound(), 0u32);
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_det_bound` from `fmpz_mat/det_bound.c`, FLINT 3.6.0, except
    /// that it panics if the matrix is not square.
    #[inline]
    pub fn determinant_bound(&self) -> Natural {
        assert!(self.height == self.width, "the matrix must be square");
        determinant_bound_helper(self, true)
    }
}
