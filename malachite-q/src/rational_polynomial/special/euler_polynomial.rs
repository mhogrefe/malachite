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

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::basic::traits::{Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

impl RationalPolynomial {
    /// Returns the Euler polynomial $E_n(x)$.
    ///
    /// The Euler polynomials are defined by the generating function
    /// $$
    /// \frac{2 e^{xt}}{e^t + 1} = \sum_{n=0}^\infty E_n(x) \frac{t^n}{n!}.
    /// $$
    /// They are related to the Euler numbers $E_n$ (see
    /// [`Integer::euler_number`](malachite_nz::integer::Integer::euler_number)) by $E_n = 2^n
    /// E_n(1/2)$, and to the Bernoulli polynomials by
    /// $$
    /// E_n(x) = \frac{2}{n+1}\left(B_{n+1}(x) - 2^{n+1} B_{n+1}\left(\frac{x}{2}\right)\right).
    /// $$
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
    /// assert_eq!(RationalPolynomial::euler_polynomial(0).to_string(), "1");
    /// assert_eq!(RationalPolynomial::euler_polynomial(1).to_string(), "x-1/2");
    /// assert_eq!(RationalPolynomial::euler_polynomial(2).to_string(), "x^2-x");
    /// assert_eq!(
    ///     RationalPolynomial::euler_polynomial(3).to_string(),
    ///     "x^3-3/2*x^2+1/4"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::euler_polynomial(4).to_string(),
    ///     "x^4-2*x^3+x"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::euler_polynomial(5).to_string(),
    ///     "x^5-5/2*x^4+5/2*x^2-1/2"
    /// );
    /// ```
    ///
    /// This is arith_euler_polynomial from arith/euler_polynomial.c, FLINT 3.6.0.
    pub fn euler_polynomial(n: u64) -> Self {
        if n == 0 {
            return Self::one();
        }
        let (numerator, denominator) =
            Self::bernoulli_polynomial(n + 1).into_numerator_and_denominator();
        let mut coefficients = numerator.into_coefficients_asc();
        // The coefficient of x^k is multiplied by 2 - 2^(n + 1 - k).
        let mut t = -Integer::TWO;
        for k in (0..=usize::exact_from(n)).rev() {
            coefficients[k] *= &t;
            t <<= 1u32;
            t -= Integer::TWO;
        }
        coefficients[usize::exact_from(n) + 1] = Integer::ZERO;
        Self::from_numerator_and_denominator(
            IntegerPolynomial::from_coefficients_asc(coefficients),
            denominator * Natural::from(n + 1),
        )
    }
}
