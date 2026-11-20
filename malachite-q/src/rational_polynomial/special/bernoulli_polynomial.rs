// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational::number_theory::bernoulli_number::even_bernoulli_numbers;
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::number_theory::traits::Primorial;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

impl RationalPolynomial {
    /// Returns the Bernoulli polynomial $B_n(x)$.
    ///
    /// $$
    /// B_n(x) = \sum_{k=0}^n \binom{n}{k} B_k x^{n-k},
    /// $$
    /// where $B_k$ is the $k$th Bernoulli number (see
    /// [`Rational::bernoulli_number`](crate::Rational::bernoulli_number)), so that $B_n(0) = B_n$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(RationalPolynomial::bernoulli_polynomial(0).to_string(), "1");
    /// assert_eq!(
    ///     RationalPolynomial::bernoulli_polynomial(1).to_string(),
    ///     "x-1/2"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::bernoulli_polynomial(2).to_string(),
    ///     "x^2-x+1/6"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::bernoulli_polynomial(3).to_string(),
    ///     "x^3-3/2*x^2+1/2*x"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::bernoulli_polynomial(4).to_string(),
    ///     "x^4-2*x^3+x^2-1/30"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::bernoulli_polynomial(5).to_string(),
    ///     "x^5-5/2*x^4+5/3*x^3-1/6*x"
    /// );
    /// ```
    ///
    /// This is arith_bernoulli_polynomial from arith/bernoulli_polynomial.c, FLINT 3.6.0.
    pub fn bernoulli_polynomial(n: u64) -> Self {
        if n == 0 {
            return Self::one();
        }
        // The common denominator of B_0, ..., B_n
        let common_denominator = Natural::primorial(n + 2);
        let evens = even_bernoulli_numbers(usize::exact_from(n >> 1) + 1);
        // The coefficient of x^(n - k) is binomial(n, k) B_k, over the common denominator.
        let mut coefficients = Vec::with_capacity(usize::exact_from(n) + 1);
        for (j, b) in evens.into_iter().enumerate() {
            if j != 0 {
                coefficients.push(Integer::ZERO);
            }
            let negative = b < 0u32;
            let (numerator, denominator) = b.into_numerator_and_denominator();
            let x = Integer::from(numerator * (&common_denominator).div_exact(denominator));
            coefficients.push(if negative { -x } else { x });
        }
        // the odd indices above 1, including the last one when n is odd
        coefficients.resize(usize::exact_from(n) + 1, Integer::ZERO);
        // B_1 = -1/2, times binomial(n, 1) = n
        coefficients[1] = -Integer::from((&common_denominator >> 1u32) * Natural::from(n));
        // Multiply the even terms by the binomial coefficients.
        let mut t = Natural::ONE;
        for k in (2..=n).step_by(2) {
            t *= Natural::from((n - k + 1) * (n - k + 2));
            t.div_exact_assign(Natural::from(k * (k - 1)));
            coefficients[usize::exact_from(k)] *= Integer::from(&t);
        }
        coefficients.reverse();
        Self::from_numerator_and_denominator(
            IntegerPolynomial::from_coefficients_asc(coefficients),
            common_denominator,
        )
    }
}
