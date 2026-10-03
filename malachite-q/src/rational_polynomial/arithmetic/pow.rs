// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::{Pow, PowAssign};

impl Pow<u64> for RationalPolynomial {
    type Output = Self;

    /// Raises a [`RationalPolynomial`] to a power, taking it by value.
    ///
    /// $$
    /// f(p, e) = p^e.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1. No GCD is needed: the numerator and
    /// the denominator are raised to the power separately, and by Gauss's lemma the content of the
    /// numerator's power is the power of its content, which stays coprime to the power of the
    /// denominator.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, and $m$ is `exp` times the largest number of significant bits of any numerator
    /// coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x+1/3").unwrap())
    ///         .pow(3)
    ///         .to_string(),
    ///     "1/8*x^3+1/4*x^2+1/6*x+1/27"
    /// );
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x-1").unwrap())
    ///         .pow(4)
    ///         .to_string(),
    ///     "1/16*x^4-1/2*x^3+3/2*x^2-2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_pow` from `fmpq_poly/pow.c`, FLINT 3.6.0, except that the
    /// numerator is raised to the power as in [`Pow`] for
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial).
    #[inline]
    fn pow(mut self, exp: u64) -> Self {
        self.pow_assign(exp);
        self
    }
}

impl Pow<u64> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Raises a [`RationalPolynomial`] to a power, taking it by reference.
    ///
    /// $$
    /// f(p, e) = p^e.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1. No GCD is needed: the numerator and
    /// the denominator are raised to the power separately, and by Gauss's lemma the content of the
    /// numerator's power is the power of its content, which stays coprime to the power of the
    /// denominator.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, and $m$ is `exp` times the largest number of significant bits of any numerator
    /// coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x+1/3").unwrap())
    ///         .pow(3)
    ///         .to_string(),
    ///     "1/8*x^3+1/4*x^2+1/6*x+1/27"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x-1").unwrap())
    ///         .pow(4)
    ///         .to_string(),
    ///     "1/16*x^4-1/2*x^3+3/2*x^2-2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_pow` from `fmpq_poly/pow.c`, FLINT 3.6.0, except that the
    /// numerator is raised to the power as in [`Pow`] for
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial).
    #[inline]
    fn pow(self, exp: u64) -> RationalPolynomial {
        RationalPolynomial {
            numerator: (&self.numerator).pow(exp),
            denominator: (&self.denominator).pow(exp),
        }
    }
}

impl PowAssign<u64> for RationalPolynomial {
    /// Raises a [`RationalPolynomial`] to a power in place.
    ///
    /// $$
    /// p \gets p^e.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1. No GCD is needed: the numerator and
    /// the denominator are raised to the power separately, and by Gauss's lemma the content of the
    /// numerator's power is the power of its content, which stays coprime to the power of the
    /// denominator.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, and $m$ is `exp` times the largest number of significant bits of any numerator
    /// coefficient or of the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// p.pow_assign(3);
    /// assert_eq!(p.to_string(), "1/8*x^3+1/4*x^2+1/6*x+1/27");
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x-1").unwrap();
    /// p.pow_assign(4);
    /// assert_eq!(p.to_string(), "1/16*x^4-1/2*x^3+3/2*x^2-2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_pow` from `fmpq_poly/pow.c`, FLINT 3.6.0, except that the
    /// numerator is raised to the power as in [`Pow`] for
    /// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial).
    #[inline]
    fn pow_assign(&mut self, exp: u64) {
        self.numerator.pow_assign(exp);
        self.denominator.pow_assign(exp);
    }
}
