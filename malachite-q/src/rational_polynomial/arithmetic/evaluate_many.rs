// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec::Vec;
use malachite_base::polynomial::arithmetic::traits::{Evaluate, EvaluateMany};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;

impl EvaluateMany<Rational> for &IntegerPolynomial {
    type Output = Rational;

    /// Evaluates an [`IntegerPolynomial`] at each of several [`Rational`]s.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. Each value is a
    /// [`Rational`] in lowest terms, found as by
    /// [`evaluate`](malachite_base::polynomial::arithmetic::traits::Evaluate::evaluate).
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(kn \log^2 n \log\log n)$
    ///
    /// $M(n, k) = O(kn \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `xs.len()`, and $n$ is `self.len()`
    /// times the larger of the greatest number of bits of any coefficient and the number of bits of
    /// the numerator or denominator of any value in `xs`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{NegativeOne, OneHalf};
    /// use malachite_base::polynomial::arithmetic::traits::EvaluateMany;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_q::Rational;
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// let xs = [Rational::ONE_HALF, Rational::NEGATIVE_ONE, Rational::from_signeds(3, 2)];
    /// assert_eq!(
    ///     (&p).evaluate_many(&xs),
    ///     [Rational::from_signeds(3, 4), Rational::from(6), Rational::from_signeds(-1, 4)]
    /// );
    /// ```
    #[inline]
    fn evaluate_many(self, xs: &[Rational]) -> Vec<Rational> {
        xs.iter().map(|x| self.evaluate(x)).collect()
    }
}

impl EvaluateMany<Rational> for &RationalPolynomial {
    type Output = Rational;

    /// Evaluates a [`RationalPolynomial`] at each of several [`Rational`]s.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. Each value is a
    /// [`Rational`] in lowest terms, found as by
    /// [`evaluate`](malachite_base::polynomial::arithmetic::traits::Evaluate::evaluate).
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(kn \log^2 n \log\log n)$
    ///
    /// $M(n, k) = O(kn \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `xs.len()`, and $n$ is `self.len()`
    /// times the larger of the greatest number of bits of any coefficient's numerator or
    /// denominator and the number of bits of the numerator or denominator of any value in `xs`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::arithmetic::traits::EvaluateMany;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2+1/3").unwrap();
    /// let xs = [Rational::ZERO, Rational::ONE, Rational::from_signeds(-2, 3)];
    /// assert_eq!(
    ///     (&p).evaluate_many(&xs),
    ///     [
    ///         Rational::from_signeds(1, 3),
    ///         Rational::from_signeds(5, 6),
    ///         Rational::from_signeds(5, 9)
    ///     ]
    /// );
    /// ```
    #[inline]
    fn evaluate_many(self, xs: &[Rational]) -> Vec<Rational> {
        xs.iter().map(|x| self.evaluate(x)).collect()
    }
}

impl EvaluateMany<Integer> for &RationalPolynomial {
    type Output = Rational;

    /// Evaluates a [`RationalPolynomial`] at each of several [`Integer`]s.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. Each value is a
    /// [`Rational`] in lowest terms, found as by
    /// [`evaluate`](malachite_base::polynomial::arithmetic::traits::Evaluate::evaluate).
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(kn \log^2 n \log\log n)$
    ///
    /// $M(n, k) = O(kn \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `xs.len()`, and $n$ is `self.len()`
    /// times the larger of the greatest number of bits of any coefficient's numerator or
    /// denominator and the number of bits of any value in `xs`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::EvaluateMany;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2+1/3").unwrap();
    /// let xs = [0i32, 1, 2].map(Integer::from);
    /// assert_eq!(
    ///     (&p).evaluate_many(&xs),
    ///     [
    ///         Rational::from_signeds(1, 3),
    ///         Rational::from_signeds(5, 6),
    ///         Rational::from_signeds(7, 3)
    ///     ]
    /// );
    /// ```
    #[inline]
    fn evaluate_many(self, xs: &[Integer]) -> Vec<Rational> {
        xs.iter().map(|x| self.evaluate(x)).collect()
    }
}
