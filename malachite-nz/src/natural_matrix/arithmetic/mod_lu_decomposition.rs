// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011, 2022 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::mul::{ProductShape, matrix_mul};
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_solve_lower_triangular::{NaturalBlockRef, solve_lower};
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, DivModPrecomputed, ModInverse, ModIsReduced, ModLuDecomposition, ModMul,
    ModSubAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;

// A block of a matrix that LU decomposition works on in place: `xs` starts at the block's top-left
// element, and row `i` of the block is `xs[i * stride..i * stride + width]`.
pub(crate) struct NaturalLuBlock<'a> {
    pub(crate) xs: &'a mut [Natural],
    pub(crate) stride: usize,
    pub(crate) height: usize,
    pub(crate) width: usize,
}

impl NaturalLuBlock<'_> {
    #[inline]
    const fn get(&self, i: usize, j: usize) -> &Natural {
        &self.xs[i * self.stride + j]
    }

    #[inline]
    fn set(&mut self, i: usize, j: usize, x: Natural) {
        self.xs[i * self.stride + j] = x;
    }

    // Rows `i` and `j` of the block, where `i < j`.
    fn two_rows(&mut self, i: usize, j: usize) -> (&mut [Natural], &mut [Natural]) {
        let (top, bottom) = self.xs.split_at_mut(j * self.stride);
        (
            &mut top[i * self.stride..i * self.stride + self.width],
            &mut bottom[..self.width],
        )
    }

    // This is equivalent to `_gr_mat_swap_rows` from `gr_mat.h`, FLINT 3.6.0.
    fn swap_rows(&mut self, perm: &mut [usize], r: usize, s: usize) {
        if r != s && self.width != 0 {
            perm.swap(r, s);
            let (r, s) = if r < s { (r, s) } else { (s, r) };
            let (x, y) = self.two_rows(r, s);
            x.swap_with_slice(y);
        }
    }
}

// Computes the LU decomposition of `a` modulo `m` in place with the classical algorithm, and
// returns its rank, or `None` if a pivot is not invertible modulo `m`. The layout is that of `lu`.
// The reductions are delayed: each update adds (m - e) times a reduced element without reducing,
// and each column, and each new pivot row, is reduced once, just before it is used.
//
// This is equivalent to `gr_mat_lu_classical` from `gr_mat/lu_classical.c`, FLINT 3.6.0, with
// `gr_mat_find_nonzero_pivot_generic` choosing the pivots and `None` standing for a status other
// than `GR_SUCCESS`, except that FLINT reduces each update immediately. The results are the same,
// since every element is reduced before it is read.
pub(crate) fn lu_classical(
    perm: &mut [usize],
    a: &mut NaturalLuBlock<'_>,
    m: &Natural,
    rank_check: bool,
) -> Option<usize> {
    let (height, width) = (a.height, a.width);
    for (i, p) in perm[..height].iter_mut().enumerate() {
        *p = i;
    }
    if height == 0 || width == 0 {
        return Some(0);
    }
    let data = Natural::precompute_div_mod_data(m);
    let reduce = |x: &mut Natural| {
        if *x >= *m {
            *x = (&*x).div_mod_precomputed(m, &data).1;
        }
    };
    let mut rank = 0;
    let mut row = 0;
    let mut col = 0;
    while row < height && col < width {
        // Reduce the current column. This can be skipped on the first iteration.
        if col != 0 {
            for j in row..height {
                reduce(&mut a.xs[j * a.stride + col]);
            }
        }
        let Some(r) = (row..height).find(|&i| *a.get(i, col) != 0u32) else {
            if rank_check {
                return Some(0);
            }
            col += 1;
            continue;
        };
        a.swap_rows(perm, row, r);
        // Reduce the current pivot row.
        if col != 0 {
            for j in col + 1..width {
                reduce(&mut a.xs[row * a.stride + j]);
            }
        }
        rank += 1;
        let d = a.get(row, col).mod_inverse(m)?;
        for i in row + 1..height {
            let e = a.get(i, col).mod_mul(&d, m);
            if e != 0u32 {
                let f = m - &e;
                let (pivot_row, other_row) = a.two_rows(row, i);
                for (x, y) in other_row[col + 1..].iter_mut().zip(&pivot_row[col + 1..]) {
                    x.add_mul_assign(&f, y);
                }
            }
            a.set(i, col, Natural::ZERO);
            a.set(i, rank - 1, e);
        }
        row += 1;
        col += 1;
    }
    Some(rank)
}

// Applies the row permutation `p` to the `num_rows` rows of `a` starting at `row_offset`, in the
// `num_cols` columns starting at `col_offset`, and composes it with the permutation `ap`.
//
// This is equivalent to `_apply_permutation` from `gr_mat/lu_recursive.c`, FLINT 3.6.0.
fn apply_permutation(
    ap: &mut [usize],
    a: &mut NaturalLuBlock<'_>,
    p: &[usize],
    num_rows: usize,
    row_offset: usize,
    num_cols: usize,
    col_offset: usize,
) {
    if num_rows == 0 {
        return;
    }
    let stride = a.stride;
    let mut tmp = Vec::with_capacity(num_rows * num_cols);
    for &pi in &p[..num_rows] {
        let start = (pi + row_offset) * stride + col_offset;
        for x in &mut a.xs[start..start + num_cols] {
            tmp.push(core::mem::take(x));
        }
    }
    if num_cols != 0 {
        for (i, row) in tmp.chunks_exact_mut(num_cols).enumerate() {
            let start = (i + row_offset) * stride + col_offset;
            for (x, y) in a.xs[start..start + num_cols].iter_mut().zip(row) {
                *x = core::mem::take(y);
            }
        }
    }
    let ap_tmp: Vec<usize> = p[..num_rows]
        .iter()
        .map(|&pi| ap[pi + row_offset])
        .collect();
    ap[row_offset..row_offset + num_rows].copy_from_slice(&ap_tmp);
}

// Computes the LU decomposition of `a` modulo `m` in place with the recursive algorithm, and
// returns its rank, or `None` if a pivot is not invertible modulo `m`.
//
// This is equivalent to `gr_mat_lu_recursive` from `gr_mat/lu_recursive.c`, FLINT 3.6.0, with the
// blocks that FLINT references through windows copied where they are read alongside an overlapping
// block, and with the product of blocks computed exactly and reduced once, as `fmpz_mod_mat_mul`
// computes it.
pub(crate) fn lu_recursive(
    perm: &mut [usize],
    a: &mut NaturalLuBlock<'_>,
    m: &Natural,
    rank_check: bool,
) -> Option<usize> {
    let (height, width, stride) = (a.height, a.width, a.stride);
    if height <= 1 || width <= 1 {
        return lu_classical(perm, a, m, rank_check);
    }
    let n1 = width >> 1;
    for (i, p) in perm[..height].iter_mut().enumerate() {
        *p = i;
    }
    let mut p1 = vec![0; height];
    let r1 = lu(
        &mut p1,
        &mut NaturalLuBlock {
            xs: a.xs,
            stride,
            height,
            width: n1,
        },
        m,
        rank_check,
    )?;
    if rank_check && r1 != n1 {
        return Some(0);
    }
    if r1 != 0 {
        apply_permutation(perm, a, &p1, height, 0, width - n1, n1);
    }
    // A00 = rows 0..r1, columns 0..r1; A10 = rows r1..height, columns 0..r1; A01 = rows 0..r1,
    // columns n1..width; A11 = rows r1..height, columns n1..width.
    let w2 = width - n1;
    let h2 = height - r1;
    if r1 != 0 {
        let whole = NaturalBlockRef {
            xs: a.xs,
            width: stride,
            row: 0,
            column: 0,
        };
        let mut a01 = whole.copy(0, r1, n1, w2);
        solve_lower(&mut a01, whole, r1, w2, m, true);
        let a10 = whole.copy(r1, h2, 0, r1);
        let product = matrix_mul(
            &a10,
            &a01,
            ProductShape {
                ar: h2,
                br: r1,
                bc: w2,
            },
        );
        for (i, a01_row) in a01.chunks_exact_mut(w2).enumerate() {
            for (x, y) in a.xs[i * stride + n1..i * stride + width]
                .iter_mut()
                .zip(a01_row)
            {
                *x = core::mem::take(y);
            }
        }
        if w2 != 0 {
            let data = Natural::precompute_div_mod_data(m);
            for (i, p_row) in product.chunks_exact(w2).enumerate() {
                let start = (r1 + i) * stride + n1;
                for (x, p) in a.xs[start..start + w2].iter_mut().zip(p_row) {
                    x.mod_sub_assign(p.div_mod_precomputed(m, &data).1, m);
                }
            }
        }
    }
    let r2 = if h2 == 0 {
        0
    } else {
        lu(
            &mut p1,
            &mut NaturalLuBlock {
                xs: &mut a.xs[r1 * stride + n1..],
                stride,
                height: h2,
                width: w2,
            },
            m,
            rank_check,
        )?
    };
    if rank_check && r1 + r2 < height.min(width) {
        return Some(0);
    }
    apply_permutation(perm, a, &p1, h2, r1, n1, 0);
    // Compress L.
    if r1 != n1 {
        for i in 0..h2 {
            let row = &mut a.xs[(r1 + i) * stride..(r1 + i) * stride + width];
            for j in 0..i.min(r2) {
                row[r1 + j] = core::mem::take(&mut row[n1 + j]);
            }
        }
    }
    Some(r1 + r2)
}

// Below this many rows or columns, LU decomposition uses the classical algorithm.
//
// This is the cutoff from `_gr_fmpz_mod_mat_lu` in `gr/fmpz_mod.c`, FLINT 3.6.0.
const LU_CUTOFF: usize = 8;

// Computes the LU decomposition of the block `a` modulo `m` in place, and returns its rank, or
// `None` if a pivot is not invertible modulo `m`, which can only happen if `m` is composite.
// Afterwards, `perm[i]` is the row of the original block that ended up in row `i`; the rows of the
// block hold U on and above the diagonal and the multipliers of L, whose diagonal elements are 1,
// below it, compressed into the first `rank` columns. If `rank_check` is `true`, the decomposition
// stops and 0 is returned as soon as the block is found not to have full rank.
//
// This is equivalent to `fmpz_mod_mat_lu` from `fmpz_mod_mat/lu.c`, FLINT 3.6.0, which calls
// `_gr_fmpz_mod_mat_lu` from `gr/fmpz_mod.c`.
pub(crate) fn lu(
    perm: &mut [usize],
    a: &mut NaturalLuBlock<'_>,
    m: &Natural,
    rank_check: bool,
) -> Option<usize> {
    if a.height < LU_CUTOFF || a.width < LU_CUTOFF {
        lu_classical(perm, a, m, rank_check)
    } else {
        lu_recursive(perm, a, m, rank_check)
    }
}

pub(crate) fn assert_reduced(a: &NaturalMatrix, m: &Natural) {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert!(
        a.mod_is_reduced(m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

// Computes the packed LU decomposition of `a` modulo `m` in place, returning the permutation and
// the rank, or `None` if a pivot is not invertible.
pub(crate) fn lu_in_place(
    a: &mut NaturalMatrix,
    m: &Natural,
    rank_check: bool,
) -> Option<(Vec<usize>, usize)> {
    let mut perm = vec![0; a.height];
    let rank = lu(
        &mut perm,
        &mut NaturalLuBlock {
            xs: &mut a.elements,
            stride: a.width,
            height: a.height,
            width: a.width,
        },
        m,
        rank_check,
    )?;
    Some((perm, rank))
}

// The panic for a pivot that is not invertible.
pub(crate) fn non_invertible_pivot(m: &Natural) -> ! {
    panic!("a pivot is not invertible mod {m}, so {m} is not prime")
}

fn mod_lu_decomposition_helper(
    mut a: NaturalMatrix,
    m: &Natural,
) -> (Vec<u64>, NaturalMatrix, NaturalMatrix) {
    assert_reduced(&a, m);
    if let Natural(Small(small_m)) = *m {
        let (p, l, u) = to_limb_matrix(&a).mod_lu_decomposition(small_m);
        return (p, NaturalMatrix::from(l), NaturalMatrix::from(u));
    }
    let (perm, rank) = lu_in_place(&mut a, m, false).unwrap_or_else(|| non_invertible_pivot(m));
    let (height, width) = (a.height, a.width);
    let mut l = Vec::with_capacity(height * rank);
    for i in 0..height {
        for c in 0..rank {
            l.push(match i.cmp(&c) {
                core::cmp::Ordering::Greater => core::mem::take(&mut a.elements[i * width + c]),
                core::cmp::Ordering::Equal => Natural::ONE,
                core::cmp::Ordering::Less => Natural::ZERO,
            });
        }
    }
    let mut u = Vec::with_capacity(rank * width);
    for i in 0..rank {
        for j in 0..width {
            u.push(if j >= i {
                core::mem::take(&mut a.elements[i * width + j])
            } else {
                Natural::ZERO
            });
        }
    }
    (
        perm.into_iter().map(u64::exact_from).collect(),
        NaturalMatrix {
            elements: l,
            height,
            width: rank,
        },
        NaturalMatrix {
            elements: u,
            height: rank,
            width,
        },
    )
}

impl ModLuDecomposition<Natural> for NaturalMatrix {
    type Output = (Vec<u64>, Self, Self);

    /// Computes an LU decomposition of a [`NaturalMatrix`] modulo a prime $m$, taking the matrix by
    /// value and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The result is a triple $(P,
    /// L, U)$, where, if $A$ is $h \times w$ and has rank $r$:
    /// - $P$ is a permutation of $0, 1, \ldots, h - 1$, and $PA$ is the matrix whose row $i$ is row
    ///   $P_i$ of $A$;
    /// - $L$ is an $h \times r$ lower triangular matrix with ones on its diagonal;
    /// - $U$ is an $r \times w$ matrix in row echelon form, with $r$ nonzero rows;
    ///
    /// and $PA = LU$ modulo $m$. The pivots are chosen as in Gaussian elimination: in each column,
    /// the pivot is the first nonzero element at or below the current row. When $m$ fits in a limb,
    /// the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the decomposition is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModLuDecomposition;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// let (p, l, u) = a.mod_lu_decomposition(Natural::from(5u32));
    /// assert_eq!(p, &[1, 0]);
    /// assert_eq!(l.to_string(), "[1, 0; 0, 1]");
    /// assert_eq!(u.to_string(), "[1, 0; 0, 1]");
    ///
    /// // A matrix of rank 2.
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// let (p, l, u) = a.mod_lu_decomposition(Natural::from(7u32));
    /// assert_eq!(p, &[0, 2, 1]);
    /// assert_eq!(l.to_string(), "[1, 0; 1, 1; 2, 0]");
    /// assert_eq!(u.to_string(), "[1, 2, 3; 0, 5, 5]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_lu` from `fmpz_mod_mat/lu.c`, FLINT 3.6.0, except that
    /// $L$ and $U$ are returned as separate matrices rather than packed into one, and that it
    /// panics if an element is not reduced or a pivot is not invertible, where FLINT aborts.
    #[inline]
    fn mod_lu_decomposition(self, m: Natural) -> (Vec<u64>, Self, Self) {
        mod_lu_decomposition_helper(self, &m)
    }
}

impl ModLuDecomposition<&Natural> for NaturalMatrix {
    type Output = (Vec<u64>, Self, Self);

    /// Computes an LU decomposition of a [`NaturalMatrix`] modulo a prime $m$, taking the matrix by
    /// value and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The result is a triple $(P,
    /// L, U)$, where, if $A$ is $h \times w$ and has rank $r$:
    /// - $P$ is a permutation of $0, 1, \ldots, h - 1$, and $PA$ is the matrix whose row $i$ is row
    ///   $P_i$ of $A$;
    /// - $L$ is an $h \times r$ lower triangular matrix with ones on its diagonal;
    /// - $U$ is an $r \times w$ matrix in row echelon form, with $r$ nonzero rows;
    ///
    /// and $PA = LU$ modulo $m$. The pivots are chosen as in Gaussian elimination: in each column,
    /// the pivot is the first nonzero element at or below the current row. When $m$ fits in a limb,
    /// the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the decomposition is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModLuDecomposition;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// let (p, l, u) = a.mod_lu_decomposition(&Natural::from(5u32));
    /// assert_eq!(p, &[1, 0]);
    /// assert_eq!(l.to_string(), "[1, 0; 0, 1]");
    /// assert_eq!(u.to_string(), "[1, 0; 0, 1]");
    ///
    /// // A matrix of rank 2.
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// let (p, l, u) = a.mod_lu_decomposition(&Natural::from(7u32));
    /// assert_eq!(p, &[0, 2, 1]);
    /// assert_eq!(l.to_string(), "[1, 0; 1, 1; 2, 0]");
    /// assert_eq!(u.to_string(), "[1, 2, 3; 0, 5, 5]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_lu` from `fmpz_mod_mat/lu.c`, FLINT 3.6.0, except that
    /// $L$ and $U$ are returned as separate matrices rather than packed into one, and that it
    /// panics if an element is not reduced or a pivot is not invertible, where FLINT aborts.
    #[inline]
    fn mod_lu_decomposition(self, m: &Natural) -> (Vec<u64>, Self, Self) {
        mod_lu_decomposition_helper(self, m)
    }
}

impl ModLuDecomposition<Natural> for &NaturalMatrix {
    type Output = (Vec<u64>, NaturalMatrix, NaturalMatrix);

    /// Computes an LU decomposition of a [`NaturalMatrix`] modulo a prime $m$, taking the matrix by
    /// reference and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The result is a triple $(P,
    /// L, U)$, where, if $A$ is $h \times w$ and has rank $r$:
    /// - $P$ is a permutation of $0, 1, \ldots, h - 1$, and $PA$ is the matrix whose row $i$ is row
    ///   $P_i$ of $A$;
    /// - $L$ is an $h \times r$ lower triangular matrix with ones on its diagonal;
    /// - $U$ is an $r \times w$ matrix in row echelon form, with $r$ nonzero rows;
    ///
    /// and $PA = LU$ modulo $m$. The pivots are chosen as in Gaussian elimination: in each column,
    /// the pivot is the first nonzero element at or below the current row. When $m$ fits in a limb,
    /// the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the decomposition is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModLuDecomposition;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// let (p, l, u) = (&a).mod_lu_decomposition(Natural::from(5u32));
    /// assert_eq!(p, &[1, 0]);
    /// assert_eq!(l.to_string(), "[1, 0; 0, 1]");
    /// assert_eq!(u.to_string(), "[1, 0; 0, 1]");
    ///
    /// // A matrix of rank 2.
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// let (p, l, u) = (&a).mod_lu_decomposition(Natural::from(7u32));
    /// assert_eq!(p, &[0, 2, 1]);
    /// assert_eq!(l.to_string(), "[1, 0; 1, 1; 2, 0]");
    /// assert_eq!(u.to_string(), "[1, 2, 3; 0, 5, 5]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_lu` from `fmpz_mod_mat/lu.c`, FLINT 3.6.0, except that
    /// $L$ and $U$ are returned as separate matrices rather than packed into one, and that it
    /// panics if an element is not reduced or a pivot is not invertible, where FLINT aborts.
    #[inline]
    fn mod_lu_decomposition(self, m: Natural) -> (Vec<u64>, NaturalMatrix, NaturalMatrix) {
        mod_lu_decomposition_helper(self.clone(), &m)
    }
}

impl ModLuDecomposition<&Natural> for &NaturalMatrix {
    type Output = (Vec<u64>, NaturalMatrix, NaturalMatrix);

    /// Computes an LU decomposition of a [`NaturalMatrix`] modulo a prime $m$, taking the matrix by
    /// reference and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The result is a triple $(P,
    /// L, U)$, where, if $A$ is $h \times w$ and has rank $r$:
    /// - $P$ is a permutation of $0, 1, \ldots, h - 1$, and $PA$ is the matrix whose row $i$ is row
    ///   $P_i$ of $A$;
    /// - $L$ is an $h \times r$ lower triangular matrix with ones on its diagonal;
    /// - $U$ is an $r \times w$ matrix in row echelon form, with $r$ nonzero rows;
    ///
    /// and $PA = LU$ modulo $m$. The pivots are chosen as in Gaussian elimination: in each column,
    /// the pivot is the first nonzero element at or below the current row. When $m$ fits in a limb,
    /// the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the decomposition is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModLuDecomposition;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// let (p, l, u) = (&a).mod_lu_decomposition(&Natural::from(5u32));
    /// assert_eq!(p, &[1, 0]);
    /// assert_eq!(l.to_string(), "[1, 0; 0, 1]");
    /// assert_eq!(u.to_string(), "[1, 0; 0, 1]");
    ///
    /// // A matrix of rank 2.
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// let (p, l, u) = (&a).mod_lu_decomposition(&Natural::from(7u32));
    /// assert_eq!(p, &[0, 2, 1]);
    /// assert_eq!(l.to_string(), "[1, 0; 1, 1; 2, 0]");
    /// assert_eq!(u.to_string(), "[1, 2, 3; 0, 5, 5]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_lu` from `fmpz_mod_mat/lu.c`, FLINT 3.6.0, except that
    /// $L$ and $U$ are returned as separate matrices rather than packed into one, and that it
    /// panics if an element is not reduced or a pivot is not invertible, where FLINT aborts.
    #[inline]
    fn mod_lu_decomposition(self, m: &Natural) -> (Vec<u64>, NaturalMatrix, NaturalMatrix) {
        mod_lu_decomposition_helper(self.clone(), m)
    }
}
