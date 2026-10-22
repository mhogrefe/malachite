// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011, 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModLuDecomposition};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::classical::Strides;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::strassen::{BlockRef, block, set_block};
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::{ProductShape, matrix_mod_sub_mul};
use crate::unsigned_matrix::arithmetic::mod_solve_lower_triangular::solve_lower;
use crate::unsigned_vector::arithmetic::mod_dot::ModData;
use alloc::vec;
use alloc::vec::Vec;

// A block of a matrix that LU decomposition works on in place: `xs` starts at the block's top-left
// element, and row `i` of the block is `xs[i * stride..i * stride + width]`.
pub(crate) struct LuBlock<'a, T> {
    pub(crate) xs: &'a mut [T],
    pub(crate) stride: usize,
    pub(crate) height: usize,
    pub(crate) width: usize,
}

impl<T: PrimitiveUnsigned> LuBlock<'_, T> {
    #[inline]
    const fn get(&self, i: usize, j: usize) -> T {
        self.xs[i * self.stride + j]
    }

    #[inline]
    const fn set(&mut self, i: usize, j: usize, x: T) {
        self.xs[i * self.stride + j] = x;
    }

    // Rows `i` and `j` of the block, where `i < j`.
    fn two_rows(&mut self, i: usize, j: usize) -> (&mut [T], &mut [T]) {
        let (top, bottom) = self.xs.split_at_mut(j * self.stride);
        (
            &mut top[i * self.stride..i * self.stride + self.width],
            &mut bottom[..self.width],
        )
    }

    // This is equivalent to `nmod_mat_swap_rows` from `nmod_mat.h`, FLINT 3.6.0.
    fn swap_rows(&mut self, perm: &mut [usize], r: usize, s: usize) {
        if r != s && self.width != 0 {
            perm.swap(r, s);
            let (r, s) = if r < s { (r, s) } else { (s, r) };
            let (x, y) = self.two_rows(r, s);
            x.swap_with_slice(y);
        }
    }
}

// The inverse of a nonzero pivot modulo `m`, which exists when `m` is prime.
pub(crate) fn pivot_inverse<T: PrimitiveUnsigned>(x: T, m: T) -> T {
    x.mod_inverse(m)
        .unwrap_or_else(|| panic!("the pivot {x} is not invertible mod {m}, so {m} is not prime"))
}

// This is equivalent to `nmod_mat_pivot` from `nmod_mat/lu_classical.c`, FLINT 3.6.0, returning
// whether a pivot was found.
fn pivot<T: PrimitiveUnsigned>(
    a: &mut LuBlock<'_, T>,
    perm: &mut [usize],
    start_row: usize,
    col: usize,
) -> bool {
    if a.get(start_row, col) != T::ZERO {
        return true;
    }
    if let Some(j) = (start_row + 1..a.height).find(|&j| a.get(j, col) != T::ZERO) {
        a.swap_rows(perm, j, start_row);
        true
    } else {
        false
    }
}

// This is equivalent to `nmod_mat_lu_classical` from `nmod_mat/lu_classical.c`, FLINT 3.6.0.
pub(crate) fn lu_classical<T: PrimitiveUnsigned>(
    perm: &mut [usize],
    a: &mut LuBlock<'_, T>,
    m: T,
    rank_check: bool,
) -> usize {
    let (height, width) = (a.height, a.width);
    for (i, p) in perm[..height].iter_mut().enumerate() {
        *p = i;
    }
    let data = T::precompute_mod_mul_data(&m);
    let mut rank = 0;
    let mut row = 0;
    let mut col = 0;
    while row < height && col < width {
        if !pivot(a, perm, row, col) {
            if rank_check {
                return 0;
            }
            col += 1;
            continue;
        }
        rank += 1;
        let d = pivot_inverse(a.get(row, col), m);
        for i in row + 1..height {
            let e = a.get(i, col).mod_mul_precomputed(d, m, &data);
            let f = e.mod_neg(m);
            let (pivot_row, other_row) = a.two_rows(row, i);
            for (x, &y) in other_row[col + 1..].iter_mut().zip(&pivot_row[col + 1..]) {
                x.mod_add_assign(f.mod_mul_precomputed(y, m, &data), m);
            }
            a.set(i, col, T::ZERO);
            a.set(i, rank - 1, e);
        }
        row += 1;
        col += 1;
    }
    rank
}

// This is equivalent to `nmod_mat_lu_classical_delayed_1` from `nmod_mat/lu_classical_delayed.c`,
// FLINT 3.6.0. The unreduced elements are kept in the matrix itself.
fn lu_classical_delayed_1<T: PrimitiveUnsigned>(
    perm: &mut [usize],
    a: &mut LuBlock<'_, T>,
    m: T,
    rank_check: bool,
) -> usize {
    let (height, width) = (a.height, a.width);
    for (i, p) in perm[..height].iter_mut().enumerate() {
        *p = i;
    }
    let mut rank = 0;
    let mut row = 0;
    let mut col = 0;
    while row < height && col < width {
        // Reduce the current column. This can be skipped on the first iteration.
        if col != 0 {
            for j in row..height {
                let x = a.get(j, col);
                a.set(j, col, x % m);
            }
        }
        let Some(pivot_row) = (row..height).find(|&i| a.get(i, col) != T::ZERO) else {
            if rank_check {
                rank = 0;
                break;
            }
            col += 1;
            continue;
        };
        a.swap_rows(perm, row, pivot_row);
        // Reduce the current pivot row.
        if col != 0 {
            for j in col + 1..width {
                let x = a.get(row, j);
                a.set(row, j, x % m);
            }
        }
        rank += 1;
        // Eliminate the remaining submatrix.
        let d = pivot_inverse(a.get(row, col), m);
        for i in row + 1..height {
            let e = if d == T::ONE {
                a.get(i, col)
            } else {
                d.mod_mul(a.get(i, col), m)
            };
            // When e is 0 the update adds a multiple of m, which FLINT does; skipping it keeps the
            // unreduced elements within the bound.
            if e != T::ZERO {
                let f = m - e;
                let (pivot_row, other_row) = a.two_rows(row, i);
                for (x, &y) in other_row[col + 1..].iter_mut().zip(&pivot_row[col + 1..]) {
                    x.wrapping_add_assign(f.wrapping_mul(y));
                }
            }
            a.set(i, col, T::ZERO);
            a.set(i, rank - 1, e);
        }
        row += 1;
        col += 1;
    }
    rank
}

// An unreduced element accumulated in two or three words.
trait Unreduced<T: PrimitiveUnsigned>: Copy {
    fn from_word(x: T) -> Self;

    fn add_product(&mut self, x: T, y: T);

    fn reduce(self, d: &ModData<T>) -> T;
}

impl<T: PrimitiveUnsigned> Unreduced<T> for (T, T) {
    #[inline]
    fn from_word(x: T) -> Self {
        (T::ZERO, x)
    }

    #[inline]
    fn add_product(&mut self, x: T, y: T) {
        let (hi, lo) = T::x_mul_y_to_zz(x, y);
        *self = T::xx_add_yy_to_zz(self.0, self.1, hi, lo);
    }

    #[inline]
    fn reduce(self, d: &ModData<T>) -> T {
        d.reduce_2(self.0, self.1)
    }
}

impl<T: PrimitiveUnsigned> Unreduced<T> for (T, T, T) {
    #[inline]
    fn from_word(x: T) -> Self {
        (T::ZERO, T::ZERO, x)
    }

    #[inline]
    fn add_product(&mut self, x: T, y: T) {
        let (hi, lo) = T::x_mul_y_to_zz(x, y);
        *self = T::xxx_add_yyy_to_zzz(self.0, self.1, self.2, T::ZERO, hi, lo);
    }

    #[inline]
    fn reduce(self, d: &ModData<T>) -> T {
        d.reduce(self.0, self.1, self.2)
    }
}

// This is equivalent to `nmod_mat_lu_classical_delayed_2` and `nmod_mat_lu_classical_delayed_3`
// from `nmod_mat/lu_classical_delayed.c`, FLINT 3.6.0, which differ only in the number of words in
// which the unreduced elements are accumulated: two or three, given by `U`. The unreduced elements
// are kept in a separate matrix, `b`.
fn lu_classical_delayed_unreduced<T: PrimitiveUnsigned, U: Unreduced<T>>(
    perm: &mut [usize],
    a: &mut LuBlock<'_, T>,
    m: T,
    rank_check: bool,
    d: &ModData<T>,
) -> usize {
    let (height, width) = (a.height, a.width);
    for (i, p) in perm[..height].iter_mut().enumerate() {
        *p = i;
    }
    let mut b: Vec<U> = Vec::with_capacity(height * width);
    for i in 0..height {
        for j in 0..width {
            b.push(U::from_word(a.get(i, j)));
        }
    }
    let data = T::precompute_mod_mul_data(&m);
    let mut rank = 0;
    let mut row = 0;
    let mut col = 0;
    while row < height && col < width {
        if col != 0 {
            for j in row..height {
                a.set(j, col, b[j * width + col].reduce(d));
            }
        }
        let Some(pivot_row) = (row..height).find(|&i| a.get(i, col) != T::ZERO) else {
            if rank_check {
                rank = 0;
                break;
            }
            col += 1;
            continue;
        };
        if pivot_row != row {
            a.swap_rows(perm, row, pivot_row);
            for j in col + 1..width {
                let x = b[row * width + j];
                a.set(row, j, b[pivot_row * width + j].reduce(d));
                b[pivot_row * width + j] = x;
            }
        } else if row != 0 {
            for j in col + 1..width {
                a.set(row, j, b[row * width + j].reduce(d));
            }
        }
        rank += 1;
        let inverse = pivot_inverse(a.get(row, col), m);
        for i in row + 1..height {
            let e = inverse.mod_mul_precomputed(a.get(i, col), m, &data);
            if e != T::ZERO {
                let f = m - e;
                let pivot_row = &a.xs[row * a.stride + col + 1..row * a.stride + width];
                for (x, &y) in b[i * width + col + 1..(i + 1) * width]
                    .iter_mut()
                    .zip(pivot_row)
                {
                    x.add_product(f, y);
                }
            }
            a.set(i, col, T::ZERO);
            a.set(i, rank - 1, e);
        }
        row += 1;
        col += 1;
    }
    rank
}

// This is equivalent to `nmod_mat_lu_classical_delayed` from `nmod_mat/lu_classical_delayed.c`,
// FLINT 3.6.0.
pub(crate) fn lu_classical_delayed<T: PrimitiveUnsigned>(
    perm: &mut [usize],
    a: &mut LuBlock<'_, T>,
    m: T,
    rank_check: bool,
) -> usize {
    let n = a.height.min(a.width);
    let d = ModData::new(m, n);
    match d.words {
        1 => lu_classical_delayed_1(perm, a, m, rank_check),
        // FLINT only handles moduli below 2^(W - 1) with two words; this case is normally only
        // reachable when n <= 3, where the classical algorithm is used anyway.
        2 if m.get_highest_bit() => lu_classical(perm, a, m, rank_check),
        2 => lu_classical_delayed_unreduced::<T, (T, T)>(perm, a, m, rank_check, &d),
        // Three words hold the unreduced sums only when there are fewer than 2^(W - 1) of them.
        // FLINT's words have 64 bits, so for it this is always the case, but it can fail for narrow
        // types.
        3 if n > d.max_terms => lu_classical(perm, a, m, rank_check),
        _ => lu_classical_delayed_unreduced::<T, (T, T, T)>(perm, a, m, rank_check, &d),
    }
}

// Applies the row permutation `p` to the `num_rows` rows of `a` starting at `row_offset`, in the
// `num_cols` columns starting at `col_offset`, and composes it with the permutation `ap`.
//
// This is equivalent to `_apply_permutation` from `nmod_mat/lu_recursive.c`, FLINT 3.6.0, without
// the slicing of wide blocks.
fn apply_permutation<T: PrimitiveUnsigned>(
    ap: &mut [usize],
    a: &mut LuBlock<'_, T>,
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
        tmp.extend_from_slice(&a.xs[start..start + num_cols]);
    }
    if num_cols != 0 {
        for (i, row) in tmp.chunks_exact(num_cols).enumerate() {
            let start = (i + row_offset) * stride + col_offset;
            a.xs[start..start + num_cols].copy_from_slice(row);
        }
    }
    let ap_tmp: Vec<usize> = p[..num_rows]
        .iter()
        .map(|&pi| ap[pi + row_offset])
        .collect();
    ap[row_offset..row_offset + num_rows].copy_from_slice(&ap_tmp);
}

// This is equivalent to `nmod_mat_lu_recursive` from `nmod_mat/lu_recursive.c`, FLINT 3.6.0. FLINT
// references the blocks through windows; here the bottom-right block is updated and decomposed in
// place, and the top-right and bottom-left blocks are copied, since the triangular solve needs a
// contiguous right-hand side and the bottom-left block shares rows with the bottom-right one.
pub(crate) fn lu_recursive<T: PrimitiveUnsigned>(
    perm: &mut [usize],
    a: &mut LuBlock<'_, T>,
    m: T,
    rank_check: bool,
) -> usize {
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
        &mut LuBlock {
            xs: a.xs,
            stride,
            height,
            width: n1,
        },
        m,
        rank_check,
    );
    if rank_check && r1 != n1 {
        return 0;
    }
    if r1 != 0 {
        apply_permutation(perm, a, &p1, height, 0, width - n1, n1);
    }
    // A00 = rows 0..r1, columns 0..r1; A10 = rows r1..height, columns 0..r1; A01 = rows 0..r1,
    // columns n1..width; A11 = rows r1..height, columns n1..width.
    let w2 = width - n1;
    let h2 = height - r1;
    if r1 != 0 {
        let mut a01 = block(a.xs, stride, 0..r1, n1..width);
        let a00 = BlockRef {
            xs: a.xs,
            width: stride,
            row: 0,
            column: 0,
        };
        solve_lower(&mut a01, a00, r1, w2, m, true);
        set_block(a.xs, stride, 0, n1, &a01, w2);
        // A10 and A11 occupy the same rows, so A10 is copied in order to update A11 in place.
        let a10 = block(a.xs, stride, r1..height, 0..r1);
        if h2 != 0 {
            matrix_mod_sub_mul(
                &mut a.xs[r1 * stride + n1..],
                &a10,
                Strides {
                    out: stride,
                    xs: r1,
                },
                &a01,
                ProductShape {
                    ar: h2,
                    br: r1,
                    bc: w2,
                },
                m,
            );
        }
    }
    let r2 = if h2 == 0 {
        0
    } else {
        lu(
            &mut p1,
            &mut LuBlock {
                xs: &mut a.xs[r1 * stride + n1..],
                stride,
                height: h2,
                width: w2,
            },
            m,
            rank_check,
        )
    };
    if rank_check && r1 + r2 < height.min(width) {
        return 0;
    }
    apply_permutation(perm, a, &p1, h2, r1, n1, 0);
    // Compress L.
    if r1 != n1 {
        for i in 0..h2 {
            let row = &mut a.xs[(r1 + i) * stride..(r1 + i) * stride + width];
            for j in 0..i.min(r2) {
                row[r1 + j] = row[n1 + j];
                row[n1 + j] = T::ZERO;
            }
        }
    }
    r1 + r2
}

// The dimension at which LU decomposition switches from the classical algorithm with delayed
// reduction to the recursive algorithm, indexed by the number of bits of the modulus minus 1.
//
// This is the first `lu_cutoff_tab` from `nmod_mat/lu.c`, FLINT 3.6.0, the one used without BLAS or
// AVX2.
const LU_CUTOFF_TAB: [usize; 64] = [
    64, 64, 244, 280, 296, 320, 332, 792, 800, 800, 800, 400, 400, 400, 400, 404, 404, 400, 408,
    416, 400, 412, 424, 408, 484, 1352, 1032, 260, 68, 56, 56, 56, 148, 160, 148, 156, 156, 156,
    120, 168, 160, 124, 156, 124, 156, 148, 112, 156, 148, 136, 156, 160, 116, 136, 168, 148, 156,
    160, 120, 128, 68, 64, 104, 104,
];

// Computes the LU decomposition of the block `a`, with elements reduced modulo the prime `m`, in
// place, and returns its rank. Afterwards, `perm[i]` is the row of the original block that ended up
// in row `i`; the rows of the block hold U on and above the diagonal and the multipliers of L,
// whose diagonal elements are 1, below it, compressed into the first `rank` columns. If
// `rank_check` is `true`, the decomposition stops and 0 is returned as soon as the block is found
// not to have full rank.
//
// This is equivalent to `nmod_mat_lu` from `nmod_mat/lu.c`, FLINT 3.6.0, without BLAS or AVX2.
pub(crate) fn lu<T: PrimitiveUnsigned>(
    perm: &mut [usize],
    a: &mut LuBlock<'_, T>,
    m: T,
    rank_check: bool,
) -> usize {
    let n = a.height.min(a.width);
    let bits = m.significant_bits();
    if n <= 3 || (bits > 28 && n <= 7) {
        lu_classical(perm, a, m, rank_check)
    } else if n < LU_CUTOFF_TAB[usize::exact_from(bits.min(64)) - 1] {
        lu_classical_delayed(perm, a, m, rank_check)
    } else {
        lu_recursive(perm, a, m, rank_check)
    }
}

pub(crate) fn assert_reduced<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert!(
        a.mod_is_reduced(&m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

// Computes the packed LU decomposition of `a` in place, returning the permutation and the rank.
pub(crate) fn lu_in_place<T: PrimitiveUnsigned>(
    a: &mut UnsignedMatrix<T>,
    m: T,
    rank_check: bool,
) -> (Vec<usize>, usize) {
    let mut perm = vec![0; a.height];
    let rank = lu(
        &mut perm,
        &mut LuBlock {
            xs: &mut a.elements,
            stride: a.width,
            height: a.height,
            width: a.width,
        },
        m,
        rank_check,
    );
    (perm, rank)
}

fn mod_lu_decomposition_helper<T: PrimitiveUnsigned>(
    mut a: UnsignedMatrix<T>,
    m: T,
) -> (Vec<u64>, UnsignedMatrix<T>, UnsignedMatrix<T>) {
    assert_reduced(&a, m);
    let (perm, rank) = lu_in_place(&mut a, m, false);
    let (height, width) = (a.height, a.width);
    let mut l = Vec::with_capacity(height * rank);
    for i in 0..height {
        for c in 0..rank {
            l.push(match i.cmp(&c) {
                core::cmp::Ordering::Greater => a.elements[i * width + c],
                core::cmp::Ordering::Equal => T::ONE,
                core::cmp::Ordering::Less => T::ZERO,
            });
        }
    }
    let mut u = Vec::with_capacity(rank * width);
    for i in 0..rank {
        for j in 0..width {
            u.push(if j >= i {
                a.elements[i * width + j]
            } else {
                T::ZERO
            });
        }
    }
    (
        perm.into_iter().map(u64::exact_from).collect(),
        UnsignedMatrix {
            elements: l,
            height,
            width: rank,
        },
        UnsignedMatrix {
            elements: u,
            height: rank,
            width,
        },
    )
}

impl<T: PrimitiveUnsigned> ModLuDecomposition<T> for UnsignedMatrix<T> {
    type Output = (Vec<u64>, Self, Self);

    /// Computes an LU decomposition of an [`UnsignedMatrix`] modulo a prime $m$, taking the matrix
    /// by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The result is a triple $(P,
    /// L, U)$, where, if $A$ is $h \times w$ and has rank $r$:
    /// - $P$ is a permutation of $0, 1, \ldots, h - 1$, and $PA$ is the matrix whose row $i$ is row
    ///   $P_i$ of $A$;
    /// - $L$ is an $h \times r$ lower triangular matrix with ones on its diagonal;
    /// - $U$ is an $r \times w$ matrix in row echelon form, with $r$ nonzero rows;
    ///
    /// and $PA = LU$ modulo $m$. The pivots are chosen as in Gaussian elimination: in each column,
    /// the pivot is the first nonzero element at or below the current row.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the decomposition is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw\min(h, w))$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, and $w$ is its
    /// width.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModLuDecomposition;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 1; 1, 0]").unwrap();
    /// let (p, l, u) = a.mod_lu_decomposition(5);
    /// assert_eq!(p, &[1, 0]);
    /// assert_eq!(l.to_string(), "[1, 0; 0, 1]");
    /// assert_eq!(u.to_string(), "[1, 0; 0, 1]");
    ///
    /// // A matrix of rank 2.
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// let (p, l, u) = a.mod_lu_decomposition(7);
    /// assert_eq!(p, &[0, 2, 1]);
    /// assert_eq!(l.to_string(), "[1, 0; 1, 1; 2, 0]");
    /// assert_eq!(u.to_string(), "[1, 2, 3; 0, 5, 5]");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_lu` from `nmod_mat/lu.c`, FLINT 3.6.0, without BLAS or AVX2,
    /// except that $L$ and $U$ are returned as separate matrices rather than packed into one, and
    /// that it panics if an element is not reduced.
    #[inline]
    fn mod_lu_decomposition(self, m: T) -> (Vec<u64>, Self, Self) {
        mod_lu_decomposition_helper(self, m)
    }
}

impl<T: PrimitiveUnsigned> ModLuDecomposition<T> for &UnsignedMatrix<T> {
    type Output = (Vec<u64>, UnsignedMatrix<T>, UnsignedMatrix<T>);

    /// Computes an LU decomposition of an [`UnsignedMatrix`] modulo a prime $m$, taking the matrix
    /// by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The result is a triple $(P,
    /// L, U)$, where, if $A$ is $h \times w$ and has rank $r$:
    /// - $P$ is a permutation of $0, 1, \ldots, h - 1$, and $PA$ is the matrix whose row $i$ is row
    ///   $P_i$ of $A$;
    /// - $L$ is an $h \times r$ lower triangular matrix with ones on its diagonal;
    /// - $U$ is an $r \times w$ matrix in row echelon form, with $r$ nonzero rows;
    ///
    /// and $PA = LU$ modulo $m$. The pivots are chosen as in Gaussian elimination: in each column,
    /// the pivot is the first nonzero element at or below the current row.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the decomposition is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw\min(h, w))$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, and $w$ is its
    /// width.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModLuDecomposition;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 1; 1, 0]").unwrap();
    /// let (p, l, u) = (&a).mod_lu_decomposition(5);
    /// assert_eq!(p, &[1, 0]);
    /// assert_eq!(l.to_string(), "[1, 0; 0, 1]");
    /// assert_eq!(u.to_string(), "[1, 0; 0, 1]");
    ///
    /// // A matrix of rank 2.
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// let (p, l, u) = (&a).mod_lu_decomposition(7);
    /// assert_eq!(p, &[0, 2, 1]);
    /// assert_eq!(l.to_string(), "[1, 0; 1, 1; 2, 0]");
    /// assert_eq!(u.to_string(), "[1, 2, 3; 0, 5, 5]");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_lu` from `nmod_mat/lu.c`, FLINT 3.6.0, without BLAS or AVX2,
    /// except that $L$ and $U$ are returned as separate matrices rather than packed into one, and
    /// that it panics if an element is not reduced.
    #[inline]
    fn mod_lu_decomposition(self, m: T) -> (Vec<u64>, UnsignedMatrix<T>, UnsignedMatrix<T>) {
        mod_lu_decomposition_helper(self.clone(), m)
    }
}
