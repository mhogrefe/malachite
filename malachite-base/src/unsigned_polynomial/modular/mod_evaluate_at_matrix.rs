// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::modular::traits::{ModIsReduced, ModMul};
use crate::polynomial::evaluate_at_matrix::{MatrixEvaluationRing, evaluate_at_matrix};
use crate::polynomial::modular::traits::ModEvaluate;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Square `UnsignedMatrix`s modulo `m`.
pub(crate) struct ModMatrixRing<T: PrimitiveUnsigned> {
    pub(crate) m: T,
}

impl<T: PrimitiveUnsigned> MatrixEvaluationRing for ModMatrixRing<T> {
    type Scalar = T;
    type Matrix = UnsignedMatrix<T>;

    #[inline]
    fn zero_matrix(&self, n: u64) -> UnsignedMatrix<T> {
        UnsignedMatrix::zero(n, n)
    }

    #[inline]
    fn identity_matrix(&self, n: u64) -> UnsignedMatrix<T> {
        UnsignedMatrix::identity(n)
    }

    #[inline]
    fn is_zero_matrix(&self, a: &UnsignedMatrix<T>) -> bool {
        a.is_zero()
    }

    #[inline]
    fn mul(&self, a: &UnsignedMatrix<T>, b: &UnsignedMatrix<T>) -> UnsignedMatrix<T> {
        a.mod_mul(b, self.m)
    }

    fn add_scalar_assign(&self, a: &mut UnsignedMatrix<T>, c: &T) {
        let n = usize::exact_from(a.height());
        for i in 0..n {
            a.elements[i * n + i].mod_add_assign(*c, self.m);
        }
    }

    fn add_scaled_assign(&self, a: &mut UnsignedMatrix<T>, b: &UnsignedMatrix<T>, c: &T) {
        for (x, &y) in a.elements.iter_mut().zip(b.elements.iter()) {
            x.mod_add_assign(c.mod_mul(y, self.m), self.m);
        }
    }
}

fn mod_evaluate_at_matrix_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    x: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    assert_eq!(x.height(), x.width(), "the matrix must be square");
    assert!(
        p.mod_is_reduced(&m),
        "the polynomial must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(
        x.mod_is_reduced(&m),
        "the matrix must be reduced mod m, but {x} has an element >= {m}"
    );
    evaluate_at_matrix(&ModMatrixRing { m }, &p.coefficients, x, x.height())
}

impl<T: PrimitiveUnsigned> ModEvaluate<&UnsignedMatrix<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedMatrix<T>;

    /// Evaluates an [`UnsignedPolynomial`] at a square [`UnsignedMatrix`], modulo a value of the
    /// coefficient type, taking both by reference. The coefficients and the elements of the matrix
    /// must already be reduced modulo `m`.
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
    /// matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, d) = O(n^3 \sqrt{d} + n^2 d)$
    ///
    /// $M(n, d) = O(n^2 \sqrt{d})$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $d$ is
    /// `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if the matrix is not square, or if any coefficient of `self` or element
    /// of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!((&p).mod_evaluate(&a, 5).to_string(), "[2, 2; 0, 2]");
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 2x + 5 mod 7.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+2*x+5").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&p).mod_evaluate(&a, 7).to_string(), "[0, 0; 0, 0]");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0.
    #[inline]
    fn mod_evaluate(self, x: &UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
        mod_evaluate_at_matrix_helper(self, x, m)
    }
}

impl<T: PrimitiveUnsigned> ModEvaluate<&UnsignedMatrix<T>, T> for UnsignedPolynomial<T> {
    type Output = UnsignedMatrix<T>;

    /// Evaluates an [`UnsignedPolynomial`] at a square [`UnsignedMatrix`], modulo a value of the
    /// coefficient type, taking the polynomial by value and the matrix by reference. The
    /// coefficients and the elements of the matrix must already be reduced modulo `m`.
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
    /// matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, d) = O(n^3 \sqrt{d} + n^2 d)$
    ///
    /// $M(n, d) = O(n^2 \sqrt{d})$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $d$ is
    /// `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if the matrix is not square, or if any coefficient of `self` or element
    /// of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(p.mod_evaluate(&a, 5).to_string(), "[2, 2; 0, 2]");
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 2x + 5 mod 7.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+2*x+5").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(p.mod_evaluate(&a, 7).to_string(), "[0, 0; 0, 0]");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0.
    #[inline]
    fn mod_evaluate(self, x: &UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
        mod_evaluate_at_matrix_helper(&self, x, m)
    }
}

impl<T: PrimitiveUnsigned> ModEvaluate<UnsignedMatrix<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedMatrix<T>;

    /// Evaluates an [`UnsignedPolynomial`] at a square [`UnsignedMatrix`], modulo a value of the
    /// coefficient type, taking the polynomial by reference and the matrix by value. The
    /// coefficients and the elements of the matrix must already be reduced modulo `m`.
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
    /// matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, d) = O(n^3 \sqrt{d} + n^2 d)$
    ///
    /// $M(n, d) = O(n^2 \sqrt{d})$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $d$ is
    /// `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if the matrix is not square, or if any coefficient of `self` or element
    /// of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!((&p).mod_evaluate(a, 5).to_string(), "[2, 2; 0, 2]");
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 2x + 5 mod 7.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+2*x+5").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&p).mod_evaluate(a, 7).to_string(), "[0, 0; 0, 0]");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0.
    #[inline]
    fn mod_evaluate(self, x: UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
        mod_evaluate_at_matrix_helper(self, &x, m)
    }
}

impl<T: PrimitiveUnsigned> ModEvaluate<UnsignedMatrix<T>, T> for UnsignedPolynomial<T> {
    type Output = UnsignedMatrix<T>;

    /// Evaluates an [`UnsignedPolynomial`] at a square [`UnsignedMatrix`], modulo a value of the
    /// coefficient type, taking both by value. The coefficients and the elements of the matrix must
    /// already be reduced modulo `m`.
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
    /// matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, d) = O(n^3 \sqrt{d} + n^2 d)$
    ///
    /// $M(n, d) = O(n^2 \sqrt{d})$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $d$ is
    /// `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, if the matrix is not square, or if any coefficient of `self` or element
    /// of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluate;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 0, 1]").unwrap();
    /// assert_eq!(p.mod_evaluate(a, 5).to_string(), "[2, 2; 0, 2]");
    ///
    /// // The characteristic polynomial of a matrix vanishes at the matrix. That of [1, 2; 3, 4] is
    /// // x^2 - 5x - 2, which is x^2 + 2x + 5 mod 7.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+2*x+5").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(p.mod_evaluate(a, 7).to_string(), "[0, 0; 0, 0]");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0.
    #[inline]
    fn mod_evaluate(self, x: UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
        mod_evaluate_at_matrix_helper(&self, &x, m)
    }
}
