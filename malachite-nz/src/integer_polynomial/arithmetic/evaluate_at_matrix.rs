// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_polynomial::IntegerPolynomial;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::arithmetic::traits::Evaluate;
use malachite_base::polynomial::evaluate_at_matrix::{MatrixEvaluationRing, evaluate_at_matrix};

// Square `IntegerMatrix`es.
pub(crate) struct IntegerMatrixRing;

impl MatrixEvaluationRing for IntegerMatrixRing {
    type Scalar = Integer;
    type Matrix = IntegerMatrix;

    #[inline]
    fn zero_matrix(&self, n: u64) -> IntegerMatrix {
        IntegerMatrix::zero(n, n)
    }

    #[inline]
    fn identity_matrix(&self, n: u64) -> IntegerMatrix {
        IntegerMatrix::identity(n)
    }

    #[inline]
    fn is_zero_matrix(&self, a: &IntegerMatrix) -> bool {
        a.is_zero()
    }

    #[inline]
    fn mul(&self, a: &IntegerMatrix, b: &IntegerMatrix) -> IntegerMatrix {
        a * b
    }

    fn add_scalar_assign(&self, a: &mut IntegerMatrix, c: &Integer) {
        let n = usize::exact_from(a.height());
        for i in 0..n {
            a.elements[i * n + i] += c;
        }
    }

    fn add_scaled_assign(&self, a: &mut IntegerMatrix, b: &IntegerMatrix, c: &Integer) {
        for (x, y) in a.elements.iter_mut().zip(b.elements.iter()) {
            x.add_mul_assign(c, y);
        }
    }
}

fn evaluate_at_matrix_helper(p: &IntegerPolynomial, x: &IntegerMatrix) -> IntegerMatrix {
    assert_eq!(x.height(), x.width(), "the matrix must be square");
    evaluate_at_matrix(&IntegerMatrixRing, &p.coefficients, x, x.height())
}

impl Evaluate<&IntegerMatrix> for &IntegerPolynomial {
    type Output = IntegerMatrix;

    /// Evaluates an [`IntegerPolynomial`] at a square [`IntegerMatrix`], taking both by reference.
    ///
    /// $$
    /// f(p, A) = \sum_{i=0}^{d-1} c_i A^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $d$ is its length, and $A^0$ is the identity
    /// matrix of the same size as $A$. The zero polynomial evaluates to the zero matrix.
    ///
    /// Short polynomials and small matrices are evaluated with Horner's rule. Otherwise the
    /// Paterson–Stockmeyer algorithm is used, which needs about $2\sqrt{d}$ matrix
    /// multiplications in place of $d - 1$, at the cost of storing about $\sqrt{d}$ powers of the
    /// matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, d, s) = O((n^3 \sqrt{d} + n^2 d) s \log s \log\log s)$
    ///
    /// $M(n, d, s) = O(n^2 \sqrt{d} s)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $s$ is $c + d(b + \log n)$, a bound on the number of significant bits of
    /// every intermediate element, where $c$ is the largest number of significant bits of any
    /// coefficient of `self` and $b$ that of any element of the matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::Evaluate;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2+1").unwrap();
    /// let a = IntegerMatrix::from_str("[0, -1; 1, 0]").unwrap();
    /// // A rotation by a quarter turn squares to -1.
    /// assert_eq!((&p).evaluate(&a).to_string(), "[0, 0; 0, 0]");
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix.
    /// let p = IntegerPolynomial::from_str("x^2-5*x-2").unwrap();
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&p).evaluate(&a).to_string(), "[0, 0; 0, 0]");
    /// let a = IntegerMatrix::from_str("[1, -2; 0, 1]").unwrap();
    /// assert_eq!((&p).evaluate(&a).to_string(), "[-6, 6; 0, -6]");
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn evaluate(self, x: &IntegerMatrix) -> IntegerMatrix {
        evaluate_at_matrix_helper(self, x)
    }
}

impl Evaluate<IntegerMatrix> for &IntegerPolynomial {
    type Output = IntegerMatrix;

    /// Evaluates an [`IntegerPolynomial`] at a square [`IntegerMatrix`], taking the polynomial by
    /// reference and the matrix by value.
    ///
    /// $$
    /// f(p, A) = \sum_{i=0}^{d-1} c_i A^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $d$ is its length, and $A^0$ is the identity
    /// matrix of the same size as $A$. The zero polynomial evaluates to the zero matrix.
    ///
    /// Short polynomials and small matrices are evaluated with Horner's rule. Otherwise the
    /// Paterson–Stockmeyer algorithm is used, which needs about $2\sqrt{d}$ matrix
    /// multiplications in place of $d - 1$, at the cost of storing about $\sqrt{d}$ powers of the
    /// matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, d, s) = O((n^3 \sqrt{d} + n^2 d) s \log s \log\log s)$
    ///
    /// $M(n, d, s) = O(n^2 \sqrt{d} s)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $s$ is $c + d(b + \log n)$, a bound on the number of significant bits of
    /// every intermediate element, where $c$ is the largest number of significant bits of any
    /// coefficient of `self` and $b$ that of any element of the matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::Evaluate;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2+1").unwrap();
    /// let a = IntegerMatrix::from_str("[0, -1; 1, 0]").unwrap();
    /// // A rotation by a quarter turn squares to -1.
    /// assert_eq!((&p).evaluate(a).to_string(), "[0, 0; 0, 0]");
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix.
    /// let p = IntegerPolynomial::from_str("x^2-5*x-2").unwrap();
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&p).evaluate(a).to_string(), "[0, 0; 0, 0]");
    /// let a = IntegerMatrix::from_str("[1, -2; 0, 1]").unwrap();
    /// assert_eq!((&p).evaluate(a).to_string(), "[-6, 6; 0, -6]");
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn evaluate(self, x: IntegerMatrix) -> IntegerMatrix {
        evaluate_at_matrix_helper(self, &x)
    }
}
