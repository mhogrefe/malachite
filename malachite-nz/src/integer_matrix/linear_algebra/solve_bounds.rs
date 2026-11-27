// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2018, 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::fraction_matrix::RightHandSide;
use crate::mag::Mag;
use crate::natural::Natural;
use alloc::vec;
use core::cmp::min;
use malachite_base::num::basic::traits::One;

impl IntegerMatrix {
    /// Returns bounds $(N, D)$ for the solution of $AX = B$, where $A$ is a square
    /// [`IntegerMatrix`] and $B$ is an [`IntegerMatrix`] of the same height.
    ///
    /// If $A$ is nonsingular, every element of $X = A^{-1}B$, written as a fraction $p/q$ in lowest
    /// terms with $q > 0$, satisfies $|p| \leq N$ and $q \leq D$. By Cramer's rule, $\det A$ is a
    /// common denominator, and each numerator is the determinant of $A$ with one column replaced by
    /// a column of $B$. So $D$ bounds the smaller of the two Hadamard bounds for $|\det A|$, the
    /// product of the Euclidean norms of the rows of $A$ and the product of the Euclidean norms of
    /// its columns; and $N$ bounds the product of the column norms of $A$ times the largest
    /// Euclidean norm of a column of $B$. The norms are accumulated with a few bits of precision,
    /// rounding up. If $A$ is empty, both bounds are 1.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 + nk + nb)$
    ///
    /// $M(n, b) = O(n + nb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.height()`, $k$ is `b.width()`, and
    /// $b$ is the maximum number of significant bits of the elements of `self` and $B$.
    ///
    /// # Panics
    /// Panics if `self` is not square, or if its height is not the height of `b`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// // The solution is (-4, 9/2).
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = IntegerMatrix::from_str("[5; 6]").unwrap();
    /// let (n, d) = a.solve_bounds(&b);
    /// assert_eq!(n, 111u32);
    /// assert_eq!(d, 12u32);
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_solve_bound` from `fmpz_mat/solve_bound.c`, FLINT 3.7.0-dev,
    /// except that it panics if the dimensions are incompatible.
    #[inline]
    pub fn solve_bounds(&self, b: &Self) -> (Natural, Natural) {
        solve_bounds_helper(self, b.into())
    }
}

// This is equivalent to `fmpz_mat_solve_bound` from `fmpz_mat/solve_bound.c`, FLINT 3.7.0-dev,
// except that it panics if the dimensions are incompatible.
pub(crate) fn solve_bounds_helper(a: &IntegerMatrix, b: RightHandSide<'_>) -> (Natural, Natural) {
    assert!(a.height == a.width, "the matrix must be square");
    assert_eq!(
        a.height, b.height,
        "the height of the matrix must equal the height of the right-hand side"
    );
    let m = a.height;
    if m == 0 {
        return (Natural::ONE, Natural::ONE);
    }
    // The squared Euclidean norms of the rows of `a`, and the product of the norms of its columns.
    let mut row_norms = vec![Mag::ZERO; m];
    let mut column_bound = Mag::ONE;
    for j in 0..m {
        let mut u = Mag::ZERO;
        for (i, row_norm) in row_norms.iter_mut().enumerate() {
            let t = Mag::from(a.elements[i * m + j].unsigned_abs_ref());
            row_norm.add_mul_assign(t, t);
            u.add_mul_assign(t, t);
        }
        column_bound = Mag::mul(column_bound, u.sqrt());
    }
    let mut row_bound = Mag::ONE;
    for row_norm in row_norms {
        row_bound = Mag::mul(row_bound, row_norm.sqrt());
    }
    // Both Hadamard products bound |det(A)|.
    let denominator_bound = min(row_bound, column_bound).ceiling();
    // The largest squared Euclidean norm of a column of `b`.
    let mut max = Mag::ZERO;
    for j in 0..b.width {
        let mut t = Mag::ZERO;
        for i in 0..m {
            let x = Mag::from(b.elements[i * b.width + j].unsigned_abs_ref());
            t.add_mul_assign(x, x);
        }
        max = max.max(t);
    }
    // A numerator is the determinant of A with one column replaced by a column of B, so only the
    // column Hadamard bound applies to it.
    (
        Mag::mul(max.sqrt(), column_bound).ceiling(),
        denominator_bound,
    )
}
