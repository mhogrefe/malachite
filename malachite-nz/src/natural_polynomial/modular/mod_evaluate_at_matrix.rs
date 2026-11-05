// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_polynomial::NaturalPolynomial;
use crate::platform::Limb;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::modular::traits::{ModAddAssign, ModAssign, ModIsReduced, ModMul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::evaluate_at_matrix::{MatrixEvaluationRing, evaluate_at_matrix};
use malachite_base::polynomial::modular::traits::ModEvaluate;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Square `NaturalMatrix`es modulo `m`.
pub(crate) struct NaturalModMatrixRing<'a> {
    pub(crate) m: &'a Natural,
}

impl MatrixEvaluationRing for NaturalModMatrixRing<'_> {
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
        a.mod_mul(b, self.m)
    }

    fn add_scalar_assign(&self, a: &mut NaturalMatrix, c: &Natural) {
        let n = usize::exact_from(a.height());
        for i in 0..n {
            a.elements[i * n + i].mod_add_assign(c, self.m);
        }
    }

    fn add_scaled_assign(&self, a: &mut NaturalMatrix, b: &NaturalMatrix, c: &Natural) {
        for (x, y) in a.elements.iter_mut().zip(b.elements.iter()) {
            x.add_mul_assign(c, y);
            x.mod_assign(self.m);
        }
    }
}

fn mod_evaluate_at_matrix_helper(
    p: &NaturalPolynomial,
    x: &NaturalMatrix,
    m: &Natural,
) -> NaturalMatrix {
    assert_eq!(x.height(), x.width(), "the matrix must be square");
    assert!(
        p.mod_is_reduced(m),
        "the polynomial must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(
        x.mod_is_reduced(m),
        "the matrix must be reduced mod m, but {x} has an element >= {m}"
    );
    if let Natural(Small(small_m)) = *m {
        // When m fits in a limb, every operation can be done in word-sized arithmetic.
        let p = UnsignedPolynomial::<Limb>::from_coefficients_asc(
            p.coefficients.iter().map(Limb::wrapping_from).collect(),
        );
        let x = UnsignedMatrix::<Limb>::from_owned_elements(
            x.height(),
            x.width(),
            x.elements.iter().map(Limb::wrapping_from).collect(),
        );
        NaturalMatrix::from(p.mod_evaluate(x, small_m))
    } else {
        evaluate_at_matrix(&NaturalModMatrixRing { m }, &p.coefficients, x, x.height())
    }
}

impl ModEvaluate<&NaturalMatrix, &Natural> for &NaturalPolynomial {
    type Output = NaturalMatrix;

    /// Evaluates a [`NaturalPolynomial`] at a square [`NaturalMatrix`], modulo a [`Natural`] $m$,
    /// taking all three by reference. The coefficients and the elements of the matrix must already
    /// be reduced modulo $m$.
    ///
    /// $$
    /// f(p, A, m) = \sum_{i=0}^{d-1} c_i A^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $d$ is its length, and $A^0$ is the identity
    /// matrix of the same size as $A$. The zero polynomial evaluates to the zero matrix.
    ///
    /// Short polynomials and small matrices are evaluated with Horner's rule. Otherwise the
    /// Paterson–Stockmeyer algorithm is used, which needs about $2\sqrt{d}$ matrix
    /// multiplications in place of $d - 1$, at the cost of storing about $\sqrt{d}$ powers of the
    /// matrix. When $m$ fits in a limb, the evaluation is done with word-sized arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, d, b) = O((n^3 \sqrt{d} + n^2 d) b \log b \log\log b)$
    ///
    /// $M(n, d, b) = O(n^2 \sqrt{d} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if $m$ is 0, if the matrix is not square, or if any coefficient of `self` or element
    /// of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&p).mod_evaluate(&a, &Natural::from(5u32)).to_string(),
    ///     "[2, 2; 0, 2]"
    /// );
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 2x + 5 mod 7.
    /// let p = NaturalPolynomial::from_str("x^2+2*x+5").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&p).mod_evaluate(&a, &Natural::from(7u32)).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn mod_evaluate(self, x: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
        mod_evaluate_at_matrix_helper(self, x, m)
    }
}

impl ModEvaluate<&NaturalMatrix, Natural> for &NaturalPolynomial {
    type Output = NaturalMatrix;

    /// Evaluates a [`NaturalPolynomial`] at a square [`NaturalMatrix`], modulo a [`Natural`] $m$,
    /// taking the polynomial and the matrix by reference and the modulus by value. The coefficients
    /// and the elements of the matrix must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, A, m) = \sum_{i=0}^{d-1} c_i A^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $d$ is its length, and $A^0$ is the identity
    /// matrix of the same size as $A$. The zero polynomial evaluates to the zero matrix.
    ///
    /// Short polynomials and small matrices are evaluated with Horner's rule. Otherwise the
    /// Paterson–Stockmeyer algorithm is used, which needs about $2\sqrt{d}$ matrix
    /// multiplications in place of $d - 1$, at the cost of storing about $\sqrt{d}$ powers of the
    /// matrix. When $m$ fits in a limb, the evaluation is done with word-sized arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, d, b) = O((n^3 \sqrt{d} + n^2 d) b \log b \log\log b)$
    ///
    /// $M(n, d, b) = O(n^2 \sqrt{d} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if $m$ is 0, if the matrix is not square, or if any coefficient of `self` or element
    /// of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&p).mod_evaluate(&a, Natural::from(5u32)).to_string(),
    ///     "[2, 2; 0, 2]"
    /// );
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 2x + 5 mod 7.
    /// let p = NaturalPolynomial::from_str("x^2+2*x+5").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&p).mod_evaluate(&a, Natural::from(7u32)).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn mod_evaluate(self, x: &NaturalMatrix, m: Natural) -> NaturalMatrix {
        mod_evaluate_at_matrix_helper(self, x, &m)
    }
}

impl ModEvaluate<NaturalMatrix, &Natural> for &NaturalPolynomial {
    type Output = NaturalMatrix;

    /// Evaluates a [`NaturalPolynomial`] at a square [`NaturalMatrix`], modulo a [`Natural`] $m$,
    /// taking the polynomial and the modulus by reference and the matrix by value. The coefficients
    /// and the elements of the matrix must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, A, m) = \sum_{i=0}^{d-1} c_i A^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $d$ is its length, and $A^0$ is the identity
    /// matrix of the same size as $A$. The zero polynomial evaluates to the zero matrix.
    ///
    /// Short polynomials and small matrices are evaluated with Horner's rule. Otherwise the
    /// Paterson–Stockmeyer algorithm is used, which needs about $2\sqrt{d}$ matrix
    /// multiplications in place of $d - 1$, at the cost of storing about $\sqrt{d}$ powers of the
    /// matrix. When $m$ fits in a limb, the evaluation is done with word-sized arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, d, b) = O((n^3 \sqrt{d} + n^2 d) b \log b \log\log b)$
    ///
    /// $M(n, d, b) = O(n^2 \sqrt{d} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if $m$ is 0, if the matrix is not square, or if any coefficient of `self` or element
    /// of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&p).mod_evaluate(a, &Natural::from(5u32)).to_string(),
    ///     "[2, 2; 0, 2]"
    /// );
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 2x + 5 mod 7.
    /// let p = NaturalPolynomial::from_str("x^2+2*x+5").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&p).mod_evaluate(a, &Natural::from(7u32)).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn mod_evaluate(self, x: NaturalMatrix, m: &Natural) -> NaturalMatrix {
        mod_evaluate_at_matrix_helper(self, &x, m)
    }
}

impl ModEvaluate<NaturalMatrix, Natural> for &NaturalPolynomial {
    type Output = NaturalMatrix;

    /// Evaluates a [`NaturalPolynomial`] at a square [`NaturalMatrix`], modulo a [`Natural`] $m$,
    /// taking the polynomial by reference and the matrix and the modulus by value. The coefficients
    /// and the elements of the matrix must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, A, m) = \sum_{i=0}^{d-1} c_i A^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $d$ is its length, and $A^0$ is the identity
    /// matrix of the same size as $A$. The zero polynomial evaluates to the zero matrix.
    ///
    /// Short polynomials and small matrices are evaluated with Horner's rule. Otherwise the
    /// Paterson–Stockmeyer algorithm is used, which needs about $2\sqrt{d}$ matrix
    /// multiplications in place of $d - 1$, at the cost of storing about $\sqrt{d}$ powers of the
    /// matrix. When $m$ fits in a limb, the evaluation is done with word-sized arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, d, b) = O((n^3 \sqrt{d} + n^2 d) b \log b \log\log b)$
    ///
    /// $M(n, d, b) = O(n^2 \sqrt{d} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if $m$ is 0, if the matrix is not square, or if any coefficient of `self` or element
    /// of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&p).mod_evaluate(a, Natural::from(5u32)).to_string(),
    ///     "[2, 2; 0, 2]"
    /// );
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 2x + 5 mod 7.
    /// let p = NaturalPolynomial::from_str("x^2+2*x+5").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&p).mod_evaluate(a, Natural::from(7u32)).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn mod_evaluate(self, x: NaturalMatrix, m: Natural) -> NaturalMatrix {
        mod_evaluate_at_matrix_helper(self, &x, &m)
    }
}

impl ModEvaluate<&NaturalMatrix, &Natural> for NaturalPolynomial {
    type Output = NaturalMatrix;

    /// Evaluates a [`NaturalPolynomial`] at a square [`NaturalMatrix`], modulo a [`Natural`] $m$,
    /// taking the polynomial by value and the matrix and the modulus by reference. The coefficients
    /// and the elements of the matrix must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, A, m) = \sum_{i=0}^{d-1} c_i A^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $d$ is its length, and $A^0$ is the identity
    /// matrix of the same size as $A$. The zero polynomial evaluates to the zero matrix.
    ///
    /// Short polynomials and small matrices are evaluated with Horner's rule. Otherwise the
    /// Paterson–Stockmeyer algorithm is used, which needs about $2\sqrt{d}$ matrix
    /// multiplications in place of $d - 1$, at the cost of storing about $\sqrt{d}$ powers of the
    /// matrix. When $m$ fits in a limb, the evaluation is done with word-sized arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, d, b) = O((n^3 \sqrt{d} + n^2 d) b \log b \log\log b)$
    ///
    /// $M(n, d, b) = O(n^2 \sqrt{d} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if $m$ is 0, if the matrix is not square, or if any coefficient of `self` or element
    /// of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(
    ///     p.mod_evaluate(&a, &Natural::from(5u32)).to_string(),
    ///     "[2, 2; 0, 2]"
    /// );
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 2x + 5 mod 7.
    /// let p = NaturalPolynomial::from_str("x^2+2*x+5").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     p.mod_evaluate(&a, &Natural::from(7u32)).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn mod_evaluate(self, x: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
        mod_evaluate_at_matrix_helper(&self, x, m)
    }
}

impl ModEvaluate<&NaturalMatrix, Natural> for NaturalPolynomial {
    type Output = NaturalMatrix;

    /// Evaluates a [`NaturalPolynomial`] at a square [`NaturalMatrix`], modulo a [`Natural`] $m$,
    /// taking the polynomial and the modulus by value and the matrix by reference. The coefficients
    /// and the elements of the matrix must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, A, m) = \sum_{i=0}^{d-1} c_i A^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $d$ is its length, and $A^0$ is the identity
    /// matrix of the same size as $A$. The zero polynomial evaluates to the zero matrix.
    ///
    /// Short polynomials and small matrices are evaluated with Horner's rule. Otherwise the
    /// Paterson–Stockmeyer algorithm is used, which needs about $2\sqrt{d}$ matrix
    /// multiplications in place of $d - 1$, at the cost of storing about $\sqrt{d}$ powers of the
    /// matrix. When $m$ fits in a limb, the evaluation is done with word-sized arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, d, b) = O((n^3 \sqrt{d} + n^2 d) b \log b \log\log b)$
    ///
    /// $M(n, d, b) = O(n^2 \sqrt{d} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if $m$ is 0, if the matrix is not square, or if any coefficient of `self` or element
    /// of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(
    ///     p.mod_evaluate(&a, Natural::from(5u32)).to_string(),
    ///     "[2, 2; 0, 2]"
    /// );
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 2x + 5 mod 7.
    /// let p = NaturalPolynomial::from_str("x^2+2*x+5").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     p.mod_evaluate(&a, Natural::from(7u32)).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn mod_evaluate(self, x: &NaturalMatrix, m: Natural) -> NaturalMatrix {
        mod_evaluate_at_matrix_helper(&self, x, &m)
    }
}

impl ModEvaluate<NaturalMatrix, &Natural> for NaturalPolynomial {
    type Output = NaturalMatrix;

    /// Evaluates a [`NaturalPolynomial`] at a square [`NaturalMatrix`], modulo a [`Natural`] $m$,
    /// taking the polynomial and the matrix by value and the modulus by reference. The coefficients
    /// and the elements of the matrix must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, A, m) = \sum_{i=0}^{d-1} c_i A^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $d$ is its length, and $A^0$ is the identity
    /// matrix of the same size as $A$. The zero polynomial evaluates to the zero matrix.
    ///
    /// Short polynomials and small matrices are evaluated with Horner's rule. Otherwise the
    /// Paterson–Stockmeyer algorithm is used, which needs about $2\sqrt{d}$ matrix
    /// multiplications in place of $d - 1$, at the cost of storing about $\sqrt{d}$ powers of the
    /// matrix. When $m$ fits in a limb, the evaluation is done with word-sized arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, d, b) = O((n^3 \sqrt{d} + n^2 d) b \log b \log\log b)$
    ///
    /// $M(n, d, b) = O(n^2 \sqrt{d} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if $m$ is 0, if the matrix is not square, or if any coefficient of `self` or element
    /// of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(
    ///     p.mod_evaluate(a, &Natural::from(5u32)).to_string(),
    ///     "[2, 2; 0, 2]"
    /// );
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 2x + 5 mod 7.
    /// let p = NaturalPolynomial::from_str("x^2+2*x+5").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     p.mod_evaluate(a, &Natural::from(7u32)).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn mod_evaluate(self, x: NaturalMatrix, m: &Natural) -> NaturalMatrix {
        mod_evaluate_at_matrix_helper(&self, &x, m)
    }
}

impl ModEvaluate<NaturalMatrix, Natural> for NaturalPolynomial {
    type Output = NaturalMatrix;

    /// Evaluates a [`NaturalPolynomial`] at a square [`NaturalMatrix`], modulo a [`Natural`] $m$,
    /// taking all three by value. The coefficients and the elements of the matrix must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(p, A, m) = \sum_{i=0}^{d-1} c_i A^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $d$ is its length, and $A^0$ is the identity
    /// matrix of the same size as $A$. The zero polynomial evaluates to the zero matrix.
    ///
    /// Short polynomials and small matrices are evaluated with Horner's rule. Otherwise the
    /// Paterson–Stockmeyer algorithm is used, which needs about $2\sqrt{d}$ matrix
    /// multiplications in place of $d - 1$, at the cost of storing about $\sqrt{d}$ powers of the
    /// matrix. When $m$ fits in a limb, the evaluation is done with word-sized arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, d, b) = O((n^3 \sqrt{d} + n^2 d) b \log b \log\log b)$
    ///
    /// $M(n, d, b) = O(n^2 \sqrt{d} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, $d$ is
    /// `self.len()`, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if $m$ is 0, if the matrix is not square, or if any coefficient of `self` or element
    /// of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(
    ///     p.mod_evaluate(a, Natural::from(5u32)).to_string(),
    ///     "[2, 2; 0, 2]"
    /// );
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 2x + 5 mod 7.
    /// let p = NaturalPolynomial::from_str("x^2+2*x+5").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     p.mod_evaluate(a, Natural::from(7u32)).to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    ///
    /// This uses the algorithm of `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0, which
    /// FLINT provides for polynomials over word-sized moduli only.
    #[inline]
    fn mod_evaluate(self, x: NaturalMatrix, m: Natural) -> NaturalMatrix {
        mod_evaluate_at_matrix_helper(&self, &x, &m)
    }
}
