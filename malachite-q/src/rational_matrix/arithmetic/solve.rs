// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2019, 2020, 2022 William Hart
//
//      Copyright © 2019 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use crate::rational_matrix::conversion::to_numerators_and_row_denominators::*;
use crate::rational_vector::RationalVector;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Solve;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::fraction_matrix::{
    Fraction, can_solve_dixon as integer_can_solve_dixon, can_solve_fflu,
    can_solve_multi_mod as integer_can_solve_multi_mod, solve_dixon as integer_solve_dixon,
    solve_multi_mod as integer_solve_multi_mod, solve_nonsingular as integer_solve_nonsingular,
};

// The largest dimension of a square system solved by Cramer's rule.
const CRAMER_MAX_DIMENSION: usize = 3;

// The largest dimension of a square system, or height of a general one, solved by fraction-free LU
// decomposition.
const FRACTION_FREE_MAX_DIMENSION: usize = 15;

// The largest width of the right-hand side of a larger square system solved by Dixon's p-adic
// lifting; wider ones are solved with the multimodular algorithm.
const DIXON_MAX_WIDTH: usize = 32;

fn from_fractions(xs: Vec<Fraction>, height: usize, width: usize) -> RationalMatrix {
    RationalMatrix {
        elements: xs
            .into_iter()
            .map(|f| Rational {
                sign: f.sign,
                numerator: f.numerator,
                denominator: f.denominator,
            })
            .collect(),
        height,
        width,
    }
}

// The rational matrix X/d.
//
// This is equivalent to `fmpq_mat_set_fmpz_mat_div_fmpz` from `fmpq_mat/set_fmpz_mat_div_fmpz.c`,
// FLINT 3.6.0.
fn divide((x, den): (IntegerMatrix, Integer)) -> RationalMatrix {
    let (height, width) = (x.height(), x.width());
    let elements = x
        .into_elements()
        .into_iter()
        .map(|n| Rational::from_integers_ref(&n, &den))
        .collect();
    RationalMatrix::from_owned_elements(height, width, elements)
}

// Solves AX = B for a square matrix A whose rows have been cleared of denominators together with
// those of B, giving the integer matrices `a` and `b`; returns `None` if A is singular.
//
// This is equivalent to `fmpq_mat_solve_fmpz_mat` from `fmpq_mat/solve.c`, FLINT 3.6.0.
pub(crate) fn solve_cleared(a: &IntegerMatrix, b: &IntegerMatrix) -> Option<RationalMatrix> {
    let n = usize::exact_from(a.height());
    let k = usize::exact_from(b.width());
    if n <= FRACTION_FREE_MAX_DIMENSION {
        integer_solve_nonsingular(a, b).map(divide)
    } else if k <= DIXON_MAX_WIDTH {
        integer_solve_dixon(a, b).map(|x| from_fractions(x, n, k))
    } else {
        integer_solve_multi_mod(a, b).map(|x| from_fractions(x, n, k))
    }
}

// Solves AX = B for a matrix A of any shape whose rows have been cleared of denominators together
// with those of B, giving the integer matrices `a` and `b`; returns `None` if there is no solution.
fn can_solve_cleared(a: &IntegerMatrix, b: &IntegerMatrix) -> Option<RationalMatrix> {
    let h = usize::exact_from(a.height());
    if h <= FRACTION_FREE_MAX_DIMENSION {
        can_solve_fflu(a, b).map(divide)
    } else {
        let (w, k) = (usize::exact_from(a.width()), usize::exact_from(b.width()));
        integer_can_solve_multi_mod(a, b).map(|x| from_fractions(x, w, k))
    }
}

fn clear(a: &RationalMatrix, b: &RationalMatrix) -> (IntegerMatrix, IntegerMatrix) {
    assert_eq!(
        a.height, b.height,
        "the height of the matrix must equal the height of the right-hand side"
    );
    let (a, b, _) = numerators_over_common_row_denominators(a, &b.elements, b.width);
    (a, b)
}

// Solves AX = B for a square matrix A by fraction-free LU decomposition (or Cramer's rule, for
// dimensions up to 3), returning `None` if A is singular.
//
// This is equivalent to `fmpq_mat_solve_fraction_free` from `fmpq_mat/solve_fraction_free.c`, FLINT
// 3.6.0, except that it panics if the dimensions are incompatible.
crate_test_fn! {
solve_fraction_free(a: &RationalMatrix, b: &RationalMatrix) -> Option<RationalMatrix> {
    let (a, b) = clear(a, b);
    integer_solve_nonsingular(&a, &b).map(divide)
}}

// Solves AX = B for a square matrix A by Dixon's p-adic lifting, returning `None` if A is singular.
//
// This is equivalent to `fmpq_mat_solve_dixon` from `fmpq_mat/solve_dixon.c`, FLINT 3.6.0, except
// that it panics if the dimensions are incompatible.
crate_test_fn! {
solve_dixon(a: &RationalMatrix, b: &RationalMatrix) -> Option<RationalMatrix> {
    let (n, k) = (a.width, b.width);
    let (a, b) = clear(a, b);
    integer_solve_dixon(&a, &b).map(|x| from_fractions(x, n, k))
}}

// Solves AX = B for a square matrix A with the multimodular algorithm, returning `None` if A is
// singular.
//
// This is equivalent to `fmpq_mat_solve_multi_mod` from `fmpq_mat/solve_multi_mod.c`, FLINT 3.6.0,
// except that it panics if the dimensions are incompatible.
crate_test_fn! {
solve_multi_mod(a: &RationalMatrix, b: &RationalMatrix) -> Option<RationalMatrix> {
    let (n, k) = (a.width, b.width);
    let (a, b) = clear(a, b);
    integer_solve_multi_mod(&a, &b).map(|x| from_fractions(x, n, k))
}}

// Solves AX = B for a square matrix A, returning `None` if A is singular.
//
// This is equivalent to `fmpq_mat_solve` from `fmpq_mat/solve.c`, FLINT 3.6.0, except that it
// panics if the dimensions are incompatible.
crate_test_fn! {
solve_nonsingular(a: &RationalMatrix, b: &RationalMatrix) -> Option<RationalMatrix> {
    let (a, b) = clear(a, b);
    solve_cleared(&a, &b)
}}

// Solves AX = B for a matrix A of any shape by fraction-free LU decomposition, returning `None` if
// there is no solution.
//
// This is equivalent to `fmpq_mat_can_solve_fraction_free` from
// `fmpq_mat/can_solve_fraction_free.c`, FLINT 3.6.0, except that it panics if the dimensions are
// incompatible.
crate_test_fn! {
can_solve_fraction_free(a: &RationalMatrix, b: &RationalMatrix) -> Option<RationalMatrix> {
    let (a, b) = clear(a, b);
    can_solve_fflu(&a, &b).map(divide)
}}

// Solves AX = B for a matrix A of any shape with the multimodular algorithm, returning `None` if
// there is no solution.
//
// This is equivalent to `fmpq_mat_can_solve_multi_mod` from `fmpq_mat/can_solve_multi_mod.c`, FLINT
// 3.6.0, except that it panics if the dimensions are incompatible.
crate_test_fn! {
can_solve_multi_mod(a: &RationalMatrix, b: &RationalMatrix) -> Option<RationalMatrix> {
    let (w, k) = (a.width, b.width);
    let (a, b) = clear(a, b);
    integer_can_solve_multi_mod(&a, &b).map(|x| from_fractions(x, w, k))
}}

// Solves AX = B for a matrix A of any shape by Dixon's p-adic lifting, returning `None` if there is
// no solution.
//
// This is equivalent to `fmpq_mat_can_solve_dixon` from `fmpq_mat/can_solve_dixon.c`, FLINT 3.6.0,
// except that it panics if the dimensions are incompatible.
crate_test_fn! {
can_solve_dixon(a: &RationalMatrix, b: &RationalMatrix) -> Option<RationalMatrix> {
    let (w, k) = (a.width, b.width);
    let (a, b) = clear(a, b);
    integer_can_solve_dixon(&a, &b).map(|x| from_fractions(x, w, k))
}}

// Solves AX = B for a matrix A of any shape, returning `None` if there is no solution.
//
// This is equivalent to `fmpq_mat_can_solve` from `fmpq_mat/can_solve.c`, FLINT 3.6.0, except that
// it panics if the dimensions are incompatible.
crate_test_fn! {
can_solve(a: &RationalMatrix, b: &RationalMatrix) -> Option<RationalMatrix> {
    let (a, b) = clear(a, b);
    can_solve_cleared(&a, &b)
}}

// Solves AX = B for A and the matrix with elements `b` and width `width`, as `solve_nonsingular`
// does when A is square and nonsingular and as `can_solve` does otherwise. Square systems of
// dimension 4 to 15 go straight to the fraction-free `can_solve`, which finds the same solution
// when A is nonsingular, since the solution is then unique.
fn solve_helper(
    a: &RationalMatrix,
    b: &[Rational],
    height: usize,
    width: usize,
) -> Option<RationalMatrix> {
    assert_eq!(
        a.height, height,
        "the height of the matrix must equal the height of the right-hand side"
    );
    let (a_cleared, b_cleared, _) = numerators_over_common_row_denominators(a, b, width);
    let n = a.height;
    if n == a.width
        && (n <= CRAMER_MAX_DIMENSION || n > FRACTION_FREE_MAX_DIMENSION)
        && let Some(x) = solve_cleared(&a_cleared, &b_cleared)
    {
        return Some(x);
    }
    can_solve_cleared(&a_cleared, &b_cleared)
}

fn solve_vector_helper(a: &RationalMatrix, b: &RationalVector) -> Option<RationalVector> {
    solve_helper(a, &b.elements, b.elements.len(), 1).map(|x| RationalVector {
        elements: x.elements,
    })
}

impl Solve<Self> for RationalMatrix {
    type Output = Self;

    /// Solves $AX = B$ for $X$, where $A$ and $B$ are [`RationalMatrix`]es, taking the matrix by
    /// value and the right-hand side by value.
    ///
    /// If $A$ is $h \times w$ and $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with
    /// $AX = B$, and `None` is returned if there is none. When $A$ is square and nonsingular, the
    /// solution is unique; when there are several, one of them is returned.
    ///
    /// The denominators of each row of $A$ and of the right-hand side are cleared together, giving
    /// an integer system with the same solutions, which is solved as for [`IntegerMatrix`]: by
    /// Cramer's rule, fraction-free LU decomposition, Dixon's $p$-adic lifting, or a multimodular
    /// algorithm, depending on the dimensions.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// $A$ and the width of $B$, and $b$ is the maximum number of significant bits of the
    /// numerators and denominators of the elements of $A$ and $B$: clearing the denominators of the
    /// rows gives integer elements of $O(nb)$ bits, and the integer system is solved within the
    /// bounds of [`solve`](malachite_base::num::arithmetic::traits::Solve::solve) for
    /// [`IntegerMatrix`] at that size.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not the height of the right-hand side.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Solve;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; 1/4, 1/5]").unwrap();
    /// let b = RationalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(a.solve(b).unwrap().to_string(), "[-8; 15]");
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = RationalMatrix::from_str("[5; 6]").unwrap();
    /// assert_eq!(a.solve(b).unwrap().to_string(), "[-4; 9/2]");
    ///
    /// // A system with infinitely many solutions
    /// let a = RationalMatrix::from_str("[1, 2, 3; 2, 4, 6]").unwrap();
    /// let b = RationalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(a.solve(b).unwrap().to_string(), "[1; 0; 0]");
    ///
    /// // A system with no solution
    /// let a = RationalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// let b = RationalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(a.solve(b), None);
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_solve` from `fmpq_mat/solve.c`, FLINT 3.6.0, when $A$ is
    /// square and nonsingular, and to `fmpq_mat_can_solve` from `fmpq_mat/can_solve.c` otherwise,
    /// except that it panics if the dimensions are incompatible.
    #[inline]
    fn solve(self, b: Self) -> Option<Self> {
        solve_helper(&self, &b.elements, b.height, b.width)
    }
}

impl Solve<&Self> for RationalMatrix {
    type Output = Self;

    /// Solves $AX = B$ for $X$, where $A$ and $B$ are [`RationalMatrix`]es, taking the matrix by
    /// value and the right-hand side by reference.
    ///
    /// If $A$ is $h \times w$ and $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with
    /// $AX = B$, and `None` is returned if there is none. When $A$ is square and nonsingular, the
    /// solution is unique; when there are several, one of them is returned.
    ///
    /// The denominators of each row of $A$ and of the right-hand side are cleared together, giving
    /// an integer system with the same solutions, which is solved as for [`IntegerMatrix`]: by
    /// Cramer's rule, fraction-free LU decomposition, Dixon's $p$-adic lifting, or a multimodular
    /// algorithm, depending on the dimensions.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// $A$ and the width of $B$, and $b$ is the maximum number of significant bits of the
    /// numerators and denominators of the elements of $A$ and $B$: clearing the denominators of the
    /// rows gives integer elements of $O(nb)$ bits, and the integer system is solved within the
    /// bounds of [`solve`](malachite_base::num::arithmetic::traits::Solve::solve) for
    /// [`IntegerMatrix`] at that size.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not the height of the right-hand side.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Solve;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; 1/4, 1/5]").unwrap();
    /// let b = RationalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(a.solve(&b).unwrap().to_string(), "[-8; 15]");
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = RationalMatrix::from_str("[5; 6]").unwrap();
    /// assert_eq!(a.solve(&b).unwrap().to_string(), "[-4; 9/2]");
    ///
    /// // A system with infinitely many solutions
    /// let a = RationalMatrix::from_str("[1, 2, 3; 2, 4, 6]").unwrap();
    /// let b = RationalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!(a.solve(&b).unwrap().to_string(), "[1; 0; 0]");
    ///
    /// // A system with no solution
    /// let a = RationalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// let b = RationalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!(a.solve(&b), None);
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_solve` from `fmpq_mat/solve.c`, FLINT 3.6.0, when $A$ is
    /// square and nonsingular, and to `fmpq_mat_can_solve` from `fmpq_mat/can_solve.c` otherwise,
    /// except that it panics if the dimensions are incompatible.
    #[inline]
    fn solve(self, b: &Self) -> Option<Self> {
        solve_helper(&self, &b.elements, b.height, b.width)
    }
}

impl Solve<RationalMatrix> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Solves $AX = B$ for $X$, where $A$ and $B$ are [`RationalMatrix`]es, taking the matrix by
    /// reference and the right-hand side by value.
    ///
    /// If $A$ is $h \times w$ and $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with
    /// $AX = B$, and `None` is returned if there is none. When $A$ is square and nonsingular, the
    /// solution is unique; when there are several, one of them is returned.
    ///
    /// The denominators of each row of $A$ and of the right-hand side are cleared together, giving
    /// an integer system with the same solutions, which is solved as for [`IntegerMatrix`]: by
    /// Cramer's rule, fraction-free LU decomposition, Dixon's $p$-adic lifting, or a multimodular
    /// algorithm, depending on the dimensions.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// $A$ and the width of $B$, and $b$ is the maximum number of significant bits of the
    /// numerators and denominators of the elements of $A$ and $B$: clearing the denominators of the
    /// rows gives integer elements of $O(nb)$ bits, and the integer system is solved within the
    /// bounds of [`solve`](malachite_base::num::arithmetic::traits::Solve::solve) for
    /// [`IntegerMatrix`] at that size.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not the height of the right-hand side.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Solve;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; 1/4, 1/5]").unwrap();
    /// let b = RationalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!((&a).solve(b).unwrap().to_string(), "[-8; 15]");
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = RationalMatrix::from_str("[5; 6]").unwrap();
    /// assert_eq!((&a).solve(b).unwrap().to_string(), "[-4; 9/2]");
    ///
    /// // A system with infinitely many solutions
    /// let a = RationalMatrix::from_str("[1, 2, 3; 2, 4, 6]").unwrap();
    /// let b = RationalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!((&a).solve(b).unwrap().to_string(), "[1; 0; 0]");
    ///
    /// // A system with no solution
    /// let a = RationalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// let b = RationalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!((&a).solve(b), None);
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_solve` from `fmpq_mat/solve.c`, FLINT 3.6.0, when $A$ is
    /// square and nonsingular, and to `fmpq_mat_can_solve` from `fmpq_mat/can_solve.c` otherwise,
    /// except that it panics if the dimensions are incompatible.
    #[inline]
    fn solve(self, b: RationalMatrix) -> Option<RationalMatrix> {
        solve_helper(self, &b.elements, b.height, b.width)
    }
}

impl Solve<&RationalMatrix> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Solves $AX = B$ for $X$, where $A$ and $B$ are [`RationalMatrix`]es, taking the matrix by
    /// reference and the right-hand side by reference.
    ///
    /// If $A$ is $h \times w$ and $B$ is $h \times k$, a solution is a $w \times k$ matrix $X$ with
    /// $AX = B$, and `None` is returned if there is none. When $A$ is square and nonsingular, the
    /// solution is unique; when there are several, one of them is returned.
    ///
    /// The denominators of each row of $A$ and of the right-hand side are cleared together, giving
    /// an integer system with the same solutions, which is solved as for [`IntegerMatrix`]: by
    /// Cramer's rule, fraction-free LU decomposition, Dixon's $p$-adic lifting, or a multimodular
    /// algorithm, depending on the dimensions.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// $A$ and the width of $B$, and $b$ is the maximum number of significant bits of the
    /// numerators and denominators of the elements of $A$ and $B$: clearing the denominators of the
    /// rows gives integer elements of $O(nb)$ bits, and the integer system is solved within the
    /// bounds of [`solve`](malachite_base::num::arithmetic::traits::Solve::solve) for
    /// [`IntegerMatrix`] at that size.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not the height of the right-hand side.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Solve;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; 1/4, 1/5]").unwrap();
    /// let b = RationalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!((&a).solve(&b).unwrap().to_string(), "[-8; 15]");
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = RationalMatrix::from_str("[5; 6]").unwrap();
    /// assert_eq!((&a).solve(&b).unwrap().to_string(), "[-4; 9/2]");
    ///
    /// // A system with infinitely many solutions
    /// let a = RationalMatrix::from_str("[1, 2, 3; 2, 4, 6]").unwrap();
    /// let b = RationalMatrix::from_str("[1; 2]").unwrap();
    /// assert_eq!((&a).solve(&b).unwrap().to_string(), "[1; 0; 0]");
    ///
    /// // A system with no solution
    /// let a = RationalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// let b = RationalMatrix::from_str("[1; 1]").unwrap();
    /// assert_eq!((&a).solve(&b), None);
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_solve` from `fmpq_mat/solve.c`, FLINT 3.6.0, when $A$ is
    /// square and nonsingular, and to `fmpq_mat_can_solve` from `fmpq_mat/can_solve.c` otherwise,
    /// except that it panics if the dimensions are incompatible.
    #[inline]
    fn solve(self, b: &RationalMatrix) -> Option<RationalMatrix> {
        solve_helper(self, &b.elements, b.height, b.width)
    }
}

impl Solve<RationalVector> for RationalMatrix {
    type Output = RationalVector;

    /// Solves $Ax = b$ for $x$, where $A$ is a [`RationalMatrix`] and $b$ is a [`RationalVector`],
    /// taking the matrix by value and the vector by value.
    ///
    /// If $A$ is $h \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with
    /// $Ax = b$, and `None` is returned if there is none. When $A$ is square and nonsingular, the
    /// solution is unique; when there are several, one of them is returned.
    ///
    /// The denominators of each row of $A$ and of the right-hand side are cleared together, giving
    /// an integer system with the same solutions, which is solved as for [`IntegerMatrix`]: by
    /// Cramer's rule, fraction-free LU decomposition, Dixon's $p$-adic lifting, or a multimodular
    /// algorithm, depending on the dimensions.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// $A$, and $b$ is the maximum number of significant bits of the numerators and denominators of
    /// the elements of $A$ and $b$: clearing the denominators of the rows gives integer elements of
    /// $O(nb)$ bits, and the integer system is solved within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve) for [`IntegerMatrix`] at
    /// that size.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not the length of the vector.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Solve;
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = RationalVector::from_str("(5, 6)").unwrap();
    /// assert_eq!(a.solve(b).unwrap().to_string(), "(-4, 9/2)");
    ///
    /// // A system with infinitely many solutions
    /// let a = RationalMatrix::from_str("[1, 2, 3; 2, 4, 6]").unwrap();
    /// let b = RationalVector::from_str("(1, 2)").unwrap();
    /// assert_eq!(a.solve(b).unwrap().to_string(), "(1, 0, 0)");
    ///
    /// // A system with no solution
    /// let a = RationalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// let b = RationalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(a.solve(b), None);
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_solve` from `fmpq_mat/solve.c`, FLINT 3.6.0, when $A$ is
    /// square and nonsingular, and to `fmpq_mat_can_solve` from `fmpq_mat/can_solve.c` otherwise,
    /// with a right-hand side of one column, except that it panics if the dimensions are
    /// incompatible.
    #[inline]
    fn solve(self, b: RationalVector) -> Option<RationalVector> {
        solve_vector_helper(&self, &b)
    }
}

impl Solve<&RationalVector> for RationalMatrix {
    type Output = RationalVector;

    /// Solves $Ax = b$ for $x$, where $A$ is a [`RationalMatrix`] and $b$ is a [`RationalVector`],
    /// taking the matrix by value and the vector by reference.
    ///
    /// If $A$ is $h \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with
    /// $Ax = b$, and `None` is returned if there is none. When $A$ is square and nonsingular, the
    /// solution is unique; when there are several, one of them is returned.
    ///
    /// The denominators of each row of $A$ and of the right-hand side are cleared together, giving
    /// an integer system with the same solutions, which is solved as for [`IntegerMatrix`]: by
    /// Cramer's rule, fraction-free LU decomposition, Dixon's $p$-adic lifting, or a multimodular
    /// algorithm, depending on the dimensions.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// $A$, and $b$ is the maximum number of significant bits of the numerators and denominators of
    /// the elements of $A$ and $b$: clearing the denominators of the rows gives integer elements of
    /// $O(nb)$ bits, and the integer system is solved within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve) for [`IntegerMatrix`] at
    /// that size.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not the length of the vector.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Solve;
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = RationalVector::from_str("(5, 6)").unwrap();
    /// assert_eq!(a.solve(&b).unwrap().to_string(), "(-4, 9/2)");
    ///
    /// // A system with infinitely many solutions
    /// let a = RationalMatrix::from_str("[1, 2, 3; 2, 4, 6]").unwrap();
    /// let b = RationalVector::from_str("(1, 2)").unwrap();
    /// assert_eq!(a.solve(&b).unwrap().to_string(), "(1, 0, 0)");
    ///
    /// // A system with no solution
    /// let a = RationalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// let b = RationalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(a.solve(&b), None);
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_solve` from `fmpq_mat/solve.c`, FLINT 3.6.0, when $A$ is
    /// square and nonsingular, and to `fmpq_mat_can_solve` from `fmpq_mat/can_solve.c` otherwise,
    /// with a right-hand side of one column, except that it panics if the dimensions are
    /// incompatible.
    #[inline]
    fn solve(self, b: &RationalVector) -> Option<RationalVector> {
        solve_vector_helper(&self, b)
    }
}

impl Solve<RationalVector> for &RationalMatrix {
    type Output = RationalVector;

    /// Solves $Ax = b$ for $x$, where $A$ is a [`RationalMatrix`] and $b$ is a [`RationalVector`],
    /// taking the matrix by reference and the vector by value.
    ///
    /// If $A$ is $h \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with
    /// $Ax = b$, and `None` is returned if there is none. When $A$ is square and nonsingular, the
    /// solution is unique; when there are several, one of them is returned.
    ///
    /// The denominators of each row of $A$ and of the right-hand side are cleared together, giving
    /// an integer system with the same solutions, which is solved as for [`IntegerMatrix`]: by
    /// Cramer's rule, fraction-free LU decomposition, Dixon's $p$-adic lifting, or a multimodular
    /// algorithm, depending on the dimensions.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// $A$, and $b$ is the maximum number of significant bits of the numerators and denominators of
    /// the elements of $A$ and $b$: clearing the denominators of the rows gives integer elements of
    /// $O(nb)$ bits, and the integer system is solved within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve) for [`IntegerMatrix`] at
    /// that size.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not the length of the vector.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Solve;
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = RationalVector::from_str("(5, 6)").unwrap();
    /// assert_eq!((&a).solve(b).unwrap().to_string(), "(-4, 9/2)");
    ///
    /// // A system with infinitely many solutions
    /// let a = RationalMatrix::from_str("[1, 2, 3; 2, 4, 6]").unwrap();
    /// let b = RationalVector::from_str("(1, 2)").unwrap();
    /// assert_eq!((&a).solve(b).unwrap().to_string(), "(1, 0, 0)");
    ///
    /// // A system with no solution
    /// let a = RationalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// let b = RationalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!((&a).solve(b), None);
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_solve` from `fmpq_mat/solve.c`, FLINT 3.6.0, when $A$ is
    /// square and nonsingular, and to `fmpq_mat_can_solve` from `fmpq_mat/can_solve.c` otherwise,
    /// with a right-hand side of one column, except that it panics if the dimensions are
    /// incompatible.
    #[inline]
    fn solve(self, b: RationalVector) -> Option<RationalVector> {
        solve_vector_helper(self, &b)
    }
}

impl Solve<&RationalVector> for &RationalMatrix {
    type Output = RationalVector;

    /// Solves $Ax = b$ for $x$, where $A$ is a [`RationalMatrix`] and $b$ is a [`RationalVector`],
    /// taking the matrix by reference and the vector by reference.
    ///
    /// If $A$ is $h \times w$ and $b$ has length $h$, a solution is a vector $x$ of length $w$ with
    /// $Ax = b$, and `None` is returned if there is none. When $A$ is square and nonsingular, the
    /// solution is unique; when there are several, one of them is returned.
    ///
    /// The denominators of each row of $A$ and of the right-hand side are cleared together, giving
    /// an integer system with the same solutions, which is solved as for [`IntegerMatrix`]: by
    /// Cramer's rule, fraction-free LU decomposition, Dixon's $p$-adic lifting, or a multimodular
    /// algorithm, depending on the dimensions.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// $A$, and $b$ is the maximum number of significant bits of the numerators and denominators of
    /// the elements of $A$ and $b$: clearing the denominators of the rows gives integer elements of
    /// $O(nb)$ bits, and the integer system is solved within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve) for [`IntegerMatrix`] at
    /// that size.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not the length of the vector.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Solve;
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = RationalVector::from_str("(5, 6)").unwrap();
    /// assert_eq!((&a).solve(&b).unwrap().to_string(), "(-4, 9/2)");
    ///
    /// // A system with infinitely many solutions
    /// let a = RationalMatrix::from_str("[1, 2, 3; 2, 4, 6]").unwrap();
    /// let b = RationalVector::from_str("(1, 2)").unwrap();
    /// assert_eq!((&a).solve(&b).unwrap().to_string(), "(1, 0, 0)");
    ///
    /// // A system with no solution
    /// let a = RationalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// let b = RationalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!((&a).solve(&b), None);
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_solve` from `fmpq_mat/solve.c`, FLINT 3.6.0, when $A$ is
    /// square and nonsingular, and to `fmpq_mat_can_solve` from `fmpq_mat/can_solve.c` otherwise,
    /// with a right-hand side of one column, except that it panics if the dimensions are
    /// incompatible.
    #[inline]
    fn solve(self, b: &RationalVector) -> Option<RationalVector> {
        solve_vector_helper(self, b)
    }
}
