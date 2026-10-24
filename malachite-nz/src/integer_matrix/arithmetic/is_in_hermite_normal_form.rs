// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014 Alex J. Best
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;

impl IntegerMatrix {
    /// Determines whether an [`IntegerMatrix`] is in Hermite normal form: whether its nonzero rows
    /// come before its zero rows, the first nonzero elements of the nonzero rows (the pivots) are
    /// positive and in increasing columns, and the elements above each pivot lie in $[0, p)$, where
    /// $p$ is the pivot.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hwb)$
    ///
    /// $M(h, w, b) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is the maximum number of significant bits of its elements: there are at most
    /// $\min(h, w)$ pivots, each compared with the elements above it.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// assert!(
    ///     IntegerMatrix::from_str("[1, 2; 0, 3]")
    ///         .unwrap()
    ///         .is_in_hermite_normal_form()
    /// );
    /// assert!(
    ///     IntegerMatrix::from_str("[1, 0; 0, 1; 0, 0]")
    ///         .unwrap()
    ///         .is_in_hermite_normal_form()
    /// );
    ///
    /// // An element above a pivot is not less than the pivot
    /// assert!(
    ///     !IntegerMatrix::from_str("[1, 3; 0, 3]")
    ///         .unwrap()
    ///         .is_in_hermite_normal_form()
    /// );
    /// // The pivots are not in increasing columns
    /// assert!(
    ///     !IntegerMatrix::from_str("[0, 1; 1, 0]")
    ///         .unwrap()
    ///         .is_in_hermite_normal_form()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_is_in_hnf` from `fmpz_mat/is_in_hnf.c`, FLINT 3.6.0.
    pub fn is_in_hermite_normal_form(&self) -> bool {
        let n = self.width;
        if n == 0 {
            return true;
        }
        let xs = &self.elements;
        let mut previous_pivot = None;
        let mut zero_row_seen = false;
        for (i, row) in xs.chunks_exact(n).enumerate() {
            let Some(j) = row.iter().position(|x| *x != 0u32) else {
                zero_row_seen = true;
                continue;
            };
            // A nonzero row must not follow a zero row, its pivot must be positive, and the pivots
            // must move right.
            let pivot = &row[j];
            if zero_row_seen || *pivot < 0u32 || previous_pivot.is_some_and(|p| j <= p) {
                return false;
            }
            previous_pivot = Some(j);
            // The elements above the pivot must lie in [0, pivot).
            if xs[..i * n]
                .chunks_exact(n)
                .any(|row_above| row_above[j] >= *pivot || row_above[j] < 0u32)
            {
                return false;
            }
        }
        true
    }
}
