// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2025 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use malachite_base::num::arithmetic::traits::{MulAddMul, Permanent};
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// Returns the permanent of the square matrix `a`: directly for dimensions up to 2, and otherwise as
// the permanent of the integer matrix found by clearing the denominators of each row, divided by
// the product of those denominators.
//
// This is equivalent to `fmpq_mat_permanent` from `fmpq_mat/permanent.c`, FLINT 3.6.0, except that
// it always succeeds and panics if the matrix is not square.
fn permanent_helper(a: &RationalMatrix) -> Rational {
    assert_eq!(a.height, a.width, "the matrix must be square");
    match a.elements.as_slice() {
        [] => Rational::ONE,
        [a00] => a00.clone(),
        [a00, a01, a10, a11] => a00.mul_add_mul(a11, a01, a10),
        _ => {
            let (numerators, denominators) = a.to_numerators_and_row_denominators();
            let denominator: Natural = denominators.elements.into_iter().product();
            Rational::from_integers(numerators.permanent(), Integer::from(denominator))
        }
    }
}

impl Permanent for RationalMatrix {
    type Output = Rational;

    /// Computes the permanent of a square [`RationalMatrix`], taking it by value.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. The permanent of the $0
    /// \times 0$ matrix is 1. For dimensions greater than 2, each row is multiplied by the least
    /// common multiple of its denominators, the permanent of the resulting
    /// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix) is computed, and it is
    /// divided by the product of those multipliers.
    ///
    /// $$
    /// f(A) = \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)}.
    /// $$
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `permanent` for the
    /// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix) found by clearing the
    /// denominators of each row.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Permanent;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 2/3; 3/4, 4/5]").unwrap();
    /// assert_eq!(a.clone().permanent().to_string(), "9/10");
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; -7, 8, 10]").unwrap();
    /// assert_eq!(a.clone().permanent().to_string(), "-356/3");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_permanent` from `fmpq_mat/permanent.c`, FLINT 3.6.0, except
    /// that it always succeeds and panics if the matrix is not square.
    #[inline]
    fn permanent(self) -> Rational {
        permanent_helper(&self)
    }
}

impl Permanent for &RationalMatrix {
    type Output = Rational;

    /// Computes the permanent of a square [`RationalMatrix`], taking it by reference.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. The permanent of the $0
    /// \times 0$ matrix is 1. For dimensions greater than 2, each row is multiplied by the least
    /// common multiple of its denominators, the permanent of the resulting
    /// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix) is computed, and it is
    /// divided by the product of those multipliers.
    ///
    /// $$
    /// f(A) = \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)}.
    /// $$
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `permanent` for the
    /// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix) found by clearing the
    /// denominators of each row.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Permanent;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 2/3; 3/4, 4/5]").unwrap();
    /// assert_eq!((&a).permanent().to_string(), "9/10");
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; -7, 8, 10]").unwrap();
    /// assert_eq!((&a).permanent().to_string(), "-356/3");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_permanent` from `fmpq_mat/permanent.c`, FLINT 3.6.0, except
    /// that it always succeeds and panics if the matrix is not square.
    #[inline]
    fn permanent(self) -> Rational {
        permanent_helper(self)
    }
}
