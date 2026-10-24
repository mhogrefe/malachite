// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::fraction_free_lu_decomposition::fflu_in_place_with_options;
use crate::integer_matrix::arithmetic::reduced_row_echelon_form::rref_in_place;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::Rank;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};

// The largest number of bits of the elements of a matrix whose rank is found with word-sized
// arithmetic: then, for elements x, y, and z of absolute value less than 2^30, xy + z fits in a
// signed 64-bit word. This is (FLINT_BITS - 4) / 2 for 64-bit FLINT_BITS.
const SMALL_RANK_BITS: u64 = 30;

// The bound on the absolute values of the elements during the word-sized elimination.
const SMALL_RANK_LIMIT: u64 = 1 << SMALL_RANK_BITS;

// The smallest dimension of the matrices whose rank is found from the reduced row echelon form
// rather than the fraction-free LU decomposition.
const RANK_RREF_MIN_DIMENSION: usize = 25;

// The rank of the `height` by `width` matrix with elements `xs`, by fraction-free LU decomposition
// when its smaller dimension is less than 25 and from its reduced row echelon form otherwise.
fn rank_general(xs: Vec<Integer>, height: usize, width: usize) -> usize {
    if height == 0 || width == 0 {
        return 0;
    }
    if height.min(width) < RANK_RREF_MIN_DIMENSION {
        let mut xs = xs;
        let mut perm: Vec<usize> = (0..height).collect();
        fflu_in_place_with_options(&mut xs, height, width, &mut perm, false, true).0
    } else {
        rref_in_place(&mut IntegerMatrix {
            elements: xs,
            height,
            width,
        })
        .0
    }
}

// The row in `start_row..end_row` whose element in column `c` is nonzero and smallest in absolute
// value, the first such row in case of a tie, or `None` if every element there is zero.
//
// This is equivalent to `fmpz_mat_find_pivot_smallest` from `fmpz_mat/find_pivot_smallest.c`, FLINT
// 3.6.0.
fn find_pivot_smallest(
    xs: &[i64],
    width: usize,
    start_row: usize,
    end_row: usize,
    c: usize,
) -> Option<usize> {
    let mut smallest: Option<(usize, u64)> = None;
    for r in start_row..end_row {
        let x = xs[r * width + c].unsigned_abs();
        if x != 0 && smallest.is_none_or(|(_, s)| x < s) {
            smallest = Some((r, x));
        }
    }
    smallest.map(|(r, _)| r)
}

// The rank of the `height` by `width` matrix with elements `xs`, all of absolute value less than
// 2^30, by elimination with word-sized integer quotients, in the manner of Euclid's algorithm: the
// pivot is the smallest nonzero element of its column, and each row below it is reduced by an
// integer multiple of the pivot row. If an element reaches 2^30 in absolute value, the rank of the
// remaining submatrix is found as in `rank_general`.
//
// This is equivalent to `fmpz_mat_rank_small_inplace` from `fmpz_mat/rank_small_inplace.c`, FLINT
// 3.6.0.
crate_test_fn! {
rank_small(mut xs: Vec<i64>, height: usize, width: usize) -> usize {
    if height == 0 || width == 0 {
        return 0;
    }
    let (m, n) = (height, width);
    let mut rank = 0;
    let mut pivot_row = 0;
    let mut pivot_col = 0;
    let mut r = find_pivot_smallest(&xs, n, pivot_row, m, pivot_col);
    while pivot_row < m && pivot_col < n {
        let Some(row) = r else {
            pivot_col += 1;
            if pivot_col == n {
                break;
            }
            r = find_pivot_smallest(&xs, n, pivot_row, m, pivot_col);
            continue;
        };
        if row != pivot_row {
            // The pivot is found at or below the pivot row, so `row` > `pivot_row`.
            let (upper, lower) = xs.split_at_mut(row * n);
            upper[pivot_row * n..(pivot_row + 1) * n].swap_with_slice(&mut lower[..n]);
        }
        let mut largest = 0;
        let pivot = xs[pivot_row * n + pivot_col];
        for j in pivot_row + 1..m {
            let x = xs[j * n + pivot_col];
            if x != 0 {
                let q = x / pivot;
                for k in pivot_col..n {
                    xs[j * n + k] -= q * xs[pivot_row * n + k];
                    largest |= xs[j * n + k].unsigned_abs();
                }
            }
        }
        if largest >= SMALL_RANK_LIMIT {
            // The rest may overflow; find the rank of the remaining submatrix with Integers.
            let mut window = Vec::with_capacity((m - pivot_row) * (n - pivot_col));
            for row in xs.chunks_exact(n).skip(pivot_row) {
                window.extend(row[pivot_col..].iter().map(|&x| Integer::from(x)));
            }
            return rank + rank_general(window, m - pivot_row, n - pivot_col);
        }
        r = find_pivot_smallest(&xs, n, pivot_row + 1, m, pivot_col);
        if r.is_none() {
            pivot_row += 1;
            pivot_col += 1;
            rank += 1;
            if pivot_row < m && pivot_col < n {
                r = find_pivot_smallest(&xs, n, pivot_row, m, pivot_col);
            }
        }
    }
    rank
}}

// The rank of a matrix: with word-sized arithmetic when its elements have at most 30 bits, by
// fraction-free LU decomposition when its smaller dimension is less than 25, and from its reduced
// row echelon form otherwise.
//
// This is equivalent to `fmpz_mat_rank` from `fmpz_mat/rank.c`, FLINT 3.6.0.
fn rank_helper(a: &IntegerMatrix) -> u64 {
    let (m, n) = (a.height, a.width);
    let rank = if m == 0 || n == 0 {
        0
    } else if vec_max_bits(&a.elements).0 <= SMALL_RANK_BITS {
        rank_small(a.elements.iter().map(i64::exact_from).collect(), m, n)
    } else {
        rank_general(a.elements.clone(), m, n)
    };
    u64::wrapping_from(rank)
}

impl Rank for IntegerMatrix {
    /// Computes the rank of an [`IntegerMatrix`]: the number of linearly independent rows, or
    /// equivalently columns, taking the matrix by value.
    ///
    /// When every element has at most 30 bits, the rank is found by elimination with word-sized
    /// integer quotients, in the manner of Euclid's algorithm, switching to the methods below if
    /// the elements grow too large; otherwise, it is found by fraction-free LU decomposition when
    /// the smaller dimension is less than 25, and from the reduced row echelon form otherwise.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^4 (b + \log n) \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: the elements
    /// of the form have $O(n(b + \log n))$ bits, and the multimodular method solves a nonsingular
    /// system within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Rank;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.clone().rank(), 2);
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.clone().rank(), 2);
    ///
    /// let a = IntegerMatrix::from_str("[2, 1; 4, 2; 6, 3]").unwrap();
    /// assert_eq!(a.clone().rank(), 1);
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_rank` from `fmpz_mat/rank.c`, FLINT 3.6.0.
    #[inline]
    fn rank(self) -> u64 {
        rank_helper(&self)
    }
}

impl Rank for &IntegerMatrix {
    /// Computes the rank of an [`IntegerMatrix`]: the number of linearly independent rows, or
    /// equivalently columns, taking the matrix by reference.
    ///
    /// When every element has at most 30 bits, the rank is found by elimination with word-sized
    /// integer quotients, in the manner of Euclid's algorithm, switching to the methods below if
    /// the elements grow too large; otherwise, it is found by fraction-free LU decomposition when
    /// the smaller dimension is less than 25, and from the reduced row echelon form otherwise.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^4 (b + \log n) \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: the elements
    /// of the form have $O(n(b + \log n))$ bits, and the multimodular method solves a nonsingular
    /// system within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Rank;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).rank(), 2);
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).rank(), 2);
    ///
    /// let a = IntegerMatrix::from_str("[2, 1; 4, 2; 6, 3]").unwrap();
    /// assert_eq!((&a).rank(), 1);
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_rank` from `fmpz_mat/rank.c`, FLINT 3.6.0.
    #[inline]
    fn rank(self) -> u64 {
        rank_helper(self)
    }
}
