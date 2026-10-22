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

use crate::num::arithmetic::traits::{DivisibleBy, ModSolveUpperTriangular};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::classical::Strides;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::strassen::BlockRef;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::{ProductShape, matrix_mod_sub_mul};
use crate::unsigned_matrix::arithmetic::mod_solve_lower_triangular::{
    SOLVE_TRI_COLUMNS_CUTOFF, SOLVE_TRI_ROWS_CUTOFF, assert_triangular_arguments, diagonal_inverse,
};
use crate::unsigned_vector::arithmetic::mod_dot::{ModData, accumulate};
use alloc::vec;

// Solves `u` X = B modulo `m` by substitution, where `u` is an `n` by `n` upper triangular block
// and `xs` holds the `n` by `k` matrix B, which is replaced by X. One row of X is computed at a
// time: the sums for row j, one for each column, are accumulated unreduced over the rows below it
// and reduced once.
//
// This is equivalent to `nmod_mat_solve_triu_classical` from `nmod_mat/solve_triu.c`, FLINT 3.6.0,
// where `xs` is both `X` and `B`, except that FLINT computes one column of X at a time, which reads
// X at a stride. The results are the same.
fn solve_upper_classical<T: PrimitiveUnsigned>(
    xs: &mut [T],
    u: BlockRef<'_, T>,
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
    for j in (0..n).rev() {
        let (row, below) = xs[j * k..].split_at_mut(k);
        let coefficients = &u.row(j, n)[j + 1..];
        let solved = &*below;
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
            let inverse = diagonal_inverse(u, j, m);
            for y in row {
                y.mod_mul_assign(inverse, m);
            }
        }
    }
}

// This is equivalent to `nmod_mat_solve_triu_recursive` from `nmod_mat/solve_triu.c`, FLINT 3.6.0,
// where `xs` is both `X` and `B`.
fn solve_upper_recursive<T: PrimitiveUnsigned>(
    xs: &mut [T],
    u: BlockRef<'_, T>,
    n: usize,
    k: usize,
    m: T,
    unit: bool,
) {
    if n == 0 || k == 0 {
        return;
    }
    let r = n >> 1;
    // Denoting the inverse of M by M^, we have: [A B]^ [X] = [A^ (X - B D^ Y)] [0 D]  [Y] = [    D^
    // Y       ]
    let ua = u;
    let ud = BlockRef {
        row: u.row + r,
        column: u.column + r,
        ..u
    };
    let (xx, xy) = xs.split_at_mut(r * k);
    solve_upper(xy, ud, n - r, k, m, unit);
    matrix_mod_sub_mul(
        xx,
        &u.xs[u.row * u.width + u.column + r..],
        Strides {
            out: k,
            xs: u.width,
        },
        xy,
        ProductShape {
            ar: r,
            br: n - r,
            bc: k,
        },
        m,
    );
    solve_upper(xx, ua, r, k, m, unit);
}

// Solves `u` X = B modulo `m`, where `u` is an `n` by `n` upper triangular block and `xs` holds the
// `n` by `k` matrix B, which is replaced by X.
//
// This is equivalent to `nmod_mat_solve_triu` from `nmod_mat/solve_triu.c`, FLINT 3.6.0, where `xs`
// is both `X` and `B`.
pub(crate) fn solve_upper<T: PrimitiveUnsigned>(
    xs: &mut [T],
    u: BlockRef<'_, T>,
    n: usize,
    k: usize,
    m: T,
    unit: bool,
) {
    if n < SOLVE_TRI_ROWS_CUTOFF || k < SOLVE_TRI_COLUMNS_CUTOFF {
        solve_upper_classical(xs, u, n, k, m, unit);
    } else {
        solve_upper_recursive(xs, u, n, k, m, unit);
    }
}

fn mod_solve_upper_triangular_in_place<T: PrimitiveUnsigned>(
    u: &UnsignedMatrix<T>,
    b: &mut UnsignedMatrix<T>,
    m: T,
    unit: bool,
) {
    assert_triangular_arguments(u, b, m);
    if m == T::ONE {
        // Every element of `b` is 0.
        return;
    }
    let ur = BlockRef {
        xs: &u.elements,
        width: u.width,
        row: 0,
        column: 0,
    };
    solve_upper(&mut b.elements, ur, u.height, b.width, m, unit);
}

impl<T: PrimitiveUnsigned> ModSolveUpperTriangular<Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a upper triangular [`UnsignedMatrix`],
    /// taking both by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. $U$ must be square, and
    /// $B$ must have the same height. Only the elements of $U$ on and above its diagonal are used;
    /// the ones below it are ignored. If `unit_diagonal` is `true`, the diagonal elements are taken
    /// to be 1 and are ignored too; otherwise, each must be invertible modulo $m$. Either way, the
    /// solution is unique. If $U$ is $n \times n$ and $B$ is $n \times k$, then $X$ is the $n
    /// \times k$ matrix with
    ///
    /// $$
    /// X_{ij} = d_i^{-1} \left ( B_{ij} - \sum_{l=i+1}^{n-1} U_{il}X_{lj} \right ) \bmod m,
    /// $$
    ///
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $U_{ii}$ otherwise.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2k)$
    ///
    /// $M(n, k) = O(n^2 + nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $U$, and $k$ is the width
    /// of $B$.
    ///
    /// # Panics
    /// Panics if `m` is zero, if $U$ is not square, if the heights of $U$ and $B$ differ, if any
    /// element of either is greater than or equal to `m`, or if `unit_diagonal` is `false` and a
    /// diagonal element of $U$ is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolveUpperTriangular;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(b, 7, false).to_string(),
    ///     "[5; 4]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(b, 7, true).to_string(),
    ///     "[2; 2]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(b, 7, false).to_string(),
    ///     "[4, 4; 0, 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_solve_triu` from `nmod_mat/solve_triu.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible or an element is not reduced, which
    /// FLINT does not check.
    fn mod_solve_upper_triangular(self, mut b: Self, m: T, unit_diagonal: bool) -> Self {
        mod_solve_upper_triangular_in_place(&self, &mut b, m, unit_diagonal);
        b
    }
}

impl<T: PrimitiveUnsigned> ModSolveUpperTriangular<&Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a upper triangular [`UnsignedMatrix`],
    /// taking the triangular matrix by value and the right-hand side by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. $U$ must be square, and
    /// $B$ must have the same height. Only the elements of $U$ on and above its diagonal are used;
    /// the ones below it are ignored. If `unit_diagonal` is `true`, the diagonal elements are taken
    /// to be 1 and are ignored too; otherwise, each must be invertible modulo $m$. Either way, the
    /// solution is unique. If $U$ is $n \times n$ and $B$ is $n \times k$, then $X$ is the $n
    /// \times k$ matrix with
    ///
    /// $$
    /// X_{ij} = d_i^{-1} \left ( B_{ij} - \sum_{l=i+1}^{n-1} U_{il}X_{lj} \right ) \bmod m,
    /// $$
    ///
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $U_{ii}$ otherwise.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2k)$
    ///
    /// $M(n, k) = O(n^2 + nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $U$, and $k$ is the width
    /// of $B$.
    ///
    /// # Panics
    /// Panics if `m` is zero, if $U$ is not square, if the heights of $U$ and $B$ differ, if any
    /// element of either is greater than or equal to `m`, or if `unit_diagonal` is `false` and a
    /// diagonal element of $U$ is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolveUpperTriangular;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(&b, 7, false).to_string(),
    ///     "[5; 4]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(&b, 7, true).to_string(),
    ///     "[2; 2]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(&b, 7, false).to_string(),
    ///     "[4, 4; 0, 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_solve_triu` from `nmod_mat/solve_triu.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible or an element is not reduced, which
    /// FLINT does not check.
    fn mod_solve_upper_triangular(self, b: &Self, m: T, unit_diagonal: bool) -> Self {
        let mut b = b.clone();
        mod_solve_upper_triangular_in_place(&self, &mut b, m, unit_diagonal);
        b
    }
}

impl<T: PrimitiveUnsigned> ModSolveUpperTriangular<UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a upper triangular [`UnsignedMatrix`],
    /// taking the triangular matrix by reference and the right-hand side by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. $U$ must be square, and
    /// $B$ must have the same height. Only the elements of $U$ on and above its diagonal are used;
    /// the ones below it are ignored. If `unit_diagonal` is `true`, the diagonal elements are taken
    /// to be 1 and are ignored too; otherwise, each must be invertible modulo $m$. Either way, the
    /// solution is unique. If $U$ is $n \times n$ and $B$ is $n \times k$, then $X$ is the $n
    /// \times k$ matrix with
    ///
    /// $$
    /// X_{ij} = d_i^{-1} \left ( B_{ij} - \sum_{l=i+1}^{n-1} U_{il}X_{lj} \right ) \bmod m,
    /// $$
    ///
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $U_{ii}$ otherwise.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2k)$
    ///
    /// $M(n, k) = O(n^2 + nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $U$, and $k$ is the width
    /// of $B$.
    ///
    /// # Panics
    /// Panics if `m` is zero, if $U$ is not square, if the heights of $U$ and $B$ differ, if any
    /// element of either is greater than or equal to `m`, or if `unit_diagonal` is `false` and a
    /// diagonal element of $U$ is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolveUpperTriangular;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(b, 7, false).to_string(),
    ///     "[5; 4]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(b, 7, true).to_string(),
    ///     "[2; 2]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(b, 7, false).to_string(),
    ///     "[4, 4; 0, 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_solve_triu` from `nmod_mat/solve_triu.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible or an element is not reduced, which
    /// FLINT does not check.
    fn mod_solve_upper_triangular(
        self,
        mut b: UnsignedMatrix<T>,
        m: T,
        unit_diagonal: bool,
    ) -> UnsignedMatrix<T> {
        mod_solve_upper_triangular_in_place(self, &mut b, m, unit_diagonal);
        b
    }
}

impl<T: PrimitiveUnsigned> ModSolveUpperTriangular<&UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a upper triangular [`UnsignedMatrix`],
    /// taking both by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. $U$ must be square, and
    /// $B$ must have the same height. Only the elements of $U$ on and above its diagonal are used;
    /// the ones below it are ignored. If `unit_diagonal` is `true`, the diagonal elements are taken
    /// to be 1 and are ignored too; otherwise, each must be invertible modulo $m$. Either way, the
    /// solution is unique. If $U$ is $n \times n$ and $B$ is $n \times k$, then $X$ is the $n
    /// \times k$ matrix with
    ///
    /// $$
    /// X_{ij} = d_i^{-1} \left ( B_{ij} - \sum_{l=i+1}^{n-1} U_{il}X_{lj} \right ) \bmod m,
    /// $$
    ///
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $U_{ii}$ otherwise.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2k)$
    ///
    /// $M(n, k) = O(n^2 + nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $U$, and $k$ is the width
    /// of $B$.
    ///
    /// # Panics
    /// Panics if `m` is zero, if $U$ is not square, if the heights of $U$ and $B$ differ, if any
    /// element of either is greater than or equal to `m`, or if `unit_diagonal` is `false` and a
    /// diagonal element of $U$ is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolveUpperTriangular;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(&b, 7, false).to_string(),
    ///     "[5; 4]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(&b, 7, true).to_string(),
    ///     "[2; 2]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(&b, 7, false).to_string(),
    ///     "[4, 4; 0, 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_solve_triu` from `nmod_mat/solve_triu.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible or an element is not reduced, which
    /// FLINT does not check.
    fn mod_solve_upper_triangular(
        self,
        b: &UnsignedMatrix<T>,
        m: T,
        unit_diagonal: bool,
    ) -> UnsignedMatrix<T> {
        let mut b = b.clone();
        mod_solve_upper_triangular_in_place(self, &mut b, m, unit_diagonal);
        b
    }
}
