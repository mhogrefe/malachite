// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2012 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::CharacteristicPolynomial;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

// The characteristic polynomial of the square matrix $N/d$, where $N$ is `numerators`: that of $N$
// with its coefficient of $x^k$ multiplied by $d^k$, over $d^n$, since $\det(xI - N/d) = d^{-n}
// \det(dxI - N)$.
//
// This is equivalent to `_fmpq_mat_charpoly` from `fmpq_mat/charpoly.c`, FLINT 3.6.0, followed by
// `fmpq_poly_canonicalise`.
fn charpoly_helper(numerators: &IntegerMatrix, d: &Natural) -> RationalPolynomial {
    let mut coefficients = numerators
        .characteristic_polynomial()
        .into_coefficients_asc();
    if *d == 1u32 || coefficients.len() == 1 {
        return RationalPolynomial::from(IntegerPolynomial::from_coefficients_asc(coefficients));
    }
    let mut power = d.clone();
    let n = coefficients.len() - 1;
    for (k, c) in coefficients.iter_mut().enumerate().skip(1) {
        *c *= Integer::from(&power);
        if k < n {
            power *= d;
        }
    }
    RationalPolynomial::from_numerator_and_denominator(
        IntegerPolynomial::from_coefficients_asc(coefficients),
        power,
    )
}

impl CharacteristicPolynomial for RationalMatrix {
    type Output = RationalPolynomial;

    /// Computes the characteristic polynomial of a square [`RationalMatrix`], taking it by value.
    ///
    /// The characteristic polynomial of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic
    /// polynomial of degree $n$; that of the $0 \times 0$ matrix is 1. Writing $A = N/d$, where $d$
    /// is the least common multiple of the denominators of the elements of $A$ and $N$ is an
    /// [`IntegerMatrix`], the characteristic polynomial of $N$ is computed as by
    /// [`characteristic_polynomial`](CharacteristicPolynomial::characteristic_polynomial) for
    /// integer matrices, and its coefficient of $x^k$ is multiplied by $d^k$ and divided by $d^n$.
    ///
    /// $$
    /// f(A) = \det(xI - A) = d^{-n} \det(dxI - N).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^7 b \log (nb) \log\log (nb) + n^7 b^2)$
    ///
    /// $M(n, b) = O(n^5 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $b$ is
    /// the maximum number of significant bits of the numerators and denominators of its elements:
    /// clearing the denominators gives integer elements of $O(n^2 b)$ bits.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CharacteristicPolynomial;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(a.clone().characteristic_polynomial().to_string(), "1");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// assert_eq!(
    ///     a.clone().characteristic_polynomial().to_string(),
    ///     "x^2-5/4*x+25/24"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.clone().characteristic_polynomial().to_string(),
    ///     "x^2-5*x-2"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3, 1; 2, 3/4, 5; -1, 0, 2/3]").unwrap();
    /// assert_eq!(
    ///     a.clone().characteristic_polynomial().to_string(),
    ///     "x^3-23/12*x^2+37/24*x+10/9"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_charpoly` from `fmpq_mat/charpoly.c`, FLINT 3.6.0.
    fn characteristic_polynomial(self) -> RationalPolynomial {
        assert_eq!(self.height, self.width, "the matrix must be square");
        let (numerators, d) = self.into_numerators_and_denominator();
        charpoly_helper(&numerators, &d)
    }
}

impl CharacteristicPolynomial for &RationalMatrix {
    type Output = RationalPolynomial;

    /// Computes the characteristic polynomial of a square [`RationalMatrix`], taking it by
    /// reference.
    ///
    /// The characteristic polynomial of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic
    /// polynomial of degree $n$; that of the $0 \times 0$ matrix is 1. Writing $A = N/d$, where $d$
    /// is the least common multiple of the denominators of the elements of $A$ and $N$ is an
    /// [`IntegerMatrix`], the characteristic polynomial of $N$ is computed as by
    /// [`characteristic_polynomial`](CharacteristicPolynomial::characteristic_polynomial) for
    /// integer matrices, and its coefficient of $x^k$ is multiplied by $d^k$ and divided by $d^n$.
    ///
    /// $$
    /// f(A) = \det(xI - A) = d^{-n} \det(dxI - N).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^7 b \log (nb) \log\log (nb) + n^7 b^2)$
    ///
    /// $M(n, b) = O(n^5 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $b$ is
    /// the maximum number of significant bits of the numerators and denominators of its elements:
    /// clearing the denominators gives integer elements of $O(n^2 b)$ bits.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CharacteristicPolynomial;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!((&a).characteristic_polynomial().to_string(), "1");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// assert_eq!(
    ///     (&a).characteristic_polynomial().to_string(),
    ///     "x^2-5/4*x+25/24"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).characteristic_polynomial().to_string(), "x^2-5*x-2");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3, 1; 2, 3/4, 5; -1, 0, 2/3]").unwrap();
    /// assert_eq!(
    ///     (&a).characteristic_polynomial().to_string(),
    ///     "x^3-23/12*x^2+37/24*x+10/9"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_charpoly` from `fmpq_mat/charpoly.c`, FLINT 3.6.0.
    fn characteristic_polynomial(self) -> RationalPolynomial {
        assert_eq!(self.height, self.width, "the matrix must be square");
        let (numerators, d) = self.to_numerators_and_denominator();
        charpoly_helper(&numerators, &d)
    }
}
