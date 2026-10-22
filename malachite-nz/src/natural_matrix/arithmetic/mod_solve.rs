// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011, 2022 Fredrik Johansson
//
//      Copyright © 2019 William Hart
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
use crate::natural_matrix::arithmetic::mod_lu_decomposition::{lu_in_place, non_invertible_pivot};
use crate::natural_matrix::arithmetic::mod_solve_lower_triangular::{NaturalBlockRef, solve_lower};
use crate::natural_matrix::arithmetic::mod_solve_upper_triangular::solve_upper;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use crate::natural_vector::NaturalVector;
use crate::platform::Limb;
use alloc::borrow::Cow;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivModPrecomputed, ModIsReduced, ModSolve};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::unsigned_matrix::UnsignedMatrix;

// The rows of the matrix with elements `xs` and width `width`, permuted: row `i` of the result is
// row `perm[i]` of the matrix. Owned elements are moved rather than cloned.
fn permute_rows(xs: Cow<'_, [Natural]>, width: usize, perm: &[usize]) -> Vec<Natural> {
    let mut out = Vec::with_capacity(perm.len() * width);
    match xs {
        Cow::Borrowed(xs) => {
            for &p in perm {
                out.extend_from_slice(&xs[p * width..(p + 1) * width]);
            }
        }
        Cow::Owned(mut xs) => {
            for &p in perm {
                out.extend(
                    xs[p * width..(p + 1) * width]
                        .iter_mut()
                        .map(core::mem::take),
                );
            }
        }
    }
    out
}

// Solves PAX = PB for X, given the packed LU decomposition `lu` of the n by n matrix A, which has
// full rank, and the permutation `perm`. Returns the elements of X.
//
// This is equivalent to `gr_mat_nonsingular_solve_lu_precomp` from
// `gr_mat/nonsingular_solve_lu_precomp.c`, FLINT 3.6.0.
fn solve_from_square_lu(
    lu: &[Natural],
    perm: &[usize],
    b: Cow<'_, [Natural]>,
    n: usize,
    k: usize,
    m: &Natural,
) -> Vec<Natural> {
    let mut x = permute_rows(b, k, perm);
    let lr = NaturalBlockRef {
        xs: lu,
        width: n,
        row: 0,
        column: 0,
    };
    solve_lower(&mut x, lr, n, k, m, true);
    solve_upper(&mut x, lr, n, k, m, false);
    x
}

// Solves AX = B modulo the prime `m`, which does not fit in a limb, for a square n by n matrix A,
// consuming it, and returns the elements of X, or `None` if A does not have full rank. B is n by k.
//
// This is equivalent to `gr_mat_nonsingular_solve_lu` from `gr_mat/nonsingular_solve_lu.c`, FLINT
// 3.6.0, which `fmpz_mod_mat_solve` calls, except that it panics at a pivot that is not invertible,
// where FLINT reports that there is no solution.
pub(crate) fn solve_nonsingular(
    mut a: NaturalMatrix,
    b: Cow<'_, [Natural]>,
    k: usize,
    m: &Natural,
) -> Option<Vec<Natural>> {
    let n = a.height;
    if n == 0 || k == 0 {
        return Some(vec![Natural::ZERO; n * k]);
    }
    let (perm, rank) = lu_in_place(&mut a, m, true).unwrap_or_else(|| non_invertible_pivot(m));
    if rank == n {
        Some(solve_from_square_lu(&a.elements, &perm, b, n, k, m))
    } else {
        None
    }
}

// Solves AX = B modulo the prime `m`, which does not fit in a limb, for an h by w matrix A,
// consuming it, and returns the elements of a solution X, or `None` if there is none. B is h by k
// and X is w by k. When A is square and has full rank, the solution is computed as by
// `gr_mat_nonsingular_solve_lu`; otherwise, the LU decomposition is reused to find a solution as
// `gr_mat_solve_field` does.
//
// This is equivalent to `gr_mat_solve_field` from `gr_mat/solve_field.c`, FLINT 3.6.0, which
// `fmpz_mod_mat_can_solve` calls, except that it panics at a pivot that is not invertible, where
// FLINT reports that there is no solution.
fn solve_large(
    mut a: NaturalMatrix,
    b: Cow<'_, [Natural]>,
    k: usize,
    m: &Natural,
) -> Option<Vec<Natural>> {
    let (h, w) = (a.height, a.width);
    if h == 0 || k == 0 {
        return Some(vec![Natural::ZERO; w * k]);
    }
    if w == 0 {
        return if b.iter().all(|x| *x == 0u32) {
            Some(Vec::new())
        } else {
            None
        };
    }
    let (perm, rank) = lu_in_place(&mut a, m, false).unwrap_or_else(|| non_invertible_pivot(m));
    let lu = &a.elements;
    if h == w && rank == h {
        return Some(solve_from_square_lu(lu, &perm, b, h, k, m));
    }
    let pb = permute_rows(b, k, &perm);
    // The pivot columns, and the rank by rank upper triangular matrix made of them.
    let mut pivots = Vec::with_capacity(rank);
    let mut lu2 = vec![Natural::ZERO; rank * rank];
    let mut col = 0;
    for i in 0..rank {
        while lu[i * w + col] == 0u32 {
            col += 1;
        }
        pivots.push(col);
        for j in 0..rank {
            lu2[j * rank + i] = lu[j * w + col].clone();
        }
        col += 1;
    }
    let lr = NaturalBlockRef {
        xs: lu,
        width: w,
        row: 0,
        column: 0,
    };
    let mut y = pb[..rank * k].to_vec();
    solve_lower(&mut y, lr, rank, k, m, true);
    if h > rank {
        // The remaining rows of L, times Y, must equal the remaining rows of PB.
        let l_rest = lr.copy(rank, h - rank, 0, rank);
        let product = matrix_mul(
            &l_rest,
            &y,
            ProductShape {
                ar: h - rank,
                br: rank,
                bc: k,
            },
        );
        let data = Natural::precompute_div_mod_data(m);
        if product
            .iter()
            .zip(&pb[rank * k..])
            .any(|(p, b)| p.div_mod_precomputed(m, &data).1 != *b)
        {
            return None;
        }
    }
    solve_upper(
        &mut y,
        NaturalBlockRef {
            xs: &lu2,
            width: rank,
            row: 0,
            column: 0,
        },
        rank,
        k,
        m,
        false,
    );
    let mut x = vec![Natural::ZERO; w * k];
    for (i, &p) in pivots.iter().enumerate() {
        for (z, y) in x[p * k..(p + 1) * k]
            .iter_mut()
            .zip(&mut y[i * k..(i + 1) * k])
        {
            *z = core::mem::take(y);
        }
    }
    Some(x)
}

fn assert_arguments(a: &NaturalMatrix, b_height: usize, m: &Natural) {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert_eq!(
        a.height, b_height,
        "the height of the matrix must equal the height of the right-hand side"
    );
    assert!(
        a.mod_is_reduced(m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

// Solves AX = B, where B is the `height` by `k` matrix with elements `b`, and returns the elements
// of X. Neither A nor B is cloned unless the modulus does not fit in a limb and A is borrowed.
fn solve(
    a: Cow<'_, NaturalMatrix>,
    b: Cow<'_, [Natural]>,
    k: usize,
    m: &Natural,
) -> Option<Vec<Natural>> {
    if let Natural(Small(small_m)) = *m {
        let b = UnsignedMatrix::from_owned_elements(
            a.height(),
            u64::exact_from(k),
            b.iter().map(Limb::wrapping_from).collect(),
        );
        to_limb_matrix(&a)
            .mod_solve(b, small_m)
            .map(|x| NaturalMatrix::from(x).elements)
    } else {
        solve_large(a.into_owned(), b, k, m)
    }
}

fn mod_solve_matrix(
    a: Cow<'_, NaturalMatrix>,
    b: Cow<'_, NaturalMatrix>,
    m: &Natural,
) -> Option<NaturalMatrix> {
    assert_arguments(&a, b.height, m);
    assert!(
        b.mod_is_reduced(m),
        "the right-hand side must be reduced mod m, but {b} has an element >= {m}"
    );
    let width = a.width;
    let k = b.width;
    let b = match b {
        Cow::Borrowed(b) => Cow::Borrowed(b.elements.as_slice()),
        Cow::Owned(b) => Cow::Owned(b.elements),
    };
    solve(a, b, k, m).map(|elements| NaturalMatrix {
        elements,
        height: width,
        width: k,
    })
}

fn mod_solve_vector(
    a: Cow<'_, NaturalMatrix>,
    b: Cow<'_, NaturalVector>,
    m: &Natural,
) -> Option<NaturalVector> {
    assert_arguments(&a, b.elements.len(), m);
    assert!(
        b.mod_is_reduced(m),
        "the right-hand side must be reduced mod m, but {b} has an element >= {m}"
    );
    let b = match b {
        Cow::Borrowed(b) => Cow::Borrowed(b.elements.as_slice()),
        Cow::Owned(b) => Cow::Owned(b.elements),
    };
    solve(a, b, 1, m).map(|elements| NaturalVector { elements })
}

impl ModSolve<Self, Natural> for NaturalMatrix {
    type Output = Self;

    /// Solves $AX = B$ modulo a prime $m$ for $X$, where $A$ and $B$ are [`NaturalMatrix`]es,
    /// taking the matrix by value, the right-hand side by value, and the modulus by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. If $A$ is $h \times w$ and
    /// $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with $AX = B$ modulo $m$, and
    /// `None` is returned if there is none. When there are several, the one returned has zeros in
    /// the rows corresponding to the columns of $A$ that are not pivot columns of its row echelon
    /// form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, b) = O(\min(h, w)(hw + (h + w)k) b \log b \log \log b)$
    ///
    /// $M(h, w, k, b) = O((hw + hk + wk) b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the height of the matrix is not the height of the right-hand side,
    /// if any element of either is greater than or equal to `m`, or if a pivot is not invertible
    /// modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve(b, Natural::from(7u32)).unwrap().to_string(),
    ///     "[6; 1]"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(a.mod_solve(b, Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// using the method of `fmpz_mod_mat_solve` from `fmpz_mod_mat/solve.c` when $A$ is square and
    /// has full rank, except that it panics if the dimensions are incompatible, an element is not
    /// reduced, or a pivot is not invertible, where FLINT reports that there is no solution.
    #[inline]
    fn mod_solve(self, b: Self, m: Natural) -> Option<Self> {
        mod_solve_matrix(Cow::Owned(self), Cow::Owned(b), &m)
    }
}

impl ModSolve<Self, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Solves $AX = B$ modulo a prime $m$ for $X$, where $A$ and $B$ are [`NaturalMatrix`]es,
    /// taking the matrix by value, the right-hand side by value, and the modulus by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. If $A$ is $h \times w$ and
    /// $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with $AX = B$ modulo $m$, and
    /// `None` is returned if there is none. When there are several, the one returned has zeros in
    /// the rows corresponding to the columns of $A$ that are not pivot columns of its row echelon
    /// form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, b) = O(\min(h, w)(hw + (h + w)k) b \log b \log \log b)$
    ///
    /// $M(h, w, k, b) = O((hw + hk + wk) b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the height of the matrix is not the height of the right-hand side,
    /// if any element of either is greater than or equal to `m`, or if a pivot is not invertible
    /// modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve(b, &Natural::from(7u32)).unwrap().to_string(),
    ///     "[6; 1]"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(a.mod_solve(b, &Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// using the method of `fmpz_mod_mat_solve` from `fmpz_mod_mat/solve.c` when $A$ is square and
    /// has full rank, except that it panics if the dimensions are incompatible, an element is not
    /// reduced, or a pivot is not invertible, where FLINT reports that there is no solution.
    #[inline]
    fn mod_solve(self, b: Self, m: &Natural) -> Option<Self> {
        mod_solve_matrix(Cow::Owned(self), Cow::Owned(b), m)
    }
}

impl ModSolve<&Self, Natural> for NaturalMatrix {
    type Output = Self;

    /// Solves $AX = B$ modulo a prime $m$ for $X$, where $A$ and $B$ are [`NaturalMatrix`]es,
    /// taking the matrix by value, the right-hand side by reference, and the modulus by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. If $A$ is $h \times w$ and
    /// $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with $AX = B$ modulo $m$, and
    /// `None` is returned if there is none. When there are several, the one returned has zeros in
    /// the rows corresponding to the columns of $A$ that are not pivot columns of its row echelon
    /// form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, b) = O(\min(h, w)(hw + (h + w)k) b \log b \log \log b)$
    ///
    /// $M(h, w, k, b) = O((hw + hk + wk) b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the height of the matrix is not the height of the right-hand side,
    /// if any element of either is greater than or equal to `m`, or if a pivot is not invertible
    /// modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve(&b, Natural::from(7u32)).unwrap().to_string(),
    ///     "[6; 1]"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(a.mod_solve(&b, Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// using the method of `fmpz_mod_mat_solve` from `fmpz_mod_mat/solve.c` when $A$ is square and
    /// has full rank, except that it panics if the dimensions are incompatible, an element is not
    /// reduced, or a pivot is not invertible, where FLINT reports that there is no solution.
    #[inline]
    fn mod_solve(self, b: &Self, m: Natural) -> Option<Self> {
        mod_solve_matrix(Cow::Owned(self), Cow::Borrowed(b), &m)
    }
}

impl ModSolve<&Self, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Solves $AX = B$ modulo a prime $m$ for $X$, where $A$ and $B$ are [`NaturalMatrix`]es,
    /// taking the matrix by value, the right-hand side by reference, and the modulus by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. If $A$ is $h \times w$ and
    /// $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with $AX = B$ modulo $m$, and
    /// `None` is returned if there is none. When there are several, the one returned has zeros in
    /// the rows corresponding to the columns of $A$ that are not pivot columns of its row echelon
    /// form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, b) = O(\min(h, w)(hw + (h + w)k) b \log b \log \log b)$
    ///
    /// $M(h, w, k, b) = O((hw + hk + wk) b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the height of the matrix is not the height of the right-hand side,
    /// if any element of either is greater than or equal to `m`, or if a pivot is not invertible
    /// modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve(&b, &Natural::from(7u32)).unwrap().to_string(),
    ///     "[6; 1]"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(a.mod_solve(&b, &Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// using the method of `fmpz_mod_mat_solve` from `fmpz_mod_mat/solve.c` when $A$ is square and
    /// has full rank, except that it panics if the dimensions are incompatible, an element is not
    /// reduced, or a pivot is not invertible, where FLINT reports that there is no solution.
    #[inline]
    fn mod_solve(self, b: &Self, m: &Natural) -> Option<Self> {
        mod_solve_matrix(Cow::Owned(self), Cow::Borrowed(b), m)
    }
}

impl ModSolve<NaturalVector, Natural> for NaturalMatrix {
    type Output = NaturalVector;

    /// Solves $Ax = b$ modulo a prime $m$ for $x$, where $A$ is a [`NaturalMatrix`] and $b$ is a
    /// [`NaturalVector`], taking the matrix by value, the vector by value, and the modulus by
    /// value.
    ///
    /// The elements of the matrix and the vector must already be reduced modulo $m$. If $A$ is $h
    /// \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with $Ax = b$
    /// modulo $m$, and `None` is returned if there is none. When there are several, the one
    /// returned has zeros in the positions corresponding to the columns of $A$ that are not pivot
    /// columns of its row echelon form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
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
    /// Panics if `m` is zero, if the height of the matrix is not the length of the vector, if any
    /// element of either is greater than or equal to `m`, or if a pivot is not invertible modulo
    /// `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(
    ///     a.mod_solve(b, Natural::from(7u32)).unwrap().to_string(),
    ///     "(6, 1)"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(a.mod_solve(b, Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// with a right-hand side of one column, except that it panics if the dimensions are
    /// incompatible, an element is not reduced, or a pivot is not invertible, where FLINT reports
    /// that there is no solution.
    #[inline]
    fn mod_solve(self, b: NaturalVector, m: Natural) -> Option<NaturalVector> {
        mod_solve_vector(Cow::Owned(self), Cow::Owned(b), &m)
    }
}

impl ModSolve<NaturalVector, &Natural> for NaturalMatrix {
    type Output = NaturalVector;

    /// Solves $Ax = b$ modulo a prime $m$ for $x$, where $A$ is a [`NaturalMatrix`] and $b$ is a
    /// [`NaturalVector`], taking the matrix by value, the vector by value, and the modulus by
    /// reference.
    ///
    /// The elements of the matrix and the vector must already be reduced modulo $m$. If $A$ is $h
    /// \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with $Ax = b$
    /// modulo $m$, and `None` is returned if there is none. When there are several, the one
    /// returned has zeros in the positions corresponding to the columns of $A$ that are not pivot
    /// columns of its row echelon form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
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
    /// Panics if `m` is zero, if the height of the matrix is not the length of the vector, if any
    /// element of either is greater than or equal to `m`, or if a pivot is not invertible modulo
    /// `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(
    ///     a.mod_solve(b, &Natural::from(7u32)).unwrap().to_string(),
    ///     "(6, 1)"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(a.mod_solve(b, &Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// with a right-hand side of one column, except that it panics if the dimensions are
    /// incompatible, an element is not reduced, or a pivot is not invertible, where FLINT reports
    /// that there is no solution.
    #[inline]
    fn mod_solve(self, b: NaturalVector, m: &Natural) -> Option<NaturalVector> {
        mod_solve_vector(Cow::Owned(self), Cow::Owned(b), m)
    }
}

impl ModSolve<&NaturalVector, Natural> for NaturalMatrix {
    type Output = NaturalVector;

    /// Solves $Ax = b$ modulo a prime $m$ for $x$, where $A$ is a [`NaturalMatrix`] and $b$ is a
    /// [`NaturalVector`], taking the matrix by value, the vector by reference, and the modulus by
    /// value.
    ///
    /// The elements of the matrix and the vector must already be reduced modulo $m$. If $A$ is $h
    /// \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with $Ax = b$
    /// modulo $m$, and `None` is returned if there is none. When there are several, the one
    /// returned has zeros in the positions corresponding to the columns of $A$ that are not pivot
    /// columns of its row echelon form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
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
    /// Panics if `m` is zero, if the height of the matrix is not the length of the vector, if any
    /// element of either is greater than or equal to `m`, or if a pivot is not invertible modulo
    /// `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(
    ///     a.mod_solve(&b, Natural::from(7u32)).unwrap().to_string(),
    ///     "(6, 1)"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(a.mod_solve(&b, Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// with a right-hand side of one column, except that it panics if the dimensions are
    /// incompatible, an element is not reduced, or a pivot is not invertible, where FLINT reports
    /// that there is no solution.
    #[inline]
    fn mod_solve(self, b: &NaturalVector, m: Natural) -> Option<NaturalVector> {
        mod_solve_vector(Cow::Owned(self), Cow::Borrowed(b), &m)
    }
}

impl ModSolve<&NaturalVector, &Natural> for NaturalMatrix {
    type Output = NaturalVector;

    /// Solves $Ax = b$ modulo a prime $m$ for $x$, where $A$ is a [`NaturalMatrix`] and $b$ is a
    /// [`NaturalVector`], taking the matrix by value, the vector by reference, and the modulus by
    /// reference.
    ///
    /// The elements of the matrix and the vector must already be reduced modulo $m$. If $A$ is $h
    /// \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with $Ax = b$
    /// modulo $m$, and `None` is returned if there is none. When there are several, the one
    /// returned has zeros in the positions corresponding to the columns of $A$ that are not pivot
    /// columns of its row echelon form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
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
    /// Panics if `m` is zero, if the height of the matrix is not the length of the vector, if any
    /// element of either is greater than or equal to `m`, or if a pivot is not invertible modulo
    /// `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(
    ///     a.mod_solve(&b, &Natural::from(7u32)).unwrap().to_string(),
    ///     "(6, 1)"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(a.mod_solve(&b, &Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// with a right-hand side of one column, except that it panics if the dimensions are
    /// incompatible, an element is not reduced, or a pivot is not invertible, where FLINT reports
    /// that there is no solution.
    #[inline]
    fn mod_solve(self, b: &NaturalVector, m: &Natural) -> Option<NaturalVector> {
        mod_solve_vector(Cow::Owned(self), Cow::Borrowed(b), m)
    }
}

impl ModSolve<NaturalMatrix, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Solves $AX = B$ modulo a prime $m$ for $X$, where $A$ and $B$ are [`NaturalMatrix`]es,
    /// taking the matrix by reference, the right-hand side by value, and the modulus by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. If $A$ is $h \times w$ and
    /// $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with $AX = B$ modulo $m$, and
    /// `None` is returned if there is none. When there are several, the one returned has zeros in
    /// the rows corresponding to the columns of $A$ that are not pivot columns of its row echelon
    /// form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, b) = O(\min(h, w)(hw + (h + w)k) b \log b \log \log b)$
    ///
    /// $M(h, w, k, b) = O((hw + hk + wk) b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the height of the matrix is not the height of the right-hand side,
    /// if any element of either is greater than or equal to `m`, or if a pivot is not invertible
    /// modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve(b, Natural::from(7u32)).unwrap().to_string(),
    ///     "[6; 1]"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!((&a).mod_solve(b, Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// using the method of `fmpz_mod_mat_solve` from `fmpz_mod_mat/solve.c` when $A$ is square and
    /// has full rank, except that it panics if the dimensions are incompatible, an element is not
    /// reduced, or a pivot is not invertible, where FLINT reports that there is no solution.
    #[inline]
    fn mod_solve(self, b: NaturalMatrix, m: Natural) -> Option<NaturalMatrix> {
        mod_solve_matrix(Cow::Borrowed(self), Cow::Owned(b), &m)
    }
}

impl ModSolve<NaturalMatrix, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Solves $AX = B$ modulo a prime $m$ for $X$, where $A$ and $B$ are [`NaturalMatrix`]es,
    /// taking the matrix by reference, the right-hand side by value, and the modulus by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. If $A$ is $h \times w$ and
    /// $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with $AX = B$ modulo $m$, and
    /// `None` is returned if there is none. When there are several, the one returned has zeros in
    /// the rows corresponding to the columns of $A$ that are not pivot columns of its row echelon
    /// form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, b) = O(\min(h, w)(hw + (h + w)k) b \log b \log \log b)$
    ///
    /// $M(h, w, k, b) = O((hw + hk + wk) b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the height of the matrix is not the height of the right-hand side,
    /// if any element of either is greater than or equal to `m`, or if a pivot is not invertible
    /// modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve(b, &Natural::from(7u32)).unwrap().to_string(),
    ///     "[6; 1]"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!((&a).mod_solve(b, &Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// using the method of `fmpz_mod_mat_solve` from `fmpz_mod_mat/solve.c` when $A$ is square and
    /// has full rank, except that it panics if the dimensions are incompatible, an element is not
    /// reduced, or a pivot is not invertible, where FLINT reports that there is no solution.
    #[inline]
    fn mod_solve(self, b: NaturalMatrix, m: &Natural) -> Option<NaturalMatrix> {
        mod_solve_matrix(Cow::Borrowed(self), Cow::Owned(b), m)
    }
}

impl ModSolve<&NaturalMatrix, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Solves $AX = B$ modulo a prime $m$ for $X$, where $A$ and $B$ are [`NaturalMatrix`]es,
    /// taking the matrix by reference, the right-hand side by reference, and the modulus by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. If $A$ is $h \times w$ and
    /// $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with $AX = B$ modulo $m$, and
    /// `None` is returned if there is none. When there are several, the one returned has zeros in
    /// the rows corresponding to the columns of $A$ that are not pivot columns of its row echelon
    /// form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, b) = O(\min(h, w)(hw + (h + w)k) b \log b \log \log b)$
    ///
    /// $M(h, w, k, b) = O((hw + hk + wk) b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the height of the matrix is not the height of the right-hand side,
    /// if any element of either is greater than or equal to `m`, or if a pivot is not invertible
    /// modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve(&b, Natural::from(7u32)).unwrap().to_string(),
    ///     "[6; 1]"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!((&a).mod_solve(&b, Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// using the method of `fmpz_mod_mat_solve` from `fmpz_mod_mat/solve.c` when $A$ is square and
    /// has full rank, except that it panics if the dimensions are incompatible, an element is not
    /// reduced, or a pivot is not invertible, where FLINT reports that there is no solution.
    #[inline]
    fn mod_solve(self, b: &NaturalMatrix, m: Natural) -> Option<NaturalMatrix> {
        mod_solve_matrix(Cow::Borrowed(self), Cow::Borrowed(b), &m)
    }
}

impl ModSolve<&NaturalMatrix, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Solves $AX = B$ modulo a prime $m$ for $X$, where $A$ and $B$ are [`NaturalMatrix`]es,
    /// taking the matrix by reference, the right-hand side by reference, and the modulus by
    /// reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. If $A$ is $h \times w$ and
    /// $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with $AX = B$ modulo $m$, and
    /// `None` is returned if there is none. When there are several, the one returned has zeros in
    /// the rows corresponding to the columns of $A$ that are not pivot columns of its row echelon
    /// form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, b) = O(\min(h, w)(hw + (h + w)k) b \log b \log \log b)$
    ///
    /// $M(h, w, k, b) = O((hw + hk + wk) b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the height of the matrix is not the height of the right-hand side,
    /// if any element of either is greater than or equal to `m`, or if a pivot is not invertible
    /// modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve(&b, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "[6; 1]"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!((&a).mod_solve(&b, &Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// using the method of `fmpz_mod_mat_solve` from `fmpz_mod_mat/solve.c` when $A$ is square and
    /// has full rank, except that it panics if the dimensions are incompatible, an element is not
    /// reduced, or a pivot is not invertible, where FLINT reports that there is no solution.
    #[inline]
    fn mod_solve(self, b: &NaturalMatrix, m: &Natural) -> Option<NaturalMatrix> {
        mod_solve_matrix(Cow::Borrowed(self), Cow::Borrowed(b), m)
    }
}

impl ModSolve<NaturalVector, Natural> for &NaturalMatrix {
    type Output = NaturalVector;

    /// Solves $Ax = b$ modulo a prime $m$ for $x$, where $A$ is a [`NaturalMatrix`] and $b$ is a
    /// [`NaturalVector`], taking the matrix by reference, the vector by value, and the modulus by
    /// value.
    ///
    /// The elements of the matrix and the vector must already be reduced modulo $m$. If $A$ is $h
    /// \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with $Ax = b$
    /// modulo $m$, and `None` is returned if there is none. When there are several, the one
    /// returned has zeros in the positions corresponding to the columns of $A$ that are not pivot
    /// columns of its row echelon form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
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
    /// Panics if `m` is zero, if the height of the matrix is not the length of the vector, if any
    /// element of either is greater than or equal to `m`, or if a pivot is not invertible modulo
    /// `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve(b, Natural::from(7u32)).unwrap().to_string(),
    ///     "(6, 1)"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!((&a).mod_solve(b, Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// with a right-hand side of one column, except that it panics if the dimensions are
    /// incompatible, an element is not reduced, or a pivot is not invertible, where FLINT reports
    /// that there is no solution.
    #[inline]
    fn mod_solve(self, b: NaturalVector, m: Natural) -> Option<NaturalVector> {
        mod_solve_vector(Cow::Borrowed(self), Cow::Owned(b), &m)
    }
}

impl ModSolve<NaturalVector, &Natural> for &NaturalMatrix {
    type Output = NaturalVector;

    /// Solves $Ax = b$ modulo a prime $m$ for $x$, where $A$ is a [`NaturalMatrix`] and $b$ is a
    /// [`NaturalVector`], taking the matrix by reference, the vector by value, and the modulus by
    /// reference.
    ///
    /// The elements of the matrix and the vector must already be reduced modulo $m$. If $A$ is $h
    /// \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with $Ax = b$
    /// modulo $m$, and `None` is returned if there is none. When there are several, the one
    /// returned has zeros in the positions corresponding to the columns of $A$ that are not pivot
    /// columns of its row echelon form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
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
    /// Panics if `m` is zero, if the height of the matrix is not the length of the vector, if any
    /// element of either is greater than or equal to `m`, or if a pivot is not invertible modulo
    /// `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve(b, &Natural::from(7u32)).unwrap().to_string(),
    ///     "(6, 1)"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!((&a).mod_solve(b, &Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// with a right-hand side of one column, except that it panics if the dimensions are
    /// incompatible, an element is not reduced, or a pivot is not invertible, where FLINT reports
    /// that there is no solution.
    #[inline]
    fn mod_solve(self, b: NaturalVector, m: &Natural) -> Option<NaturalVector> {
        mod_solve_vector(Cow::Borrowed(self), Cow::Owned(b), m)
    }
}

impl ModSolve<&NaturalVector, Natural> for &NaturalMatrix {
    type Output = NaturalVector;

    /// Solves $Ax = b$ modulo a prime $m$ for $x$, where $A$ is a [`NaturalMatrix`] and $b$ is a
    /// [`NaturalVector`], taking the matrix by reference, the vector by reference, and the modulus
    /// by value.
    ///
    /// The elements of the matrix and the vector must already be reduced modulo $m$. If $A$ is $h
    /// \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with $Ax = b$
    /// modulo $m$, and `None` is returned if there is none. When there are several, the one
    /// returned has zeros in the positions corresponding to the columns of $A$ that are not pivot
    /// columns of its row echelon form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
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
    /// Panics if `m` is zero, if the height of the matrix is not the length of the vector, if any
    /// element of either is greater than or equal to `m`, or if a pivot is not invertible modulo
    /// `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve(&b, Natural::from(7u32)).unwrap().to_string(),
    ///     "(6, 1)"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!((&a).mod_solve(&b, Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// with a right-hand side of one column, except that it panics if the dimensions are
    /// incompatible, an element is not reduced, or a pivot is not invertible, where FLINT reports
    /// that there is no solution.
    #[inline]
    fn mod_solve(self, b: &NaturalVector, m: Natural) -> Option<NaturalVector> {
        mod_solve_vector(Cow::Borrowed(self), Cow::Borrowed(b), &m)
    }
}

impl ModSolve<&NaturalVector, &Natural> for &NaturalMatrix {
    type Output = NaturalVector;

    /// Solves $Ax = b$ modulo a prime $m$ for $x$, where $A$ is a [`NaturalMatrix`] and $b$ is a
    /// [`NaturalVector`], taking the matrix by reference, the vector by reference, and the modulus
    /// by reference.
    ///
    /// The elements of the matrix and the vector must already be reduced modulo $m$. If $A$ is $h
    /// \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with $Ax = b$
    /// modulo $m$, and `None` is returned if there is none. When there are several, the one
    /// returned has zeros in the positions corresponding to the columns of $A$ that are not pivot
    /// columns of its row echelon form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the solution is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
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
    /// Panics if `m` is zero, if the height of the matrix is not the length of the vector, if any
    /// element of either is greater than or equal to `m`, or if a pivot is not invertible modulo
    /// `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve(&b, &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(6, 1)"
    /// );
    ///
    /// // A system with no solution
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!((&a).mod_solve(&b, &Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_can_solve` from `fmpz_mod_mat/can_solve.c`, FLINT 3.6.0,
    /// with a right-hand side of one column, except that it panics if the dimensions are
    /// incompatible, an element is not reduced, or a pivot is not invertible, where FLINT reports
    /// that there is no solution.
    #[inline]
    fn mod_solve(self, b: &NaturalVector, m: &Natural) -> Option<NaturalVector> {
        mod_solve_vector(Cow::Borrowed(self), Cow::Borrowed(b), m)
    }
}
