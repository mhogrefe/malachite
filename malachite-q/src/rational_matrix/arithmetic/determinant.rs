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
use malachite_base::num::arithmetic::traits::{Determinant, MulSubMul};
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// Returns the determinant of a square matrix: directly for dimensions up to 2, and otherwise as the
// determinant of the integer matrix found by clearing the denominators of each row, divided by the
// product of those denominators.
//
// This is equivalent to `fmpq_mat_det` from `fmpq_mat/det.c`, FLINT 3.6.0, except that it panics if
// the matrix is not square.
fn determinant_helper(a: &RationalMatrix) -> Rational {
    assert_eq!(a.height, a.width, "the matrix must be square");
    match a.height {
        0 => Rational::ONE,
        1 => a.elements[0].clone(),
        2 => {
            let [a00, a01, a10, a11] = a.elements.as_slice() else {
                unreachable!()
            };
            a00.mul_sub_mul(a11, a01, a10)
        }
        _ => {
            let (numerators, denominators) = a.to_numerators_and_row_denominators();
            let denominator: Natural = denominators.elements.into_iter().product();
            Rational::from_integers(numerators.determinant(), Integer::from(denominator))
        }
    }
}

impl Determinant for RationalMatrix {
    type Output = Rational;

    /// Computes the determinant of a square [`RationalMatrix`], taking it by value.
    ///
    /// Matrices of dimension at most 2 are handled directly. For larger ones, the denominators of
    /// each row are cleared, and the determinant of the resulting
    /// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix), computed as by
    /// [`determinant`](Determinant::determinant) for integer matrices, is divided by the product of
    /// the row denominators.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^5 b \log (nb) \log\log (nb) + n^5 b^2)$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $b$ is
    /// the maximum number of significant bits of the numerators and denominators of its elements:
    /// clearing the denominators of the rows gives integer elements of $O(nb)$ bits.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Determinant;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; 1/4, 1/5]").unwrap();
    /// assert_eq!(a.determinant().to_string(), "1/60");
    ///
    /// let a = RationalMatrix::from_str("[1, 1/2, 1/3; 1/2, 1/3, 1/4; 1/3, 1/4, 1/5]").unwrap();
    /// assert_eq!(a.determinant().to_string(), "1/2160");
    ///
    /// // A singular matrix
    /// let a = RationalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// assert_eq!(a.determinant().to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_det` from `fmpq_mat/det.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square.
    #[inline]
    fn determinant(self) -> Rational {
        determinant_helper(&self)
    }
}

impl Determinant for &RationalMatrix {
    type Output = Rational;

    /// Computes the determinant of a square [`RationalMatrix`], taking it by reference.
    ///
    /// Matrices of dimension at most 2 are handled directly. For larger ones, the denominators of
    /// each row are cleared, and the determinant of the resulting
    /// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix), computed as by
    /// [`determinant`](Determinant::determinant) for integer matrices, is divided by the product of
    /// the row denominators.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^5 b \log (nb) \log\log (nb) + n^5 b^2)$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $b$ is
    /// the maximum number of significant bits of the numerators and denominators of its elements:
    /// clearing the denominators of the rows gives integer elements of $O(nb)$ bits.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Determinant;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; 1/4, 1/5]").unwrap();
    /// assert_eq!((&a).determinant().to_string(), "1/60");
    ///
    /// let a = RationalMatrix::from_str("[1, 1/2, 1/3; 1/2, 1/3, 1/4; 1/3, 1/4, 1/5]").unwrap();
    /// assert_eq!((&a).determinant().to_string(), "1/2160");
    ///
    /// // A singular matrix
    /// let a = RationalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// assert_eq!((&a).determinant().to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_det` from `fmpq_mat/det.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square.
    #[inline]
    fn determinant(self) -> Rational {
        determinant_helper(self)
    }
}
