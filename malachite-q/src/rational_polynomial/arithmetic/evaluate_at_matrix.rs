// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use crate::rational_polynomial::RationalPolynomial;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{AddMulAssign, Reciprocal};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::arithmetic::traits::Evaluate;
use malachite_base::polynomial::evaluate_at_matrix::{MatrixEvaluationRing, evaluate_at_matrix};
use malachite_nz::integer::Integer;

// Square `RationalMatrix`es, with integer scalars: a `RationalPolynomial` is an integer polynomial
// over a common denominator, so the numerator is evaluated and the result divided by the
// denominator once.
pub(crate) struct RationalMatrixRing;

impl MatrixEvaluationRing for RationalMatrixRing {
    type Scalar = Integer;
    type Matrix = RationalMatrix;

    #[inline]
    fn zero_matrix(&self, n: u64) -> RationalMatrix {
        RationalMatrix::zero(n, n)
    }

    #[inline]
    fn identity_matrix(&self, n: u64) -> RationalMatrix {
        RationalMatrix::identity(n)
    }

    #[inline]
    fn is_zero_matrix(&self, a: &RationalMatrix) -> bool {
        a.is_zero()
    }

    #[inline]
    fn mul(&self, a: &RationalMatrix, b: &RationalMatrix) -> RationalMatrix {
        a * b
    }

    fn add_scalar_assign(&self, a: &mut RationalMatrix, c: &Integer) {
        let n = usize::exact_from(a.height());
        for i in 0..n {
            a.elements[i * n + i] += Rational::from(c);
        }
    }

    fn add_scaled_assign(&self, a: &mut RationalMatrix, b: &RationalMatrix, c: &Integer) {
        let c = Rational::from(c);
        for (x, y) in a.elements.iter_mut().zip(b.elements.iter()) {
            x.add_mul_assign(&c, y);
        }
    }
}

fn evaluate_at_matrix_helper(p: &RationalPolynomial, x: &RationalMatrix) -> RationalMatrix {
    assert_eq!(x.height(), x.width(), "the matrix must be square");
    let mut y = evaluate_at_matrix(
        &RationalMatrixRing,
        p.numerator.coefficients_asc(),
        x,
        x.height(),
    );
    if p.denominator != 1u32 {
        y *= Rational::from(&p.denominator).reciprocal();
    }
    y
}

impl Evaluate<&RationalMatrix> for &RationalPolynomial {
    type Output = RationalMatrix;

    /// Evaluates a [`RationalPolynomial`] at a square [`RationalMatrix`], taking both by reference.
    ///
    /// $$
    /// f(p, A) = \sum_{i=0}^{d-1} c_i A^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $d$ is its length, and $A^0$ is the identity
    /// matrix of the same size as $A$. The zero polynomial evaluates to the zero matrix.
    ///
    /// The polynomial is the quotient of a polynomial with integer coefficients and a positive
    /// integer. The integer polynomial is evaluated at the matrix, and the result is divided by the
    /// integer once. Short polynomials and small matrices are evaluated with Horner's rule.
    /// Otherwise the Paterson–Stockmeyer algorithm is used, which needs about $2\sqrt{d}$ matrix
    /// multiplications in place of $d - 1$, at the cost of storing about $\sqrt{d}$ powers of the
    /// matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, d, s) = O((n^3 \sqrt{d} + n^2 d)(s \log s \log\log s + s^2))$
    ///
    /// $M(n, d, s) = O(n^2 \sqrt{d} s)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $s$ is $c + d(n^2 b + \log n)$, a bound on the number of significant bits
    /// of every intermediate numerator and denominator, where $c$ is the largest number of
    /// significant bits of the numerator of any coefficient of `self` or of its common denominator,
    /// and $b$ that of the numerator or denominator of any element of the matrix: clearing the
    /// denominators of the matrix gives integer elements of $O(n^2 b)$ bits.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::Evaluate;
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2+1/2").unwrap();
    /// let a = RationalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!((&p).evaluate(&a).to_string(), "[1, 1; 0, 1]");
    ///
    /// // (x - 1/2)(x - 1/3) vanishes at a triangular matrix with those eigenvalues.
    /// let p = RationalPolynomial::from_str("x^2-5/6*x+1/6").unwrap();
    /// let a = RationalMatrix::from_str("[1/2, 1/3; 0, 1/3]").unwrap();
    /// assert_eq!((&p).evaluate(&a).to_string(), "[0, 0; 0, 0]");
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn evaluate(self, x: &RationalMatrix) -> RationalMatrix {
        evaluate_at_matrix_helper(self, x)
    }
}

impl Evaluate<RationalMatrix> for &RationalPolynomial {
    type Output = RationalMatrix;

    /// Evaluates a [`RationalPolynomial`] at a square [`RationalMatrix`], taking the polynomial by
    /// reference and the matrix by value.
    ///
    /// $$
    /// f(p, A) = \sum_{i=0}^{d-1} c_i A^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $d$ is its length, and $A^0$ is the identity
    /// matrix of the same size as $A$. The zero polynomial evaluates to the zero matrix.
    ///
    /// The polynomial is the quotient of a polynomial with integer coefficients and a positive
    /// integer. The integer polynomial is evaluated at the matrix, and the result is divided by the
    /// integer once. Short polynomials and small matrices are evaluated with Horner's rule.
    /// Otherwise the Paterson–Stockmeyer algorithm is used, which needs about $2\sqrt{d}$ matrix
    /// multiplications in place of $d - 1$, at the cost of storing about $\sqrt{d}$ powers of the
    /// matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, d, s) = O((n^3 \sqrt{d} + n^2 d)(s \log s \log\log s + s^2))$
    ///
    /// $M(n, d, s) = O(n^2 \sqrt{d} s)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $s$ is $c + d(n^2 b + \log n)$, a bound on the number of significant bits
    /// of every intermediate numerator and denominator, where $c$ is the largest number of
    /// significant bits of the numerator of any coefficient of `self` or of its common denominator,
    /// and $b$ that of the numerator or denominator of any element of the matrix: clearing the
    /// denominators of the matrix gives integer elements of $O(n^2 b)$ bits.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::Evaluate;
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2+1/2").unwrap();
    /// let a = RationalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!((&p).evaluate(a).to_string(), "[1, 1; 0, 1]");
    ///
    /// // (x - 1/2)(x - 1/3) vanishes at a triangular matrix with those eigenvalues.
    /// let p = RationalPolynomial::from_str("x^2-5/6*x+1/6").unwrap();
    /// let a = RationalMatrix::from_str("[1/2, 1/3; 0, 1/3]").unwrap();
    /// assert_eq!((&p).evaluate(a).to_string(), "[0, 0; 0, 0]");
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn evaluate(self, x: RationalMatrix) -> RationalMatrix {
        evaluate_at_matrix_helper(self, &x)
    }
}
