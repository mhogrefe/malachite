// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011, 2022 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::mul::ProductShape;
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_solve_lower_triangular::{
    NaturalBlockRef, SOLVE_TRI_CUTOFF, assert_triangular_arguments, diagonal_inverse, sub_mul,
};
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use alloc::vec;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, ModAssign, ModMulAssign, ModSolveUpperTriangular, ModSubAssign,
};
use malachite_base::num::basic::traits::Zero;

// Solves `u` X = B modulo `m` by substitution, where `u` is an `n` by `n` upper triangular block
// and `xs` holds the `n` by `k` matrix B, which is replaced by X. One row of X is computed at a
// time, from the last up: the sums for row j, one for each column, are computed exactly over the
// rows below it and reduced once.
//
// This is equivalent to `gr_mat_nonsingular_solve_triu_classical` from
// `gr_mat/nonsingular_solve_triu.c`, FLINT 3.6.0, where `xs` is both `X` and `B`, except that FLINT
// computes one column of X at a time. The results are the same.
fn solve_upper_classical(
    xs: &mut [Natural],
    u: NaturalBlockRef<'_>,
    n: usize,
    k: usize,
    m: &Natural,
    unit: bool,
) {
    if n == 0 || k == 0 {
        return;
    }
    let mut acc = vec![Natural::ZERO; k];
    for j in (0..n).rev() {
        let (row, below) = xs[j * k..].split_at_mut(k);
        acc.fill(Natural::ZERO);
        for (c, x_row) in u.row(j, n)[j + 1..].iter().zip(below.chunks_exact(k)) {
            if *c != 0u32 {
                for (a, x) in acc.iter_mut().zip(x_row) {
                    a.add_mul_assign(c, x);
                }
            }
        }
        for (y, a) in row.iter_mut().zip(acc.iter_mut()) {
            a.mod_assign(m);
            y.mod_sub_assign(&*a, m);
        }
        if !unit {
            let inverse = diagonal_inverse(u, j, m);
            for y in row {
                y.mod_mul_assign(&inverse, m);
            }
        }
    }
}

// This is equivalent to `gr_mat_nonsingular_solve_triu_recursive` from
// `gr_mat/nonsingular_solve_triu.c`, FLINT 3.6.0, where `xs` is both `X` and `B`.
fn solve_upper_recursive(
    xs: &mut [Natural],
    u: NaturalBlockRef<'_>,
    n: usize,
    k: usize,
    m: &Natural,
    unit: bool,
) {
    if n == 0 || k == 0 {
        return;
    }
    let r = n >> 1;
    // Denoting the inverse of M by M^, we have: [A B]^ [X] = [A^ (X - B D^ Y)] [0 D]  [Y] = [    D^
    // Y       ]
    let ua = u;
    let ub = u.copy(0, r, r, n - r);
    let ud = NaturalBlockRef {
        row: u.row + r,
        column: u.column + r,
        ..u
    };
    let (xx, xy) = xs.split_at_mut(r * k);
    solve_upper(xy, ud, n - r, k, m, unit);
    sub_mul(
        xx,
        &ub,
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
// This is equivalent to `gr_mat_nonsingular_solve_triu_generic` from
// `gr_mat/nonsingular_solve_triu.c`, FLINT 3.6.0, where `xs` is both `X` and `B`.
pub(crate) fn solve_upper(
    xs: &mut [Natural],
    u: NaturalBlockRef<'_>,
    n: usize,
    k: usize,
    m: &Natural,
    unit: bool,
) {
    if n < SOLVE_TRI_CUTOFF || k < SOLVE_TRI_CUTOFF {
        solve_upper_classical(xs, u, n, k, m, unit);
    } else {
        solve_upper_recursive(xs, u, n, k, m, unit);
    }
}

fn mod_solve_upper_triangular_in_place(
    u: &NaturalMatrix,
    b: &mut NaturalMatrix,
    m: &Natural,
    unit: bool,
) {
    assert_triangular_arguments(u, b, m);
    if let Natural(Small(small_m)) = *m {
        *b = NaturalMatrix::from(to_limb_matrix(u).mod_solve_upper_triangular(
            to_limb_matrix(b),
            small_m,
            unit,
        ));
    } else {
        let ur = NaturalBlockRef {
            xs: &u.elements,
            width: u.width,
            row: 0,
            column: 0,
        };
        solve_upper(&mut b.elements, ur, u.height, b.width, m, unit);
    }
}

impl ModSolveUpperTriangular<Self, Natural> for NaturalMatrix {
    type Output = Self;

    /// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a upper triangular [`NaturalMatrix`],
    /// taking the triangular matrix by value, the right-hand side by value, and the modulus by
    /// value.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $U_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $U$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(b, Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[5; 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(b, Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[2; 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_triu` from `fmpz_mod_mat/solve_triu.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_upper_triangular(self, mut b: Self, m: Natural, unit_diagonal: bool) -> Self {
        mod_solve_upper_triangular_in_place(&self, &mut b, &m, unit_diagonal);
        b
    }
}

impl ModSolveUpperTriangular<Self, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a upper triangular [`NaturalMatrix`],
    /// taking the triangular matrix by value, the right-hand side by value, and the modulus by
    /// reference.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $U_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $U$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(b, &Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[5; 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(b, &Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[2; 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_triu` from `fmpz_mod_mat/solve_triu.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_upper_triangular(self, mut b: Self, m: &Natural, unit_diagonal: bool) -> Self {
        mod_solve_upper_triangular_in_place(&self, &mut b, m, unit_diagonal);
        b
    }
}

impl ModSolveUpperTriangular<&Self, Natural> for NaturalMatrix {
    type Output = Self;

    /// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a upper triangular [`NaturalMatrix`],
    /// taking the triangular matrix by value, the right-hand side by reference, and the modulus by
    /// value.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $U_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $U$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(&b, Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[5; 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(&b, Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[2; 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_triu` from `fmpz_mod_mat/solve_triu.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_upper_triangular(self, b: &Self, m: Natural, unit_diagonal: bool) -> Self {
        let mut b = b.clone();
        mod_solve_upper_triangular_in_place(&self, &mut b, &m, unit_diagonal);
        b
    }
}

impl ModSolveUpperTriangular<&Self, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a upper triangular [`NaturalMatrix`],
    /// taking the triangular matrix by value, the right-hand side by reference, and the modulus by
    /// reference.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $U_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $U$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(&b, &Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[5; 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_upper_triangular(&b, &Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[2; 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_triu` from `fmpz_mod_mat/solve_triu.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_upper_triangular(self, b: &Self, m: &Natural, unit_diagonal: bool) -> Self {
        let mut b = b.clone();
        mod_solve_upper_triangular_in_place(&self, &mut b, m, unit_diagonal);
        b
    }
}

impl ModSolveUpperTriangular<NaturalMatrix, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a upper triangular [`NaturalMatrix`],
    /// taking the triangular matrix by reference, the right-hand side by value, and the modulus by
    /// value.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $U_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $U$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(b, Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[5; 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(b, Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[2; 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_triu` from `fmpz_mod_mat/solve_triu.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_upper_triangular(
        self,
        mut b: NaturalMatrix,
        m: Natural,
        unit_diagonal: bool,
    ) -> NaturalMatrix {
        mod_solve_upper_triangular_in_place(self, &mut b, &m, unit_diagonal);
        b
    }
}

impl ModSolveUpperTriangular<NaturalMatrix, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a upper triangular [`NaturalMatrix`],
    /// taking the triangular matrix by reference, the right-hand side by value, and the modulus by
    /// reference.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $U_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $U$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(b, &Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[5; 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(b, &Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[2; 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_triu` from `fmpz_mod_mat/solve_triu.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_upper_triangular(
        self,
        mut b: NaturalMatrix,
        m: &Natural,
        unit_diagonal: bool,
    ) -> NaturalMatrix {
        mod_solve_upper_triangular_in_place(self, &mut b, m, unit_diagonal);
        b
    }
}

impl ModSolveUpperTriangular<&NaturalMatrix, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a upper triangular [`NaturalMatrix`],
    /// taking the triangular matrix by reference, the right-hand side by reference, and the modulus
    /// by value.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $U_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $U$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(&b, Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[5; 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(&b, Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[2; 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_triu` from `fmpz_mod_mat/solve_triu.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_upper_triangular(
        self,
        b: &NaturalMatrix,
        m: Natural,
        unit_diagonal: bool,
    ) -> NaturalMatrix {
        let mut b = b.clone();
        mod_solve_upper_triangular_in_place(self, &mut b, &m, unit_diagonal);
        b
    }
}

impl ModSolveUpperTriangular<&NaturalMatrix, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Solves $UX = B$ modulo $m$ for $X$, where $U$ is a upper triangular [`NaturalMatrix`],
    /// taking the triangular matrix by reference, the right-hand side by reference, and the modulus
    /// by reference.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $U_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $U$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(&b, &Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[5; 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 3; 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_upper_triangular(&b, &Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[2; 2]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_triu` from `fmpz_mod_mat/solve_triu.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_upper_triangular(
        self,
        b: &NaturalMatrix,
        m: &Natural,
        unit_diagonal: bool,
    ) -> NaturalMatrix {
        let mut b = b.clone();
        mod_solve_upper_triangular_in_place(self, &mut b, m, unit_diagonal);
        b
    }
}
