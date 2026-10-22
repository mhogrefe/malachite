// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Fredrik Johansson
//
//      Copyright © 2019 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModSolve};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_lu_decomposition::lu_in_place;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::strassen::{BlockRef, block};
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::{ProductShape, matrix_mod_mul};
use crate::unsigned_matrix::arithmetic::mod_solve_lower_triangular::solve_lower;
use crate::unsigned_matrix::arithmetic::mod_solve_upper_triangular::solve_upper;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec;
use alloc::vec::Vec;

// The rows of the `height` by `width` matrix with elements `xs`, permuted: row `i` of the result is
// row `perm[i]` of the matrix.
fn permute_rows<T: PrimitiveUnsigned>(xs: &[T], width: usize, perm: &[usize]) -> Vec<T> {
    let mut out = Vec::with_capacity(perm.len() * width);
    for &p in perm {
        out.extend_from_slice(&xs[p * width..(p + 1) * width]);
    }
    out
}

// Solves PAX = PB for X, given the packed LU decomposition `lu` of the n by n matrix A, which has
// full rank, and the permutation `perm`. Returns the elements of X.
fn solve_from_square_lu<T: PrimitiveUnsigned>(
    lu: &[T],
    perm: &[usize],
    b: &[T],
    n: usize,
    k: usize,
    m: T,
) -> Vec<T> {
    let mut x = permute_rows(b, k, perm);
    let lr = BlockRef {
        xs: lu,
        width: n,
        row: 0,
        column: 0,
    };
    solve_lower(&mut x, lr, n, k, m, true);
    solve_upper(&mut x, lr, n, k, m, false);
    x
}

// Solves AX = B modulo the prime `m` for a square n by n matrix A, consuming it, and returns the
// elements of X, or `None` if A does not have full rank. B is n by k.
//
// This is equivalent to `nmod_mat_solve` from `nmod_mat/solve.c`, FLINT 3.6.0.
pub(crate) fn solve_nonsingular<T: PrimitiveUnsigned>(
    mut a: UnsignedMatrix<T>,
    b: &[T],
    k: usize,
    m: T,
) -> Option<Vec<T>> {
    let n = a.height;
    if n == 0 || k == 0 {
        return Some(vec![T::ZERO; n * k]);
    }
    let (perm, rank) = lu_in_place(&mut a, m, true);
    if rank == n {
        Some(solve_from_square_lu(&a.elements, &perm, b, n, k, m))
    } else {
        None
    }
}

// Solves AX = B modulo the prime `m` for an h by w matrix A, consuming it, and returns the elements
// of a solution X, or `None` if there is none. B is h by k and X is w by k. When A is square and
// has full rank, the solution is computed as by `nmod_mat_solve`; otherwise, the LU decomposition
// is reused to find a solution as `nmod_mat_can_solve_inner` does.
//
// This is equivalent to `nmod_mat_can_solve_inner` from `nmod_mat/can_solve.c`, FLINT 3.6.0,
// without returning the rank, the permutation, or the pivots.
fn solve<T: PrimitiveUnsigned>(
    mut a: UnsignedMatrix<T>,
    b: &[T],
    k: usize,
    m: T,
) -> Option<Vec<T>> {
    let (h, w) = (a.height, a.width);
    if h == 0 || k == 0 {
        return Some(vec![T::ZERO; w * k]);
    }
    if w == 0 {
        return if b.iter().all(|&x| x == T::ZERO) {
            Some(Vec::new())
        } else {
            None
        };
    }
    let (perm, rank) = lu_in_place(&mut a, m, false);
    let lu = &a.elements;
    if h == w && rank == h {
        return Some(solve_from_square_lu(lu, &perm, b, h, k, m));
    }
    let pb = permute_rows(b, k, &perm);
    // The pivot columns, and the rank by rank upper triangular matrix made of them.
    let mut pivots = Vec::with_capacity(rank);
    let mut lu2 = vec![T::ZERO; rank * rank];
    let mut col = 0;
    for i in 0..rank {
        while lu[i * w + col] == T::ZERO {
            col += 1;
        }
        pivots.push(col);
        for j in 0..rank {
            lu2[j * rank + i] = lu[j * w + col];
        }
        col += 1;
    }
    let lr = BlockRef {
        xs: lu,
        width: w,
        row: 0,
        column: 0,
    };
    let mut y = pb[..rank * k].to_vec();
    solve_lower(&mut y, lr, rank, k, m, true);
    if h > rank {
        // The remaining rows of L, times Y, must equal the remaining rows of PB.
        let l_rest = block(lu, w, rank..h, 0..rank);
        let product = matrix_mod_mul(
            &l_rest,
            &y,
            ProductShape {
                ar: h - rank,
                br: rank,
                bc: k,
            },
            m,
        );
        if product != pb[rank * k..] {
            return None;
        }
    }
    solve_upper(
        &mut y,
        BlockRef {
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
    let mut x = vec![T::ZERO; w * k];
    for (i, &p) in pivots.iter().enumerate() {
        x[p * k..(p + 1) * k].copy_from_slice(&y[i * k..(i + 1) * k]);
    }
    Some(x)
}

fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, b_height: usize, m: T) {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert_eq!(
        a.height, b_height,
        "the height of the matrix must equal the height of the right-hand side"
    );
    assert!(
        a.mod_is_reduced(&m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

fn mod_solve_matrix<T: PrimitiveUnsigned>(
    a: UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
) -> Option<UnsignedMatrix<T>> {
    assert_arguments(&a, b.height, m);
    assert!(
        b.mod_is_reduced(&m),
        "the right-hand side must be reduced mod m, but {b} has an element >= {m}"
    );
    let width = a.width;
    solve(a, &b.elements, b.width, m).map(|elements| UnsignedMatrix {
        elements,
        height: width,
        width: b.width,
    })
}

fn mod_solve_vector<T: PrimitiveUnsigned>(
    a: UnsignedMatrix<T>,
    b: &UnsignedVector<T>,
    m: T,
) -> Option<UnsignedVector<T>> {
    assert_arguments(&a, b.elements.len(), m);
    assert!(
        b.mod_is_reduced(&m),
        "the right-hand side must be reduced mod m, but {b} has an element >= {m}"
    );
    solve(a, &b.elements, 1, m).map(|elements| UnsignedVector { elements })
}

impl<T: PrimitiveUnsigned> ModSolve<Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Solves $AX = B$ modulo a prime $m$ for $X$, where $A$ and $B$ are [`UnsignedMatrix`]es,
    /// taking both by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. If $A$ is $h \times w$ and
    /// $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with $AX = B$ modulo $m$, and
    /// `None` is returned if there is none. When there are several, the one returned has zeros in
    /// the rows corresponding to the columns of $A$ that are not pivot columns of its row echelon
    /// form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(\min(h, w)(hw + (h + w)k))$
    ///
    /// $M(h, w, k) = O(hw + hk + wk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is the width of the right-hand side.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 1]").unwrap();
    /// assert_eq!(a.mod_solve(b, 7).unwrap().to_string(), "[6; 1]");
    ///
    /// // A singular system with many solutions
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(a.mod_solve(b, 7).unwrap().to_string(), "[1; 0]");
    ///
    /// // A system with no solution
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 1]").unwrap();
    /// assert_eq!(a.mod_solve(b, 7), None);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve(b, 11).unwrap().to_string(),
    ///     "[2, 8; 5, 7; 0, 0]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_can_solve` from `nmod_mat/can_solve.c`, FLINT 3.6.0, using
    /// the method of `nmod_mat_solve` from `nmod_mat/solve.c` when $A$ is square and has full rank,
    /// except that it panics if the dimensions are incompatible or an element is not reduced.
    #[inline]
    fn mod_solve(self, b: Self, m: T) -> Option<Self> {
        mod_solve_matrix(self, &b, m)
    }
}

impl<T: PrimitiveUnsigned> ModSolve<&Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Solves $AX = B$ modulo a prime $m$ for $X$, where $A$ and $B$ are [`UnsignedMatrix`]es,
    /// taking the matrix by value and the right-hand side by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. If $A$ is $h \times w$ and
    /// $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with $AX = B$ modulo $m$, and
    /// `None` is returned if there is none. When there are several, the one returned has zeros in
    /// the rows corresponding to the columns of $A$ that are not pivot columns of its row echelon
    /// form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(\min(h, w)(hw + (h + w)k))$
    ///
    /// $M(h, w, k) = O(hw + hk + wk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is the width of the right-hand side.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 1]").unwrap();
    /// assert_eq!(a.mod_solve(&b, 7).unwrap().to_string(), "[6; 1]");
    ///
    /// // A singular system with many solutions
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!(a.mod_solve(&b, 7).unwrap().to_string(), "[1; 0]");
    ///
    /// // A system with no solution
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 1]").unwrap();
    /// assert_eq!(a.mod_solve(&b, 7), None);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_solve(&b, 11).unwrap().to_string(),
    ///     "[2, 8; 5, 7; 0, 0]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_can_solve` from `nmod_mat/can_solve.c`, FLINT 3.6.0, using
    /// the method of `nmod_mat_solve` from `nmod_mat/solve.c` when $A$ is square and has full rank,
    /// except that it panics if the dimensions are incompatible or an element is not reduced.
    #[inline]
    fn mod_solve(self, b: &Self, m: T) -> Option<Self> {
        mod_solve_matrix(self, b, m)
    }
}

impl<T: PrimitiveUnsigned> ModSolve<UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Solves $AX = B$ modulo a prime $m$ for $X$, where $A$ and $B$ are [`UnsignedMatrix`]es,
    /// taking the matrix by reference and the right-hand side by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. If $A$ is $h \times w$ and
    /// $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with $AX = B$ modulo $m$, and
    /// `None` is returned if there is none. When there are several, the one returned has zeros in
    /// the rows corresponding to the columns of $A$ that are not pivot columns of its row echelon
    /// form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(\min(h, w)(hw + (h + w)k))$
    ///
    /// $M(h, w, k) = O(hw + hk + wk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is the width of the right-hand side.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 1]").unwrap();
    /// assert_eq!((&a).mod_solve(b, 7).unwrap().to_string(), "[6; 1]");
    ///
    /// // A singular system with many solutions
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!((&a).mod_solve(b, 7).unwrap().to_string(), "[1; 0]");
    ///
    /// // A system with no solution
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 1]").unwrap();
    /// assert_eq!((&a).mod_solve(b, 7), None);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve(b, 11).unwrap().to_string(),
    ///     "[2, 8; 5, 7; 0, 0]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_can_solve` from `nmod_mat/can_solve.c`, FLINT 3.6.0, using
    /// the method of `nmod_mat_solve` from `nmod_mat/solve.c` when $A$ is square and has full rank,
    /// except that it panics if the dimensions are incompatible or an element is not reduced.
    #[inline]
    fn mod_solve(self, b: UnsignedMatrix<T>, m: T) -> Option<UnsignedMatrix<T>> {
        mod_solve_matrix(self.clone(), &b, m)
    }
}

impl<T: PrimitiveUnsigned> ModSolve<&UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Solves $AX = B$ modulo a prime $m$ for $X$, where $A$ and $B$ are [`UnsignedMatrix`]es,
    /// taking both by reference.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. If $A$ is $h \times w$ and
    /// $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with $AX = B$ modulo $m$, and
    /// `None` is returned if there is none. When there are several, the one returned has zeros in
    /// the rows corresponding to the columns of $A$ that are not pivot columns of its row echelon
    /// form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(\min(h, w)(hw + (h + w)k))$
    ///
    /// $M(h, w, k) = O(hw + hk + wk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is the width of the right-hand side.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 1]").unwrap();
    /// assert_eq!((&a).mod_solve(&b, 7).unwrap().to_string(), "[6; 1]");
    ///
    /// // A singular system with many solutions
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    /// assert_eq!((&a).mod_solve(&b, 7).unwrap().to_string(), "[1; 0]");
    ///
    /// // A system with no solution
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 1]").unwrap();
    /// assert_eq!((&a).mod_solve(&b, 7), None);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1, 0; 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_solve(&b, 11).unwrap().to_string(),
    ///     "[2, 8; 5, 7; 0, 0]"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_mat_can_solve` from `nmod_mat/can_solve.c`, FLINT 3.6.0, using
    /// the method of `nmod_mat_solve` from `nmod_mat/solve.c` when $A$ is square and has full rank,
    /// except that it panics if the dimensions are incompatible or an element is not reduced.
    #[inline]
    fn mod_solve(self, b: &UnsignedMatrix<T>, m: T) -> Option<UnsignedMatrix<T>> {
        mod_solve_matrix(self.clone(), b, m)
    }
}

impl<T: PrimitiveUnsigned> ModSolve<UnsignedVector<T>, T> for UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;

    /// Solves $Ax = b$ modulo a prime $m$ for $x$, where $A$ is an [`UnsignedMatrix`] and $b$ is an
    /// [`UnsignedVector`], taking both by value.
    ///
    /// The elements of the matrix and the vector must already be reduced modulo $m$. If $A$ is $h
    /// \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with $Ax = b$
    /// modulo $m$, and `None` is returned if there is none. When there are several, the one
    /// returned has zeros in the positions corresponding to the columns of $A$ that are not pivot
    /// columns of its row echelon form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
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
    /// Panics if `m` is zero, if the height of the matrix is not the length of the vector, if any
    /// element of either is greater than or equal to `m`, or if a pivot is not invertible modulo
    /// `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    /// assert_eq!(a.mod_solve(b, 7).unwrap().to_string(), "(6, 1)");
    ///
    /// // A system with no solution
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    /// assert_eq!(a.mod_solve(b, 7), None);
    /// ```
    ///
    /// This is equivalent to `nmod_mat_can_solve` from `nmod_mat/can_solve.c`, FLINT 3.6.0, with a
    /// right-hand side of one column, and to `nmod_mat_solve_vec` from `nmod_mat/solve_vec.c` when
    /// $A$ is square and has full rank, except that it panics if the dimensions are incompatible or
    /// an element is not reduced.
    #[inline]
    fn mod_solve(self, b: UnsignedVector<T>, m: T) -> Option<UnsignedVector<T>> {
        mod_solve_vector(self, &b, m)
    }
}

impl<T: PrimitiveUnsigned> ModSolve<&UnsignedVector<T>, T> for UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;

    /// Solves $Ax = b$ modulo a prime $m$ for $x$, where $A$ is an [`UnsignedMatrix`] and $b$ is an
    /// [`UnsignedVector`], taking the matrix by value and the vector by reference.
    ///
    /// The elements of the matrix and the vector must already be reduced modulo $m$. If $A$ is $h
    /// \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with $Ax = b$
    /// modulo $m$, and `None` is returned if there is none. When there are several, the one
    /// returned has zeros in the positions corresponding to the columns of $A$ that are not pivot
    /// columns of its row echelon form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
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
    /// Panics if `m` is zero, if the height of the matrix is not the length of the vector, if any
    /// element of either is greater than or equal to `m`, or if a pivot is not invertible modulo
    /// `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    /// assert_eq!(a.mod_solve(&b, 7).unwrap().to_string(), "(6, 1)");
    ///
    /// // A system with no solution
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    /// assert_eq!(a.mod_solve(&b, 7), None);
    /// ```
    ///
    /// This is equivalent to `nmod_mat_can_solve` from `nmod_mat/can_solve.c`, FLINT 3.6.0, with a
    /// right-hand side of one column, and to `nmod_mat_solve_vec` from `nmod_mat/solve_vec.c` when
    /// $A$ is square and has full rank, except that it panics if the dimensions are incompatible or
    /// an element is not reduced.
    #[inline]
    fn mod_solve(self, b: &UnsignedVector<T>, m: T) -> Option<UnsignedVector<T>> {
        mod_solve_vector(self, b, m)
    }
}

impl<T: PrimitiveUnsigned> ModSolve<UnsignedVector<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;

    /// Solves $Ax = b$ modulo a prime $m$ for $x$, where $A$ is an [`UnsignedMatrix`] and $b$ is an
    /// [`UnsignedVector`], taking the matrix by reference and the vector by value.
    ///
    /// The elements of the matrix and the vector must already be reduced modulo $m$. If $A$ is $h
    /// \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with $Ax = b$
    /// modulo $m$, and `None` is returned if there is none. When there are several, the one
    /// returned has zeros in the positions corresponding to the columns of $A$ that are not pivot
    /// columns of its row echelon form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
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
    /// Panics if `m` is zero, if the height of the matrix is not the length of the vector, if any
    /// element of either is greater than or equal to `m`, or if a pivot is not invertible modulo
    /// `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    /// assert_eq!((&a).mod_solve(b, 7).unwrap().to_string(), "(6, 1)");
    ///
    /// // A system with no solution
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    /// assert_eq!((&a).mod_solve(b, 7), None);
    /// ```
    ///
    /// This is equivalent to `nmod_mat_can_solve` from `nmod_mat/can_solve.c`, FLINT 3.6.0, with a
    /// right-hand side of one column, and to `nmod_mat_solve_vec` from `nmod_mat/solve_vec.c` when
    /// $A$ is square and has full rank, except that it panics if the dimensions are incompatible or
    /// an element is not reduced.
    #[inline]
    fn mod_solve(self, b: UnsignedVector<T>, m: T) -> Option<UnsignedVector<T>> {
        mod_solve_vector(self.clone(), &b, m)
    }
}

impl<T: PrimitiveUnsigned> ModSolve<&UnsignedVector<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;

    /// Solves $Ax = b$ modulo a prime $m$ for $x$, where $A$ is an [`UnsignedMatrix`] and $b$ is an
    /// [`UnsignedVector`], taking both by reference.
    ///
    /// The elements of the matrix and the vector must already be reduced modulo $m$. If $A$ is $h
    /// \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with $Ax = b$
    /// modulo $m$, and `None` is returned if there is none. When there are several, the one
    /// returned has zeros in the positions corresponding to the columns of $A$ that are not pivot
    /// columns of its row echelon form; when $A$ has rank $w$, the solution is unique.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
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
    /// Panics if `m` is zero, if the height of the matrix is not the length of the vector, if any
    /// element of either is greater than or equal to `m`, or if a pivot is not invertible modulo
    /// `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSolve;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    /// assert_eq!((&a).mod_solve(&b, 7).unwrap().to_string(), "(6, 1)");
    ///
    /// // A system with no solution
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    /// assert_eq!((&a).mod_solve(&b, 7), None);
    /// ```
    ///
    /// This is equivalent to `nmod_mat_can_solve` from `nmod_mat/can_solve.c`, FLINT 3.6.0, with a
    /// right-hand side of one column, and to `nmod_mat_solve_vec` from `nmod_mat/solve_vec.c` when
    /// $A$ is square and has full rank, except that it panics if the dimensions are incompatible or
    /// an element is not reduced.
    #[inline]
    fn mod_solve(self, b: &UnsignedVector<T>, m: T) -> Option<UnsignedVector<T>> {
        mod_solve_vector(self.clone(), b, m)
    }
}
