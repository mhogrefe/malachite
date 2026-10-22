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

use crate::integer_matrix::arithmetic::mul::{ProductShape, matrix_mul};
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, DivModPrecomputed, ModAssign, ModInverse, ModIsReduced, ModMulAssign,
    ModSolveLowerTriangular, ModSubAssign,
};
use malachite_base::num::basic::traits::Zero;

// Below this many rows or columns of the right-hand side, a triangular system is solved with the
// classical algorithm.
//
// This is the cutoff from `gr_mat_nonsingular_solve_tril_generic` in
// `gr_mat/nonsingular_solve_tril.c`, FLINT 3.6.0.
pub(crate) const SOLVE_TRI_CUTOFF: usize = 10;

// A block of a matrix, referenced in place: the matrix's elements and width, and the row and column
// of the block's top-left element.
#[derive(Clone, Copy)]
pub(crate) struct NaturalBlockRef<'a> {
    pub(crate) xs: &'a [Natural],
    pub(crate) width: usize,
    pub(crate) row: usize,
    pub(crate) column: usize,
}

impl NaturalBlockRef<'_> {
    // Row `i` of the block, `columns` elements long.
    pub(crate) fn row(&self, i: usize, columns: usize) -> &[Natural] {
        let start = (self.row + i) * self.width + self.column;
        &self.xs[start..start + columns]
    }

    // A copy of the `rows` by `columns` block of this block whose top-left element is in row `row`
    // and column `column`.
    pub(crate) fn copy(
        &self,
        row: usize,
        rows: usize,
        column: usize,
        columns: usize,
    ) -> Vec<Natural> {
        let mut out = Vec::with_capacity(rows * columns);
        for i in row..row + rows {
            out.extend_from_slice(&self.row(i, column + columns)[column..]);
        }
        out
    }
}

// The inverse modulo `m` of the diagonal element in row `i` of the block `t`.
pub(crate) fn diagonal_inverse(t: NaturalBlockRef<'_>, i: usize, m: &Natural) -> Natural {
    (&t.row(i, i + 1)[i])
        .mod_inverse(m)
        .expect("a diagonal element of the triangular matrix is not invertible mod m")
}

// Subtracts the product of the `shape.ar` by `shape.br` matrix with elements `xs` and the
// `shape.br` by `shape.bc` matrix with elements `ys` from the matrix with elements `out`, modulo
// `m`. The product is computed exactly and each of its elements reduced once.
//
// This is equivalent to the `gr_mat_mul` and `gr_mat_sub` calls in
// `gr_mat_nonsingular_solve_tril_recursive` from `gr_mat/nonsingular_solve_tril.c`, FLINT 3.6.0,
// with `gr_mat_mul` computed as `fmpz_mod_mat_mul` does.
pub(crate) fn sub_mul(
    out: &mut [Natural],
    xs: &[Natural],
    ys: &[Natural],
    shape: ProductShape,
    m: &Natural,
) {
    let data = Natural::precompute_div_mod_data(m);
    for (z, p) in out.iter_mut().zip(matrix_mul(xs, ys, shape)) {
        z.mod_sub_assign(p.div_mod_precomputed(m, &data).1, m);
    }
}

// Solves `l` X = B modulo `m` by substitution, where `l` is an `n` by `n` lower triangular block
// and `xs` holds the `n` by `k` matrix B, which is replaced by X. One row of X is computed at a
// time: the sums for row j, one for each column, are computed exactly over the rows above it and
// reduced once.
//
// This is equivalent to `gr_mat_nonsingular_solve_tril_classical` from
// `gr_mat/nonsingular_solve_tril.c`, FLINT 3.6.0, where `xs` is both `X` and `B`, except that FLINT
// computes one column of X at a time. The results are the same.
fn solve_lower_classical(
    xs: &mut [Natural],
    l: NaturalBlockRef<'_>,
    n: usize,
    k: usize,
    m: &Natural,
    unit: bool,
) {
    if n == 0 || k == 0 {
        return;
    }
    let mut acc = vec![Natural::ZERO; k];
    for j in 0..n {
        let (above, rest) = xs.split_at_mut(j * k);
        let row = &mut rest[..k];
        acc.fill(Natural::ZERO);
        for (c, x_row) in l.row(j, j).iter().zip(above.chunks_exact(k)) {
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
            let inverse = diagonal_inverse(l, j, m);
            for y in row {
                y.mod_mul_assign(&inverse, m);
            }
        }
    }
}

// This is equivalent to `gr_mat_nonsingular_solve_tril_recursive` from
// `gr_mat/nonsingular_solve_tril.c`, FLINT 3.6.0, where `xs` is both `X` and `B`.
fn solve_lower_recursive(
    xs: &mut [Natural],
    l: NaturalBlockRef<'_>,
    n: usize,
    k: usize,
    m: &Natural,
    unit: bool,
) {
    if n == 0 || k == 0 {
        return;
    }
    let r = n >> 1;
    // Denoting the inverse of M by M^, we have: [A 0]^ [X] = [A^          0 ] [X] = [A^ X] [C D]
    // [Y] = [-D^ C A^    D^] [Y] = [D^ (Y - C A^ X)]
    let la = l;
    let lc = l.copy(r, n - r, 0, r);
    let ld = NaturalBlockRef {
        row: l.row + r,
        column: l.column + r,
        ..l
    };
    let (xx, xy) = xs.split_at_mut(r * k);
    solve_lower(xx, la, r, k, m, unit);
    sub_mul(
        xy,
        &lc,
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
// This is equivalent to `gr_mat_nonsingular_solve_tril_generic` from
// `gr_mat/nonsingular_solve_tril.c`, FLINT 3.6.0, where `xs` is both `X` and `B`.
pub(crate) fn solve_lower(
    xs: &mut [Natural],
    l: NaturalBlockRef<'_>,
    n: usize,
    k: usize,
    m: &Natural,
    unit: bool,
) {
    if n < SOLVE_TRI_CUTOFF || k < SOLVE_TRI_CUTOFF {
        solve_lower_classical(xs, l, n, k, m, unit);
    } else {
        solve_lower_recursive(xs, l, n, k, m, unit);
    }
}

pub(crate) fn assert_triangular_arguments(a: &NaturalMatrix, b: &NaturalMatrix, m: &Natural) {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert_eq!(a.height, a.width, "the triangular matrix must be square");
    assert_eq!(
        a.height, b.height,
        "the height of the triangular matrix must equal the height of the right-hand side"
    );
    assert!(
        a.mod_is_reduced(m),
        "the triangular matrix must be reduced mod m, but {a} has an element >= {m}"
    );
    assert!(
        b.mod_is_reduced(m),
        "the right-hand side must be reduced mod m, but {b} has an element >= {m}"
    );
}

fn mod_solve_lower_triangular_in_place(
    l: &NaturalMatrix,
    b: &mut NaturalMatrix,
    m: &Natural,
    unit: bool,
) {
    assert_triangular_arguments(l, b, m);
    if let Natural(Small(small_m)) = *m {
        *b = NaturalMatrix::from(to_limb_matrix(l).mod_solve_lower_triangular(
            to_limb_matrix(b),
            small_m,
            unit,
        ));
    } else {
        let lr = NaturalBlockRef {
            xs: &l.elements,
            width: l.width,
            row: 0,
            column: 0,
        };
        solve_lower(&mut b.elements, lr, l.height, b.width, m, unit);
    }
}

impl ModSolveLowerTriangular<Self, Natural> for NaturalMatrix {
    type Output = Self;

    /// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a lower triangular [`NaturalMatrix`],
    /// taking the triangular matrix by value, the right-hand side by value, and the modulus by
    /// value.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $L_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $L$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(b, Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[4; 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(b, Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[1; 6]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_tril` from `fmpz_mod_mat/solve_tril.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_lower_triangular(self, mut b: Self, m: Natural, unit_diagonal: bool) -> Self {
        mod_solve_lower_triangular_in_place(&self, &mut b, &m, unit_diagonal);
        b
    }
}

impl ModSolveLowerTriangular<Self, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a lower triangular [`NaturalMatrix`],
    /// taking the triangular matrix by value, the right-hand side by value, and the modulus by
    /// reference.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $L_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $L$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(b, &Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[4; 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(b, &Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[1; 6]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_tril` from `fmpz_mod_mat/solve_tril.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_lower_triangular(self, mut b: Self, m: &Natural, unit_diagonal: bool) -> Self {
        mod_solve_lower_triangular_in_place(&self, &mut b, m, unit_diagonal);
        b
    }
}

impl ModSolveLowerTriangular<&Self, Natural> for NaturalMatrix {
    type Output = Self;

    /// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a lower triangular [`NaturalMatrix`],
    /// taking the triangular matrix by value, the right-hand side by reference, and the modulus by
    /// value.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $L_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $L$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(&b, Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[4; 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(&b, Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[1; 6]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_tril` from `fmpz_mod_mat/solve_tril.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_lower_triangular(self, b: &Self, m: Natural, unit_diagonal: bool) -> Self {
        let mut b = b.clone();
        mod_solve_lower_triangular_in_place(&self, &mut b, &m, unit_diagonal);
        b
    }
}

impl ModSolveLowerTriangular<&Self, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a lower triangular [`NaturalMatrix`],
    /// taking the triangular matrix by value, the right-hand side by reference, and the modulus by
    /// reference.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $L_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $L$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(&b, &Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[4; 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve_lower_triangular(&b, &Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[1; 6]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_tril` from `fmpz_mod_mat/solve_tril.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_lower_triangular(self, b: &Self, m: &Natural, unit_diagonal: bool) -> Self {
        let mut b = b.clone();
        mod_solve_lower_triangular_in_place(&self, &mut b, m, unit_diagonal);
        b
    }
}

impl ModSolveLowerTriangular<NaturalMatrix, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a lower triangular [`NaturalMatrix`],
    /// taking the triangular matrix by reference, the right-hand side by value, and the modulus by
    /// value.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $L_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $L$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(b, Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[4; 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(b, Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[1; 6]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_tril` from `fmpz_mod_mat/solve_tril.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_lower_triangular(
        self,
        mut b: NaturalMatrix,
        m: Natural,
        unit_diagonal: bool,
    ) -> NaturalMatrix {
        mod_solve_lower_triangular_in_place(self, &mut b, &m, unit_diagonal);
        b
    }
}

impl ModSolveLowerTriangular<NaturalMatrix, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a lower triangular [`NaturalMatrix`],
    /// taking the triangular matrix by reference, the right-hand side by value, and the modulus by
    /// reference.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $L_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $L$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(b, &Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[4; 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(b, &Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[1; 6]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_tril` from `fmpz_mod_mat/solve_tril.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_lower_triangular(
        self,
        mut b: NaturalMatrix,
        m: &Natural,
        unit_diagonal: bool,
    ) -> NaturalMatrix {
        mod_solve_lower_triangular_in_place(self, &mut b, m, unit_diagonal);
        b
    }
}

impl ModSolveLowerTriangular<&NaturalMatrix, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a lower triangular [`NaturalMatrix`],
    /// taking the triangular matrix by reference, the right-hand side by reference, and the modulus
    /// by value.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $L_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $L$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(&b, Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[4; 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(&b, Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[1; 6]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_tril` from `fmpz_mod_mat/solve_tril.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_lower_triangular(
        self,
        b: &NaturalMatrix,
        m: Natural,
        unit_diagonal: bool,
    ) -> NaturalMatrix {
        let mut b = b.clone();
        mod_solve_lower_triangular_in_place(self, &mut b, &m, unit_diagonal);
        b
    }
}

impl ModSolveLowerTriangular<&NaturalMatrix, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Solves $LX = B$ modulo $m$ for $X$, where $L$ is a lower triangular [`NaturalMatrix`],
    /// taking the triangular matrix by reference, the right-hand side by reference, and the modulus
    /// by reference.
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
    /// where $d_i$ is 1 if `unit_diagonal` is `true` and $L_{ii}$ otherwise. When $m$ fits in a
    /// limb, the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(n^2 k b \log b \log \log b)$
    ///
    /// $M(n, k, b) = O(n^2 b + nkb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of $L$, $k$ is the width of
    /// $B$, and $b$ is `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(&b, &Natural::from(7u32), false)
    ///         .to_string(),
    ///     "[4; 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve_lower_triangular(&b, &Natural::from(7u32), true)
    ///         .to_string(),
    ///     "[1; 6]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_solve_tril` from `fmpz_mod_mat/solve_tril.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible or an element is not
    /// reduced.
    fn mod_solve_lower_triangular(
        self,
        b: &NaturalMatrix,
        m: &Natural,
        unit_diagonal: bool,
    ) -> NaturalMatrix {
        let mut b = b.clone();
        mod_solve_lower_triangular_in_place(self, &mut b, m, unit_diagonal);
        b
    }
}
