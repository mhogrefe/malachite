// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use core::mem::replace;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::{NthDerivative, NthDerivativeAssign};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

impl NthDerivative for RationalPolynomial {
    type Output = Self;

    /// Computes the $n$th derivative of a [`RationalPolynomial`], taking it by value.
    ///
    /// $$
    /// f(p, n) = p^{(n)} = \sum_{i=n}^d i^{\underline n}a_ix^{i-n}.
    /// $$
    ///
    /// Here $i^{\underline n} = i(i-1)\cdots(i-n+1)$ is a falling factorial, and $d$ is the degree.
    /// The zeroth derivative is the polynomial itself, and a polynomial of degree less than $n$ has
    /// $n$th derivative zero.
    ///
    /// The numerator is differentiated, and since the falling factorials can share factors with the
    /// denominator, the result is then brought back to lowest terms.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the total number of bits of the
    /// coefficients, and $k$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::NthDerivative;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/12*x^4+1/2*x^2+1").unwrap();
    /// assert_eq!(p.clone().nth_derivative(2).to_string(), "x^2+1");
    /// assert_eq!(
    ///     p.clone().nth_derivative(0).to_string(),
    ///     "1/12*x^4+1/2*x^2+1"
    /// );
    /// assert_eq!(p.nth_derivative(5), RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_nth_derivative` from `fmpq_poly/nth_derivative.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn nth_derivative(mut self, n: u64) -> Self {
        self.nth_derivative_assign(n);
        self
    }
}

impl NthDerivative for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Computes the $n$th derivative of a [`RationalPolynomial`], taking it by reference.
    ///
    /// $$
    /// f(p, n) = p^{(n)} = \sum_{i=n}^d i^{\underline n}a_ix^{i-n}.
    /// $$
    ///
    /// Here $i^{\underline n} = i(i-1)\cdots(i-n+1)$ is a falling factorial, and $d$ is the degree.
    /// The zeroth derivative is the polynomial itself, and a polynomial of degree less than $n$ has
    /// $n$th derivative zero.
    ///
    /// The numerator is differentiated, and since the falling factorials can share factors with the
    /// denominator, the result is then brought back to lowest terms.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the total number of bits of the
    /// coefficients, and $k$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::NthDerivative;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/12*x^4+1/2*x^2+1").unwrap();
    /// assert_eq!((&p).nth_derivative(2).to_string(), "x^2+1");
    /// assert_eq!((&p).nth_derivative(0).to_string(), "1/12*x^4+1/2*x^2+1");
    /// assert_eq!((&p).nth_derivative(5), RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_nth_derivative` from `fmpq_poly/nth_derivative.c`, FLINT
    /// 3.6.0.
    fn nth_derivative(self, n: u64) -> RationalPolynomial {
        if n == 0 {
            return self.clone();
        }
        RationalPolynomial::canonicalize(
            (&self.numerator).nth_derivative(n),
            self.denominator.clone(),
        )
    }
}

impl NthDerivativeAssign for RationalPolynomial {
    /// Replaces a [`RationalPolynomial`] with its $n$th derivative.
    ///
    /// $$
    /// p \gets p^{(n)} = \sum_{i=n}^d i^{\underline n}a_ix^{i-n}.
    /// $$
    ///
    /// Here $i^{\underline n} = i(i-1)\cdots(i-n+1)$ is a falling factorial, and $d$ is the degree.
    /// The zeroth derivative is the polynomial itself, and a polynomial of degree less than $n$ has
    /// $n$th derivative zero.
    ///
    /// The numerator is differentiated, and since the falling factorials can share factors with the
    /// denominator, the result is then brought back to lowest terms.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the total number of bits of the
    /// coefficients, and $k$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::NthDerivativeAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/12*x^4+1/2*x^2+1").unwrap();
    /// p.nth_derivative_assign(2);
    /// assert_eq!(p.to_string(), "x^2+1");
    ///
    /// let mut p = RationalPolynomial::from_str("1/12*x^4+1/2*x^2+1").unwrap();
    /// p.nth_derivative_assign(0);
    /// assert_eq!(p.to_string(), "1/12*x^4+1/2*x^2+1");
    ///
    /// let mut p = RationalPolynomial::from_str("1/12*x^4+1/2*x^2+1").unwrap();
    /// p.nth_derivative_assign(5);
    /// assert_eq!(p, RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_nth_derivative` from `fmpq_poly/nth_derivative.c`, FLINT
    /// 3.6.0.
    fn nth_derivative_assign(&mut self, n: u64) {
        if n == 0 {
            return;
        }
        let mut numerator = replace(&mut self.numerator, IntegerPolynomial::ZERO);
        numerator.nth_derivative_assign(n);
        let denominator = replace(&mut self.denominator, Natural::ONE);
        *self = Self::canonicalize(numerator, denominator);
    }
}
