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

use crate::num::arithmetic::traits::{DivisibleBy, ModIsReduced, ModSolveLowerTriangular};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::classical::Strides;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::strassen::BlockRef;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::{ProductShape, matrix_mod_sub_mul};
use crate::unsigned_vector::arithmetic::mod_dot::{ModData, accumulate};
use alloc::vec;

// Below this many rows or columns of the right-hand side, a triangular system is solved with the
// classical algorithm.
//
// These are `NMOD_MAT_SOLVE_TRI_ROWS_CUTOFF` and `NMOD_MAT_SOLVE_TRI_COLS_CUTOFF` from
// `nmod_mat.h`, FLINT 3.6.0.
pub(crate) const SOLVE_TRI_ROWS_CUTOFF: usize = 64;
pub(crate) const SOLVE_TRI_COLUMNS_CUTOFF: usize = 64;

// The inverse modulo `m` of the diagonal element in row `i` of the block `t`.
pub(crate) fn diagonal_inverse<T: PrimitiveUnsigned>(t: BlockRef<'_, T>, i: usize, m: T) -> T {
    t.row(i, i + 1)[i]
        .mod_inverse(m)
        .expect("a diagonal element of the triangular matrix is not invertible mod m")
}

// Solves `l` X = B modulo `m` by substitution, where `l` is an `n` by `n` lower triangular block
// and `xs` holds the `n` by `k` matrix B, which is replaced by X. One row of X is computed at a
// time: the sums for row j, one for each column, are accumulated unreduced over the rows above it
// and reduced once.
//
// This is equivalent to `nmod_mat_solve_tril_classical` from `nmod_mat/solve_tril.c`, FLINT 3.6.0,
// where `xs` is both `X` and `B`, except that FLINT computes one column of X at a time, which reads
// X at a stride. The results are the same.
fn solve_lower_classical<T: PrimitiveUnsigned>(
    xs: &mut [T],
    l: BlockRef<'_, T>,
    n: usize,
    k: usize,
    m: T,
    unit: bool,
) {
    if n == 0 || k == 0 {
        return;
    }
    let d = ModData::new(m, n);
    let mut acc = vec![(T::ZERO, T::ZERO, T::ZERO); k];
    for j in 0..n {
        let (above, rest) = xs.split_at_mut(j * k);
        let row = &mut rest[..k];
        let coefficients = l.row(j, j);
        let solved = above;
        acc.fill((T::ZERO, T::ZERO, T::ZERO));
        match d.words {
            1 => {
                for (&c, x_row) in coefficients.iter().zip(solved.chunks_exact(k)) {
                    if c != T::ZERO {
                        for (a, &x) in acc.iter_mut().zip(x_row) {
                            a.2.wrapping_add_assign(c.wrapping_mul(x));
                        }
                    }
                }
            }
            2 => {
                for (&c, x_row) in coefficients.iter().zip(solved.chunks_exact(k)) {
                    if c != T::ZERO {
                        for (a, &x) in acc.iter_mut().zip(x_row) {
                            let (hi, lo) = T::x_mul_y_to_zz(c, x);
                            (a.1, a.2) = T::xx_add_yy_to_zz(a.1, a.2, hi, lo);
                        }
                    }
                }
            }
            _ => {
                for (i, (&c, x_row)) in coefficients.iter().zip(solved.chunks_exact(k)).enumerate()
                {
                    if i != 0 && i.divisible_by(d.max_terms) {
                        for a in &mut acc {
                            *a = (T::ZERO, T::ZERO, d.reduce(a.0, a.1, a.2));
                        }
                    }
                    for (a, &x) in acc.iter_mut().zip(x_row) {
                        accumulate(a, c, x);
                    }
                }
            }
        }
        for (y, a) in row.iter_mut().zip(&acc) {
            y.mod_sub_assign(d.reduce_sum(*a), m);
        }
        if !unit {
            let inverse = diagonal_inverse(l, j, m);
            for y in row {
                y.mod_mul_assign(inverse, m);
            }
        }
    }
}

// This is equivalent to `nmod_mat_solve_tril_recursive` from `nmod_mat/solve_tril.c`, FLINT 3.6.0,
// where `xs` is both `X` and `B`.
fn solve_lower_recursive<T: PrimitiveUnsigned>(
    xs: &mut [T],
    l: BlockRef<'_, T>,
    n: usize,
    k: usize,
    m: T,
    unit: bool,
) {
    if n == 0 || k == 0 {
        return;
    }
    let r = n >> 1;
    // Denoting the inverse of M by M^, we have: [A 0]^ [X] = [A^          0 ] [X] = [A^ X] [C D]
    // [Y] = [-D^ C A^    D^] [Y] = [D^ (Y - C A^ X)]
    let la = l;
    let ld = BlockRef {
        row: l.row + r,
        column: l.column + r,
        ..l
    };
    let (xx, xy) = xs.split_at_mut(r * k);
    solve_lower(xx, la, r, k, m, unit);
    matrix_mod_sub_mul(
        xy,
        &l.xs[(l.row + r) * l.width + l.column..],
        Strides {
            out: k,
            xs: l.width,
        },
        xx,
        ProductShape {
            ar: n - r,
            br: r,
            bc: k,
        },
        m,
    );
    solve_lower(xy, ld, n - r, k, m, unit);
}

// Solves `l` X = B modulo `m`, where `l` is an `n` by `n` lower triangular block and `xs` holds the
// `n` by `k` matrix B, which is replaced by X.
//
// This is equivalent to `nmod_mat_solve_tril` from `nmod_mat/solve_tril.c`, FLINT 3.6.0, where `xs`
// is both `X` and `B`.
pub(crate) fn solve_lower<T: PrimitiveUnsigned>(
    xs: &mut [T],
    l: BlockRef<'_, T>,
    n: usize,
    k: usize,
    m: T,
    unit: bool,
) {
    if n < SOLVE_TRI_ROWS_CUTOFF || k < SOLVE_TRI_COLUMNS_CUTOFF {
        solve_lower_classical(xs, l, n, k, m, unit);
    } else {
        solve_lower_recursive(xs, l, n, k, m, unit);
    }
}

pub(crate) fn assert_triangular_arguments<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
) {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert_eq!(a.height, a.width, "the triangular matrix must be square");
    assert_eq!(
        a.height, b.height,
        "the height of the triangular matrix must equal the height of the right-hand side"
    );
    assert!(
        a.mod_is_reduced(&m),
        "the triangular matrix must be reduced mod m, but {a} has an element >= {m}"
    );
    assert!(
        b.mod_is_reduced(&m),
        "the right-hand side must be reduced mod m, but {b} has an element >= {m}"
    );
}

fn mod_solve_lower_triangular_in_place<T: PrimitiveUnsigned>(
    l: &UnsignedMatrix<T>,
    b: &mut UnsignedMatrix<T>,
    m: T,
    unit: bool,
) {
    assert_triangular_arguments(l, b, m);
    if m == T::ONE {
        // Every element of `b` is 0.
        return;
    }
    let lr = BlockRef {
        xs: &l.elements,
        width: l.width,
        row: 0,
        column: 0,
    };
    solve_lower(&mut b.elements, lr, l.height, b.width, m, unit);
}

impl<T: PrimitiveUnsigned> ModSolveLowerTriangular<Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a lower triangular [`UnsignedMatrix`],
    /// taking both by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. $L$ must be square, and
    /// $B$ must have the same height. Only the elements of $L$ on and below its diagonal are used;
    /// the ones above it are ignored. If `unit_diagonal` is `true`, the diagonal elements are taken
    /// to be 1 and are ignored too; otherwise, each must be invertible modulo $m$. Either way, the
    /// solution is unique. If $L$ is $n \times n$ and $B$ is $n \times k$, then $X$ is the $n
    /// \times k$ matrix with
    ///
    /// $$
    /// X_{ij} = d_i^{-1} \left ( B_{ij} - \sum_{l=0}^{i-1} L_{il}X_{lj} \right ) \bmod m,
    /// $$
    ///
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $L_{ii}$ otherwise.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2k)$
    ///
    /// $M(n, k) = O(n^2 + nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $L$, and $k$ is the width
    /// of $B$.
    ///
    /// # Panics
    /// Panics if `m` is zero, if $L$ is not square, if the heights of $L$ and $B$ differ, if any
    /// element of either is greater than or equal to `m`, or if `unit_diagonal` is `false` and a
    /// diagonal element of $L$ is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolveLowerTriangular;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(b, 7, false).to_string(),
    ///     "[4; 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(b, 7, true).to_string(),
    ///     "[1; 6]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(b, 7, false).to_string(),
    ///     "[4, 0; 4, 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_solve_tril` from `nmod_mat/solve_tril.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible or an element is not reduced, which
    /// FLINT does not check.
    fn mod_solve_lower_triangular(self, mut b: Self, m: T, unit_diagonal: bool) -> Self {
        mod_solve_lower_triangular_in_place(&self, &mut b, m, unit_diagonal);
        b
    }
}

impl<T: PrimitiveUnsigned> ModSolveLowerTriangular<&Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a lower triangular [`UnsignedMatrix`],
    /// taking the triangular matrix by value and the right-hand side by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. $L$ must be square, and
    /// $B$ must have the same height. Only the elements of $L$ on and below its diagonal are used;
    /// the ones above it are ignored. If `unit_diagonal` is `true`, the diagonal elements are taken
    /// to be 1 and are ignored too; otherwise, each must be invertible modulo $m$. Either way, the
    /// solution is unique. If $L$ is $n \times n$ and $B$ is $n \times k$, then $X$ is the $n
    /// \times k$ matrix with
    ///
    /// $$
    /// X_{ij} = d_i^{-1} \left ( B_{ij} - \sum_{l=0}^{i-1} L_{il}X_{lj} \right ) \bmod m,
    /// $$
    ///
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $L_{ii}$ otherwise.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2k)$
    ///
    /// $M(n, k) = O(n^2 + nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $L$, and $k$ is the width
    /// of $B$.
    ///
    /// # Panics
    /// Panics if `m` is zero, if $L$ is not square, if the heights of $L$ and $B$ differ, if any
    /// element of either is greater than or equal to `m`, or if `unit_diagonal` is `false` and a
    /// diagonal element of $L$ is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolveLowerTriangular;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(&b, 7, false).to_string(),
    ///     "[4; 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(&b, 7, true).to_string(),
    ///     "[1; 6]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(&b, 7, false).to_string(),
    ///     "[4, 0; 4, 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_solve_tril` from `nmod_mat/solve_tril.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible or an element is not reduced, which
    /// FLINT does not check.
    fn mod_solve_lower_triangular(self, b: &Self, m: T, unit_diagonal: bool) -> Self {
        let mut b = b.clone();
        mod_solve_lower_triangular_in_place(&self, &mut b, m, unit_diagonal);
        b
    }
}

impl<T: PrimitiveUnsigned> ModSolveLowerTriangular<UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a lower triangular [`UnsignedMatrix`],
    /// taking the triangular matrix by reference and the right-hand side by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. $L$ must be square, and
    /// $B$ must have the same height. Only the elements of $L$ on and below its diagonal are used;
    /// the ones above it are ignored. If `unit_diagonal` is `true`, the diagonal elements are taken
    /// to be 1 and are ignored too; otherwise, each must be invertible modulo $m$. Either way, the
    /// solution is unique. If $L$ is $n \times n$ and $B$ is $n \times k$, then $X$ is the $n
    /// \times k$ matrix with
    ///
    /// $$
    /// X_{ij} = d_i^{-1} \left ( B_{ij} - \sum_{l=0}^{i-1} L_{il}X_{lj} \right ) \bmod m,
    /// $$
    ///
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $L_{ii}$ otherwise.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2k)$
    ///
    /// $M(n, k) = O(n^2 + nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $L$, and $k$ is the width
    /// of $B$.
    ///
    /// # Panics
    /// Panics if `m` is zero, if $L$ is not square, if the heights of $L$ and $B$ differ, if any
    /// element of either is greater than or equal to `m`, or if `unit_diagonal` is `false` and a
    /// diagonal element of $L$ is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolveLowerTriangular;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(b, 7, false).to_string(),
    ///     "[4; 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(b, 7, true).to_string(),
    ///     "[1; 6]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(b, 7, false).to_string(),
    ///     "[4, 0; 4, 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_solve_tril` from `nmod_mat/solve_tril.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible or an element is not reduced, which
    /// FLINT does not check.
    fn mod_solve_lower_triangular(
        self,
        mut b: UnsignedMatrix<T>,
        m: T,
        unit_diagonal: bool,
    ) -> UnsignedMatrix<T> {
        mod_solve_lower_triangular_in_place(self, &mut b, m, unit_diagonal);
        b
    }
}

impl<T: PrimitiveUnsigned> ModSolveLowerTriangular<&UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a lower triangular [`UnsignedMatrix`],
    /// taking both by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. $L$ must be square, and
    /// $B$ must have the same height. Only the elements of $L$ on and below its diagonal are used;
    /// the ones above it are ignored. If `unit_diagonal` is `true`, the diagonal elements are taken
    /// to be 1 and are ignored too; otherwise, each must be invertible modulo $m$. Either way, the
    /// solution is unique. If $L$ is $n \times n$ and $B$ is $n \times k$, then $X$ is the $n
    /// \times k$ matrix with
    ///
    /// $$
    /// X_{ij} = d_i^{-1} \left ( B_{ij} - \sum_{l=0}^{i-1} L_{il}X_{lj} \right ) \bmod m,
    /// $$
    ///
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $L_{ii}$ otherwise.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2k)$
    ///
    /// $M(n, k) = O(n^2 + nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $L$, and $k$ is the width
    /// of $B$.
    ///
    /// # Panics
    /// Panics if `m` is zero, if $L$ is not square, if the heights of $L$ and $B$ differ, if any
    /// element of either is greater than or equal to `m`, or if `unit_diagonal` is `false` and a
    /// diagonal element of $L$ is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolveLowerTriangular;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(&b, 7, false).to_string(),
    ///     "[4; 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(&b, 7, true).to_string(),
    ///     "[1; 6]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(&b, 7, false).to_string(),
    ///     "[4, 0; 4, 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_solve_tril` from `nmod_mat/solve_tril.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible or an element is not reduced, which
    /// FLINT does not check.
    fn mod_solve_lower_triangular(
        self,
        b: &UnsignedMatrix<T>,
        m: T,
        unit_diagonal: bool,
    ) -> UnsignedMatrix<T> {
        let mut b = b.clone();
        mod_solve_lower_triangular_in_place(self, &mut b, m, unit_diagonal);
        b
    }
}
