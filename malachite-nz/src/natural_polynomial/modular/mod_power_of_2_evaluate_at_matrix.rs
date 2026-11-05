// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::modular::traits::{
    ModPowerOf2AddAssign, ModPowerOf2IsReduced, ModPowerOf2Mul,
};
use malachite_base::polynomial::evaluate_at_matrix::{MatrixEvaluationRing, evaluate_at_matrix};
use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;

// Square `NaturalMatrix`es modulo 2^pow.
pub(crate) struct NaturalModPowerOf2MatrixRing {
    pub(crate) pow: u64,
}

impl MatrixEvaluationRing for NaturalModPowerOf2MatrixRing {
    type Scalar = Natural;
    type Matrix = NaturalMatrix;

    #[inline]
    fn zero_matrix(&self, n: u64) -> NaturalMatrix {
        NaturalMatrix::zero(n, n)
    }

    #[inline]
    fn identity_matrix(&self, n: u64) -> NaturalMatrix {
        NaturalMatrix::identity(n)
    }

    #[inline]
    fn is_zero_matrix(&self, a: &NaturalMatrix) -> bool {
        a.is_zero()
    }

    #[inline]
    fn mul(&self, a: &NaturalMatrix, b: &NaturalMatrix) -> NaturalMatrix {
        a.mod_power_of_2_mul(b, self.pow)
    }

    fn add_scalar_assign(&self, a: &mut NaturalMatrix, c: &Natural) {
        let n = usize::exact_from(a.height());
        for i in 0..n {
            a.elements[i * n + i].mod_power_of_2_add_assign(c, self.pow);
        }
    }

    fn add_scaled_assign(&self, a: &mut NaturalMatrix, b: &NaturalMatrix, c: &Natural) {
        for (x, y) in a.elements.iter_mut().zip(b.elements.iter()) {
            x.mod_power_of_2_add_assign(c.mod_power_of_2_mul(y, self.pow), self.pow);
        }
    }
}

fn mod_power_of_2_evaluate_at_matrix_helper(
    p: &NaturalPolynomial,
    x: &NaturalMatrix,
    pow: u64,
) -> NaturalMatrix {
    assert_eq!(x.height(), x.width(), "the matrix must be square");
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "the polynomial must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        x.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^pow, but {x} has an element >= 2^{pow}"
    );
    evaluate_at_matrix(
        &NaturalModPowerOf2MatrixRing { pow },
        &p.coefficients,
        x,
        x.height(),
    )
}

impl ModPowerOf2Evaluate<&NaturalMatrix> for &NaturalPolynomial {
    type Output = NaturalMatrix;

    /// Evaluates a [`NaturalPolynomial`] at a square [`NaturalMatrix`], modulo $2^k$, taking both
    /// by reference. The coefficients and the elements of the matrix must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(p, A, k) = \sum_{i=0}^{d-1} c_i A^i \bmod 2^k,
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
    /// $T(n, d, k) = O((n^3 \sqrt{d} + n^2 d) k \log k \log\log k)$
    ///
    /// $M(n, d, k) = O(n^2 \sqrt{d} k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any coefficient of `self` or element of the matrix
    /// is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_evaluate(&a, 3).to_string(),
    ///     "[2, 2; 0, 2]"
    /// );
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 3x + 6 mod 8.
    /// let p = NaturalPolynomial::from_str("x^2+3*x+6").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_evaluate(&a, 3).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn mod_power_of_2_evaluate(self, x: &NaturalMatrix, pow: u64) -> NaturalMatrix {
        mod_power_of_2_evaluate_at_matrix_helper(self, x, pow)
    }
}

impl ModPowerOf2Evaluate<NaturalMatrix> for &NaturalPolynomial {
    type Output = NaturalMatrix;

    /// Evaluates a [`NaturalPolynomial`] at a square [`NaturalMatrix`], modulo $2^k$, taking the
    /// polynomial by reference and the matrix by value. The coefficients and the elements of the
    /// matrix must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, A, k) = \sum_{i=0}^{d-1} c_i A^i \bmod 2^k,
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
    /// $T(n, d, k) = O((n^3 \sqrt{d} + n^2 d) k \log k \log\log k)$
    ///
    /// $M(n, d, k) = O(n^2 \sqrt{d} k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any coefficient of `self` or element of the matrix
    /// is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_evaluate(a, 3).to_string(),
    ///     "[2, 2; 0, 2]"
    /// );
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 3x + 6 mod 8.
    /// let p = NaturalPolynomial::from_str("x^2+3*x+6").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_evaluate(a, 3).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn mod_power_of_2_evaluate(self, x: NaturalMatrix, pow: u64) -> NaturalMatrix {
        mod_power_of_2_evaluate_at_matrix_helper(self, &x, pow)
    }
}

impl ModPowerOf2Evaluate<&NaturalMatrix> for NaturalPolynomial {
    type Output = NaturalMatrix;

    /// Evaluates a [`NaturalPolynomial`] at a square [`NaturalMatrix`], modulo $2^k$, taking the
    /// polynomial by value and the matrix by reference. The coefficients and the elements of the
    /// matrix must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, A, k) = \sum_{i=0}^{d-1} c_i A^i \bmod 2^k,
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
    /// $T(n, d, k) = O((n^3 \sqrt{d} + n^2 d) k \log k \log\log k)$
    ///
    /// $M(n, d, k) = O(n^2 \sqrt{d} k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any coefficient of `self` or element of the matrix
    /// is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(p.mod_power_of_2_evaluate(&a, 3).to_string(), "[2, 2; 0, 2]");
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 3x + 6 mod 8.
    /// let p = NaturalPolynomial::from_str("x^2+3*x+6").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(p.mod_power_of_2_evaluate(&a, 3).to_string(), "[0, 0; 0, 0]");
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn mod_power_of_2_evaluate(self, x: &NaturalMatrix, pow: u64) -> NaturalMatrix {
        mod_power_of_2_evaluate_at_matrix_helper(&self, x, pow)
    }
}

impl ModPowerOf2Evaluate<NaturalMatrix> for NaturalPolynomial {
    type Output = NaturalMatrix;

    /// Evaluates a [`NaturalPolynomial`] at a square [`NaturalMatrix`], modulo $2^k$, taking both
    /// by value. The coefficients and the elements of the matrix must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(p, A, k) = \sum_{i=0}^{d-1} c_i A^i \bmod 2^k,
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
    /// $T(n, d, k) = O((n^3 \sqrt{d} + n^2 d) k \log k \log\log k)$
    ///
    /// $M(n, d, k) = O(n^2 \sqrt{d} k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any coefficient of `self` or element of the matrix
    /// is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(p.mod_power_of_2_evaluate(a, 3).to_string(), "[2, 2; 0, 2]");
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 3x + 6 mod 8.
    /// let p = NaturalPolynomial::from_str("x^2+3*x+6").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(p.mod_power_of_2_evaluate(a, 3).to_string(), "[0, 0; 0, 0]");
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn mod_power_of_2_evaluate(self, x: NaturalMatrix, pow: u64) -> NaturalMatrix {
        mod_power_of_2_evaluate_at_matrix_helper(&self, &x, pow)
    }
}
