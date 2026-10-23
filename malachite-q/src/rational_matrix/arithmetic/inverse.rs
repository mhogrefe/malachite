// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use crate::rational_matrix::arithmetic::solve::solve_cleared;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Inverse, MulSubMul, Reciprocal, ReciprocalAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;

// Returns the inverse of a square matrix, or `None` if it is singular.
//
// This is equivalent to `fmpq_mat_inv` from `fmpq_mat/inv.c`, FLINT 3.6.0, except that it panics if
// the matrix is not square.
fn inverse_helper(a: &RationalMatrix) -> Option<RationalMatrix> {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    let matrix = |elements: Vec<Rational>| RationalMatrix {
        elements,
        height: n,
        width: n,
    };
    match n {
        0 => Some(matrix(Vec::new())),
        1 => {
            let x = &a.elements[0];
            if *x == 0u32 {
                None
            } else {
                Some(matrix(vec![x.reciprocal()]))
            }
        }
        2 => {
            let [a00, a01, a10, a11] = a.elements.as_slice() else {
                unreachable!()
            };
            let mut d = a00.mul_sub_mul(a11, a01, a10);
            if d == 0u32 {
                return None;
            }
            d.reciprocal_assign();
            Some(matrix(vec![a11 * &d, -(a01 * &d), -(a10 * &d), a00 * &d]))
        }
        _ => {
            // A = D^(-1) A', with A' an integer matrix and D the diagonal matrix of row
            // denominators, so A^(-1) = A'^(-1) D, which solves A' X = D.
            let (numerators, denominators) = a.to_numerators_and_row_denominators();
            let mut d = vec![Integer::ZERO; n * n];
            for (i, den) in denominators.elements.into_iter().enumerate() {
                d[i * (n + 1)] = Integer::from(den);
            }
            solve_cleared(
                &numerators,
                &IntegerMatrix::from_owned_elements(u64::exact_from(n), u64::exact_from(n), d),
            )
        }
    }
}

impl Inverse for RationalMatrix {
    type Output = Self;

    /// Finds the inverse of a square [`RationalMatrix`], taking it by value.
    ///
    /// `None` is returned if the matrix is singular.
    ///
    /// Matrices of dimension at most 2 are inverted directly. For larger ones, the denominators of
    /// each row of $A$ are cleared, writing $A = D^{-1}A'$ with $A'$ an integer matrix and $D$ the
    /// diagonal matrix of row denominators, and $A^{-1} = A'^{-1}D$ is found by solving $A'X = D$
    /// as for [`IntegerMatrix`].
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $b$ is
    /// the maximum number of significant bits of the numerators and denominators of the elements of
    /// the matrix: clearing the denominators of the rows gives integer elements of $O(nb)$ bits,
    /// and the integer system is solved within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve) for [`IntegerMatrix`] at
    /// that size.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Inverse;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.inverse().unwrap().to_string(), "[-2, 1; 3/2, -1/2]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; 1/4, 1/5]").unwrap();
    /// assert_eq!(a.inverse().unwrap().to_string(), "[12, -20; -15, 30]");
    ///
    /// // A singular matrix
    /// let a = RationalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// assert_eq!(a.inverse(), None);
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_inv` from `fmpq_mat/inv.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square.
    #[inline]
    fn inverse(self) -> Option<Self> {
        inverse_helper(&self)
    }
}

impl Inverse for &RationalMatrix {
    type Output = RationalMatrix;

    /// Finds the inverse of a square [`RationalMatrix`], taking it by reference.
    ///
    /// `None` is returned if the matrix is singular.
    ///
    /// Matrices of dimension at most 2 are inverted directly. For larger ones, the denominators of
    /// each row of $A$ are cleared, writing $A = D^{-1}A'$ with $A'$ an integer matrix and $D$ the
    /// diagonal matrix of row denominators, and $A^{-1} = A'^{-1}D$ is found by solving $A'X = D$
    /// as for [`IntegerMatrix`].
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $b$ is
    /// the maximum number of significant bits of the numerators and denominators of the elements of
    /// the matrix: clearing the denominators of the rows gives integer elements of $O(nb)$ bits,
    /// and the integer system is solved within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve) for [`IntegerMatrix`] at
    /// that size.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Inverse;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).inverse().unwrap().to_string(), "[-2, 1; 3/2, -1/2]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; 1/4, 1/5]").unwrap();
    /// assert_eq!((&a).inverse().unwrap().to_string(), "[12, -20; -15, 30]");
    ///
    /// // A singular matrix
    /// let a = RationalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// assert_eq!((&a).inverse(), None);
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_inv` from `fmpq_mat/inv.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square.
    #[inline]
    fn inverse(self) -> Option<RationalMatrix> {
        inverse_helper(self)
    }
}
