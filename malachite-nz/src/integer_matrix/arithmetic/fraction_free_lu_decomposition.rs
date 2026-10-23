// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::natural_matrix::arithmetic::mod_determinant::permutation_is_odd;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    DivExactAssign, FractionFreeLuDecomposition, ModPowerOf2Inverse, MulSubMulAssign, NegAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};

// The first row in `start_row..end_row` of the `width`-column matrix with elements `xs` whose
// element in column `c` is nonzero, or `None` if there is none.
//
// This is equivalent to `fmpz_mat_find_pivot_any` from `fmpz_mat/find_pivot_any.c`, FLINT 3.6.0.
pub(crate) fn find_pivot_any(
    xs: &[Integer],
    width: usize,
    start_row: usize,
    end_row: usize,
    c: usize,
) -> Option<usize> {
    (start_row..end_row).find(|&r| xs[r * width + c] != 0u32)
}

// The state of a fraction-free elimination in progress: the rank so far, the position of the next
// pivot, and the denominator, the last pivot (or 1).
struct FfluState {
    rank: usize,
    pivot_row: usize,
    pivot_col: usize,
    den: Integer,
    den_is_one: bool,
}

impl FfluState {
    const fn new() -> Self {
        Self {
            rank: 0,
            pivot_row: 0,
            pivot_col: 0,
            den: Integer::ONE,
            den_is_one: false,
        }
    }
}

// Swaps rows `i` and `j` of the `width`-column matrix with elements `xs`, where `i < j`, and the
// corresponding elements of `perm`.
fn swap_rows<T>(xs: &mut [T], width: usize, perm: &mut [usize], i: usize, j: usize) {
    let (top, bottom) = xs.split_at_mut(j * width);
    top[i * width..(i + 1) * width].swap_with_slice(&mut bottom[..width]);
    perm.swap(i, j);
}

// Continues the elimination of `fflu_in_place_with_options` from `state`, with `Integer` elements.
//
// This is the general path of `fmpz_mat_fflu` from `fmpz_mat/fflu.c`, FLINT 3.6.0.
fn fflu_general(
    xs: &mut [Integer],
    height: usize,
    width: usize,
    perm: &mut [usize],
    rank_check: bool,
    state: FfluState,
) -> (usize, Integer) {
    let FfluState {
        mut rank,
        mut pivot_row,
        mut pivot_col,
        mut den,
        mut den_is_one,
    } = state;
    while pivot_row < height && pivot_col < width {
        let Some(r) = find_pivot_any(xs, width, pivot_row, height, pivot_col) else {
            if rank_check {
                return (0, Integer::ZERO);
            }
            pivot_col += 1;
            continue;
        };
        if r != pivot_row {
            swap_rows(xs, width, perm, pivot_row, r);
        }
        rank += 1;
        let (top, bottom) = xs.split_at_mut((pivot_row + 1) * width);
        let pivot_elements = &top[pivot_row * width..];
        let pivot = &pivot_elements[pivot_col];
        for row in bottom.chunks_exact_mut(width) {
            // With a previous pivot of 1 and a pivot of 1, a row with a zero in the pivot column is
            // unchanged.
            if den_is_one && row[pivot_col] == 0u32 && *pivot == 1u32 {
                continue;
            }
            let (left, right) = row.split_at_mut(pivot_col + 1);
            let multiplier = &left[pivot_col];
            for (x, y) in right.iter_mut().zip(&pivot_elements[pivot_col + 1..]) {
                x.mul_sub_mul_assign(pivot, multiplier, y);
                if pivot_row > 0 && !den_is_one {
                    x.div_exact_assign(&den);
                }
            }
        }
        den = pivot.clone();
        den_is_one = den == 1u32;
        pivot_row += 1;
        pivot_col += 1;
    }
    (rank, den)
}

// The largest absolute value of an element on the small-element path: FLINT's `COEFF_MAX`, the
// largest absolute value of a small `fmpz`.
pub(crate) const SMALL_MAX: i64 = (1 << 62) - 1;

// `SMALL_MAX` as a `u128`, for comparing with the absolute values of double-word products.
pub(crate) const SMALL_MAX_U128: u128 = SMALL_MAX.unsigned_abs() as u128;

// The smallest height and width of a matrix for which the elimination starts on the small-element
// path; for smaller matrices, converting the elements costs more than the path saves.
pub(crate) const SMALL_PATH_MIN_DIMENSION: usize = 3;

// The smallest dimension of a matrix for which the determinant starts on the small-element path.
// Below it the path is slower, and determinants of such matrices are better computed by cofactor
// expansion anyway.
const DETERMINANT_SMALL_PATH_MIN_DIMENSION: usize = 5;

// How the small-element path ended.
enum SmallOutcome {
    // The elimination finished, with the given rank and denominator.
    Finished {
        rank: usize,
        den: i64,
    },
    // The rank check found that the matrix does not have full rank.
    NotFullRank,
    // A step produced elements too large for the path. The step is complete: the large elements,
    // with their indices, are in `large` (their places in the small elements hold 0), and `state`
    // is where the elimination should continue.
    TooLarge {
        state: FfluState,
        large: Vec<(usize, i128)>,
    },
}

// The elements of `xs` as `i64`s, if they are all at most `SMALL_MAX` in absolute value.
pub(crate) fn small_elements(xs: &[Integer]) -> Option<Vec<i64>> {
    xs.iter()
        .map(|x| {
            i64::try_from(x)
                .ok()
                .filter(|y| y.unsigned_abs() <= SMALL_MAX.unsigned_abs())
        })
        .collect()
}

// Writes the small elements `ys`, and the large elements `large` at their indices, to `xs`.
fn write_back(xs: &mut [Integer], ys: &[i64], large: &[(usize, i128)]) {
    for (x, &y) in xs.iter_mut().zip(ys) {
        *x = Integer::from(y);
    }
    for &(index, q) in large {
        xs[index] = Integer::from(q);
    }
}

// Runs the elimination of `fflu_in_place_with_options` with the elements in `ys`, which are at most
// `SMALL_MAX` in absolute value, for as long as they stay that small.
//
// This is the small-element path of `fmpz_mat_fflu` from `fmpz_mat/fflu.c`, FLINT 3.6.0, which
// computes each new element as a double-word product and divides it exactly by the previous pivot.
// Here the exact division is a multiplication by an inverse modulo 2^64.
fn fflu_small(
    ys: &mut [i64],
    height: usize,
    width: usize,
    perm: &mut [usize],
    rank_check: bool,
) -> SmallOutcome {
    let mut rank = 0;
    let mut pivot_row = 0;
    let mut pivot_col = 0;
    let mut den: i64 = 1;
    let mut den_is_one = false;
    // The elements of the current step that are too large, with their indices.
    let mut large: Vec<(usize, i128)> = Vec::new();
    while pivot_row < height && pivot_col < width {
        let Some(r) = (pivot_row..height).find(|&i| ys[i * width + pivot_col] != 0) else {
            if rank_check {
                return SmallOutcome::NotFullRank;
            }
            pivot_col += 1;
            continue;
        };
        if r != pivot_row {
            swap_rows(ys, width, perm, pivot_row, r);
        }
        rank += 1;
        let divide = pivot_row > 0 && !den_is_one;
        // An exact quotient by `den` is (x / 2^shift) times the inverse of den / 2^shift modulo
        // 2^64, if it fits in a word: if |x| <= SMALL_MAX * |den|.
        let shift = den.trailing_zeros();
        let inverse = (den >> shift)
            .cast_unsigned()
            .mod_power_of_2_inverse(u64::WIDTH)
            .unwrap();
        let quotient_bound = u128::from(SMALL_MAX.unsigned_abs()) * u128::from(den.unsigned_abs());
        let (top, bottom) = ys.split_at_mut((pivot_row + 1) * width);
        let pivot_elements = &top[pivot_row * width..];
        let pivot = pivot_elements[pivot_col];
        for (i, row) in bottom.chunks_exact_mut(width).enumerate() {
            let multiplier = row[pivot_col];
            // With a previous pivot of 1 and a pivot of 1, a row with a zero in the pivot column is
            // unchanged.
            if den_is_one && multiplier == 0 && pivot == 1 {
                continue;
            }
            let row_start = (pivot_row + 1 + i) * width;
            for (j, (x, &y)) in row
                .iter_mut()
                .zip(pivot_elements)
                .enumerate()
                .skip(pivot_col + 1)
            {
                let product =
                    i128::from(*x) * i128::from(pivot) - i128::from(multiplier) * i128::from(y);
                if !divide {
                    if product.unsigned_abs() <= SMALL_MAX_U128 {
                        *x = i64::wrapping_from(product);
                    } else {
                        *x = 0;
                        large.push((row_start + j, product));
                    }
                } else if product.unsigned_abs() <= quotient_bound {
                    *x = u64::wrapping_from(product >> shift)
                        .wrapping_mul(inverse)
                        .cast_signed();
                } else {
                    *x = 0;
                    large.push((row_start + j, product / i128::from(den)));
                }
            }
        }
        den = pivot;
        den_is_one = den == 1;
        pivot_row += 1;
        pivot_col += 1;
        if !large.is_empty() {
            return SmallOutcome::TooLarge {
                state: FfluState {
                    rank,
                    pivot_row,
                    pivot_col,
                    den: Integer::from(den),
                    den_is_one,
                },
                large,
            };
        }
    }
    SmallOutcome::Finished { rank, den }
}

// Replaces the `height` by `width` matrix with elements `xs` with its packed fraction-free LU
// decomposition, applies the row swaps to `perm`, and returns the rank and the denominator, the
// last pivot (or 1 if there are no pivots). Below each pivot are the elements of L, and in each
// pivot's row, from the pivot rightward, are the elements of U. If `rank_check` is true and the
// matrix is found not to have full rank, the elimination stops and (0, 0) is returned. If
// `allow_small` is true, the matrix is not too small, and every element is at most `SMALL_MAX` in
// absolute value, the elimination starts with word-sized arithmetic.
//
// This is equivalent to `fmpz_mat_fflu` from `fmpz_mat/fflu.c`, FLINT 3.6.0.
pub(crate) fn fflu_in_place_with_options(
    xs: &mut [Integer],
    height: usize,
    width: usize,
    perm: &mut [usize],
    rank_check: bool,
    allow_small: bool,
) -> (usize, Integer) {
    if height == 0 || width == 0 {
        return (0, Integer::ONE);
    }
    if allow_small
        && height.min(width) >= SMALL_PATH_MIN_DIMENSION
        && let Some(mut ys) = small_elements(xs)
    {
        return match fflu_small(&mut ys, height, width, perm, rank_check) {
            SmallOutcome::Finished { rank, den } => {
                write_back(xs, &ys, &[]);
                (rank, Integer::from(den))
            }
            SmallOutcome::NotFullRank => {
                write_back(xs, &ys, &[]);
                (0, Integer::ZERO)
            }
            SmallOutcome::TooLarge { state, large } => {
                write_back(xs, &ys, &large);
                fflu_general(xs, height, width, perm, rank_check, state)
            }
        };
    }
    fflu_general(xs, height, width, perm, rank_check, FfluState::new())
}

// The determinant of the `n` by `n` matrix with elements `xs`, where `n` is positive: the last
// pivot of its fraction-free LU decomposition with the rank check, negated if the row permutation
// is odd. On the small-element path, the elements are never copied back to `Integer`s unless they
// outgrow it. If `allow_small` is false, only `Integer` arithmetic is used.
//
// This is equivalent to `_fmpz_mat_det_bareiss` from `fmpz_mat/det_bareiss.c`, FLINT 3.6.0.
pub(crate) fn fflu_determinant(xs: &[Integer], n: usize, allow_small: bool) -> Integer {
    let mut perm: Vec<usize> = (0..n).collect();
    let small = if allow_small && n >= DETERMINANT_SMALL_PATH_MIN_DIMENSION {
        small_elements(xs)
    } else {
        None
    };
    let mut det = if let Some(mut ys) = small {
        match fflu_small(&mut ys, n, n, &mut perm, true) {
            SmallOutcome::Finished { den, .. } => Integer::from(den),
            SmallOutcome::NotFullRank => return Integer::ZERO,
            SmallOutcome::TooLarge { state, large } => {
                let mut xs = vec![Integer::ZERO; n * n];
                write_back(&mut xs, &ys, &large);
                fflu_general(&mut xs, n, n, &mut perm, true, state).1
            }
        }
    } else {
        let mut xs = xs.to_vec();
        fflu_general(&mut xs, n, n, &mut perm, true, FfluState::new()).1
    };
    if permutation_is_odd(&perm) {
        det.neg_assign();
    }
    det
}

// The fraction-free LU decomposition of `a`, with the small-element path allowed if `allow_small`
// is true.
pub(crate) fn fraction_free_lu_decomposition_helper(
    mut a: IntegerMatrix,
    allow_small: bool,
) -> (Vec<u64>, IntegerMatrix, IntegerMatrix, Integer) {
    let (height, width) = (a.height, a.width);
    let mut perm: Vec<usize> = (0..height).collect();
    let (rank, den) = fflu_in_place_with_options(
        &mut a.elements,
        height,
        width,
        &mut perm,
        false,
        allow_small,
    );
    let xs = &mut a.elements;
    // The pivot columns.
    let mut pivots = Vec::with_capacity(rank);
    let mut col = 0;
    for k in 0..rank {
        while xs[k * width + col] == 0u32 {
            col += 1;
        }
        pivots.push(col);
        col += 1;
    }
    let mut l = Vec::with_capacity(height * rank);
    for i in 0..height {
        for (k, &p) in pivots.iter().enumerate() {
            l.push(match i.cmp(&k) {
                core::cmp::Ordering::Greater => core::mem::take(&mut xs[i * width + p]),
                core::cmp::Ordering::Equal => xs[i * width + p].clone(),
                core::cmp::Ordering::Less => Integer::ZERO,
            });
        }
    }
    let mut u = Vec::with_capacity(rank * width);
    for (k, &p) in pivots.iter().enumerate() {
        for j in 0..width {
            u.push(if j >= p {
                core::mem::take(&mut xs[k * width + j])
            } else {
                Integer::ZERO
            });
        }
    }
    (
        perm.into_iter().map(u64::exact_from).collect(),
        IntegerMatrix {
            elements: l,
            height,
            width: rank,
        },
        IntegerMatrix {
            elements: u,
            height: rank,
            width,
        },
        den,
    )
}

impl FractionFreeLuDecomposition for IntegerMatrix {
    type Output = (Vec<u64>, Self, Self, Integer);

    /// Computes a fraction-free LU decomposition of an [`IntegerMatrix`], taking it by value.
    ///
    /// For an $h \times w$ matrix $A$ of rank $r$, the result is $(P, L, U, d)$, where:
    /// - $P$ is a permutation of the rows, given as the list whose $i$th element is the index of
    ///   the row of $A$ that becomes row $i$;
    /// - $U$ is an $r \times w$ matrix in row echelon form, whose first nonzero elements, the
    ///   pivots, are $p_0, p_1, \ldots, p_{r-1}$;
    /// - $L$ is an $h \times r$ lower triangular matrix whose diagonal elements are the same
    ///   pivots;
    /// - $d$ is the last pivot, $p_{r-1}$, or 1 if $r = 0$;
    ///
    /// and $PA = LD^{-1}U$, where $D = \operatorname{diag}(p_0, p_0p_1, p_1p_2, \ldots,
    /// p_{r-2}p_{r-1})$. Every element of $L$ and $U$ is an integer, the determinant of a submatrix
    /// of $PA$ (Bareiss's elimination keeps every division exact), and $d$ is, up to sign, the
    /// determinant of the $r \times r$ submatrix of $A$ in the pivot rows and columns; for a square
    /// nonsingular $A$, $d = \pm\det A$. The rank $r$ is the width of $L$. Pivots are chosen as the
    /// first nonzero elements in their columns.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw\min(h, w) n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $n$ is $\min(h, w)$ times the maximum number of significant bits of its elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use itertools::Itertools;
    /// use malachite_base::num::arithmetic::traits::FractionFreeLuDecomposition;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let (p, l, u, d) = a.fraction_free_lu_decomposition();
    /// assert_eq!(p.iter().join(", "), "0, 1");
    /// assert_eq!(l.to_string(), "[1, 0; 3, -2]");
    /// assert_eq!(u.to_string(), "[1, 2; 0, -2]");
    /// assert_eq!(d.to_string(), "-2");
    ///
    /// // A matrix of rank 2
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// let (p, l, u, d) = a.fraction_free_lu_decomposition();
    /// assert_eq!(p.iter().join(", "), "0, 2, 1");
    /// assert_eq!(l.to_string(), "[1, 0; 1, -2; 2, 0]");
    /// assert_eq!(u.to_string(), "[1, 2, 3; 0, -2, -2]");
    /// assert_eq!(d.to_string(), "-2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_fflu` from `fmpz_mat/fflu.c`, FLINT 3.6.0, without the rank
    /// check, except that $L$ and $U$ are returned separately rather than packed into one matrix,
    /// and the rank is the width of $L$.
    #[inline]
    fn fraction_free_lu_decomposition(self) -> (Vec<u64>, Self, Self, Integer) {
        fraction_free_lu_decomposition_helper(self, true)
    }
}

impl FractionFreeLuDecomposition for &IntegerMatrix {
    type Output = (Vec<u64>, IntegerMatrix, IntegerMatrix, Integer);

    /// Computes a fraction-free LU decomposition of an [`IntegerMatrix`], taking it by reference.
    ///
    /// For an $h \times w$ matrix $A$ of rank $r$, the result is $(P, L, U, d)$, where:
    /// - $P$ is a permutation of the rows, given as the list whose $i$th element is the index of
    ///   the row of $A$ that becomes row $i$;
    /// - $U$ is an $r \times w$ matrix in row echelon form, whose first nonzero elements, the
    ///   pivots, are $p_0, p_1, \ldots, p_{r-1}$;
    /// - $L$ is an $h \times r$ lower triangular matrix whose diagonal elements are the same
    ///   pivots;
    /// - $d$ is the last pivot, $p_{r-1}$, or 1 if $r = 0$;
    ///
    /// and $PA = LD^{-1}U$, where $D = \operatorname{diag}(p_0, p_0p_1, p_1p_2, \ldots,
    /// p_{r-2}p_{r-1})$. Every element of $L$ and $U$ is an integer, the determinant of a submatrix
    /// of $PA$ (Bareiss's elimination keeps every division exact), and $d$ is, up to sign, the
    /// determinant of the $r \times r$ submatrix of $A$ in the pivot rows and columns; for a square
    /// nonsingular $A$, $d = \pm\det A$. The rank $r$ is the width of $L$. Pivots are chosen as the
    /// first nonzero elements in their columns.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw\min(h, w) n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $n$ is $\min(h, w)$ times the maximum number of significant bits of its elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use itertools::Itertools;
    /// use malachite_base::num::arithmetic::traits::FractionFreeLuDecomposition;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let (p, l, u, d) = (&a).fraction_free_lu_decomposition();
    /// assert_eq!(p.iter().join(", "), "0, 1");
    /// assert_eq!(l.to_string(), "[1, 0; 3, -2]");
    /// assert_eq!(u.to_string(), "[1, 2; 0, -2]");
    /// assert_eq!(d.to_string(), "-2");
    ///
    /// // A matrix of rank 2
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// let (p, l, u, d) = (&a).fraction_free_lu_decomposition();
    /// assert_eq!(p.iter().join(", "), "0, 2, 1");
    /// assert_eq!(l.to_string(), "[1, 0; 1, -2; 2, 0]");
    /// assert_eq!(u.to_string(), "[1, 2, 3; 0, -2, -2]");
    /// assert_eq!(d.to_string(), "-2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_fflu` from `fmpz_mat/fflu.c`, FLINT 3.6.0, without the rank
    /// check, except that $L$ and $U$ are returned separately rather than packed into one matrix,
    /// and the rank is the width of $L$.
    #[inline]
    fn fraction_free_lu_decomposition(self) -> (Vec<u64>, IntegerMatrix, IntegerMatrix, Integer) {
        fraction_free_lu_decomposition_helper(self.clone(), true)
    }
}
