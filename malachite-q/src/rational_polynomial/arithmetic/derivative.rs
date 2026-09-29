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
use malachite_base::polynomial::{Derivative, DerivativeAssign};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

impl Derivative for RationalPolynomial {
    type Output = Self;

    /// Computes the derivative of a [`RationalPolynomial`], taking it by value.
    ///
    /// $$
    /// f(p) = p' = \sum_{i=1}^n ia_ix^{i-1}.
    /// $$
    ///
    /// The numerator is differentiated, and since the multipliers $i$ can share factors with the
    /// denominator, the result is then brought back to lowest terms. A constant polynomial,
    /// including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// numerator's coefficients and the denominator, and $m$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Derivative;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1").unwrap();
    /// assert_eq!(p.derivative().to_string(), "x+1/3");
    ///
    /// // The derivative can share a factor with the denominator, which is then cancelled.
    /// let p = RationalPolynomial::from_str("1/2*x^2+1/2").unwrap();
    /// assert_eq!(p.derivative().to_string(), "x");
    ///
    /// let p = RationalPolynomial::from_str("7/3").unwrap();
    /// assert_eq!(p.derivative(), RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_derivative` from `fmpq_poly/derivative.c`, FLINT 3.6.0.
    #[inline]
    fn derivative(mut self) -> Self {
        self.derivative_assign();
        self
    }
}

impl Derivative for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Computes the derivative of a [`RationalPolynomial`], taking it by reference.
    ///
    /// $$
    /// f(p) = p' = \sum_{i=1}^n ia_ix^{i-1}.
    /// $$
    ///
    /// The numerator is differentiated, and since the multipliers $i$ can share factors with the
    /// denominator, the result is then brought back to lowest terms. A constant polynomial,
    /// including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// numerator's coefficients and the denominator, and $m$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Derivative;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1").unwrap();
    /// assert_eq!((&p).derivative().to_string(), "x+1/3");
    ///
    /// // The derivative can share a factor with the denominator, which is then cancelled.
    /// let p = RationalPolynomial::from_str("1/2*x^2+1/2").unwrap();
    /// assert_eq!((&p).derivative().to_string(), "x");
    ///
    /// let p = RationalPolynomial::from_str("7/3").unwrap();
    /// assert_eq!((&p).derivative(), RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_derivative` from `fmpq_poly/derivative.c`, FLINT 3.6.0.
    #[inline]
    fn derivative(self) -> RationalPolynomial {
        RationalPolynomial::canonicalize((&self.numerator).derivative(), self.denominator.clone())
    }
}

impl DerivativeAssign for RationalPolynomial {
    /// Replaces a [`RationalPolynomial`] with its derivative.
    ///
    /// $$
    /// p \gets p' = \sum_{i=1}^n ia_ix^{i-1}.
    /// $$
    ///
    /// The numerator is differentiated, and since the multipliers $i$ can share factors with the
    /// denominator, the result is then brought back to lowest terms. A constant polynomial,
    /// including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// numerator's coefficients and the denominator, and $m$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::DerivativeAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1").unwrap();
    /// p.derivative_assign();
    /// assert_eq!(p.to_string(), "x+1/3");
    ///
    /// // The derivative can share a factor with the denominator, which is then cancelled.
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/2").unwrap();
    /// p.derivative_assign();
    /// assert_eq!(p.to_string(), "x");
    ///
    /// let mut p = RationalPolynomial::from_str("7/3").unwrap();
    /// p.derivative_assign();
    /// assert_eq!(p, RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_derivative` from `fmpq_poly/derivative.c`, FLINT 3.6.0.
    fn derivative_assign(&mut self) {
        let mut numerator = replace(&mut self.numerator, IntegerPolynomial::ZERO);
        numerator.derivative_assign();
        let denominator = replace(&mut self.denominator, Natural::ONE);
        *self = Self::canonicalize(numerator, denominator);
    }
}
