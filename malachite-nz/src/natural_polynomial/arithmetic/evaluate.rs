// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::evaluate::evaluate;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::polynomial::Evaluate;

impl Evaluate<&Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], taking both by reference.
    ///
    /// $$
    /// f(p, x) = \sum_{i=0}^{n-1} c_i x^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used unless the polynomial is long compared with the size of `x`, in which
    /// case divide and conquer, which pairs off coefficients and merges the pairs so that each
    /// multiplication has operands of about the same size, is faster.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()` times the larger of the
    /// greatest number of bits of any coefficient and the number of bits of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_base::polynomial::Evaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!((&p).evaluate(&Natural::ZERO), 2);
    /// assert_eq!((&p).evaluate(&Natural::ONE), 6);
    /// assert_eq!((&p).evaluate(&Natural::from(10u32)), 132);
    ///
    /// let q = NaturalPolynomial::from_str("x^100+1").unwrap();
    /// assert_eq!(
    ///     (&q).evaluate(&Natural::TWO).to_string(),
    ///     "1267650600228229401496703205377"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_evaluate_fmpz` from `fmpz_poly/evaluate_fmpz.c`, FLINT
    /// 3.6.0, for a polynomial whose coefficients are all nonnegative, evaluated at a nonnegative
    /// value.
    #[inline]
    fn evaluate(self, x: &Natural) -> Natural {
        evaluate(&self.coefficients, x)
    }
}

impl Evaluate<Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at a [`Natural`], taking the polynomial by reference and
    /// the value by value.
    ///
    /// $$
    /// f(p, x) = \sum_{i=0}^{n-1} c_i x^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Horner's rule is used unless the polynomial is long compared with the size of `x`, in which
    /// case divide and conquer, which pairs off coefficients and merges the pairs so that each
    /// multiplication has operands of about the same size, is faster.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()` times the larger of the
    /// greatest number of bits of any coefficient and the number of bits of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_base::polynomial::Evaluate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!((&p).evaluate(Natural::ZERO), 2);
    /// assert_eq!((&p).evaluate(Natural::ONE), 6);
    /// assert_eq!((&p).evaluate(Natural::from(10u32)), 132);
    ///
    /// let q = NaturalPolynomial::from_str("x^100+1").unwrap();
    /// assert_eq!(
    ///     (&q).evaluate(Natural::TWO).to_string(),
    ///     "1267650600228229401496703205377"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_evaluate_fmpz` from `fmpz_poly/evaluate_fmpz.c`, FLINT
    /// 3.6.0, for a polynomial whose coefficients are all nonnegative, evaluated at a nonnegative
    /// value.
    #[inline]
    fn evaluate(self, x: Natural) -> Natural {
        evaluate(&self.coefficients, &x)
    }
}
