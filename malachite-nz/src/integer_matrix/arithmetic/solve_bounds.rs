// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::determinant_bound::determinant_bound_helper;
use crate::integer_matrix::arithmetic::fraction_matrix::RightHandSide;
use crate::natural::Natural;
use alloc::vec;
use malachite_base::num::arithmetic::traits::{CeilingSqrt, Square};
use malachite_base::num::basic::traits::Zero;

impl IntegerMatrix {
    /// Returns bounds $(N, D)$ for the solution of $AX = B$, where $A$ is a square
    /// [`IntegerMatrix`] and $B$ is an [`IntegerMatrix`] of the same height.
    ///
    /// If $A$ is nonsingular, every element of $X = A^{-1}B$, written as a fraction $p/q$ in lowest
    /// terms with $q > 0$, satisfies $|p| \leq N$ and $q \leq D$. The denominator bound $D$ is
    /// [`determinant_bound`](IntegerMatrix::determinant_bound) of $A$, since $\det A$ is a common
    /// denominator, and $N$ is $D$ times the ceiling of the largest Euclidean norm of a column of
    /// $B$.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 + nkb \log b \log\log b)$
    ///
    /// $M(n, k, b) = O(n + b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.height()`, $k$ is `b.width()`, and
    /// $b$ is the maximum number of significant bits of the elements of $B$.
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
    /// assert_eq!(n, 96u32);
    /// assert_eq!(d, 12u32);
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_solve_bound` from `fmpz_mat/solve_bound.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible.
    #[inline]
    pub fn solve_bounds(&self, b: &Self) -> (Natural, Natural) {
        solve_bounds_helper(self, b.into())
    }
}

pub(crate) fn solve_bounds_helper(a: &IntegerMatrix, b: RightHandSide<'_>) -> (Natural, Natural) {
    assert!(a.height == a.width, "the matrix must be square");
    assert_eq!(
        a.height, b.height,
        "the height of the matrix must equal the height of the right-hand side"
    );
    let denominator_bound = determinant_bound_helper(a, true);
    // The squared Euclidean norms of the columns of `b`, accumulated row by row.
    let mut norms = vec![Natural::ZERO; b.width];
    if b.width != 0 {
        for row in b.elements.chunks_exact(b.width) {
            for (norm, x) in norms.iter_mut().zip(row) {
                *norm += x.unsigned_abs_ref().square();
            }
        }
    }
    let max = norms.into_iter().max().unwrap_or_default();
    (&denominator_bound * max.ceiling_sqrt(), denominator_bound)
}
