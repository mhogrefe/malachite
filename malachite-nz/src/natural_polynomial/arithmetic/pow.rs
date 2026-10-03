// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::pow::{pow_assign_vec, pow_ref};
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{Pow, PowAssign};

impl Pow<u64> for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power, taking it by value.
    ///
    /// $$
    /// f(p, e) = p^e.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1. Depending on the length of the
    /// polynomial, the size of its coefficients, and the exponent, the power is computed by the
    /// binomial theorem, by J. C. P. Miller's recurrence for the coefficients of a power, by an
    /// addition chain, or by repeated squaring.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, and $m$ is `exp` times the largest number of significant bits of any of its
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x+1")
    ///         .unwrap()
    ///         .pow(3)
    ///         .to_string(),
    ///     "x^3+3*x^2+3*x+1"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("2*x+1")
    ///         .unwrap()
    ///         .pow(4)
    ///         .to_string(),
    ///     "16*x^4+32*x^3+24*x^2+8*x+1"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+x")
    ///         .unwrap()
    ///         .pow(0)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pow` from `fmpz_poly/pow.c`, FLINT 3.6.0, except that a
    /// factor of $x^k$ is removed before powering, and that the algorithm is chosen by measured
    /// criteria, which include addition chains.
    #[inline]
    fn pow(mut self, exp: u64) -> Self {
        self.pow_assign(exp);
        self
    }
}

impl Pow<u64> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power, taking it by reference.
    ///
    /// $$
    /// f(p, e) = p^e.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1. Depending on the length of the
    /// polynomial, the size of its coefficients, and the exponent, the power is computed by the
    /// binomial theorem, by J. C. P. Miller's recurrence for the coefficients of a power, by an
    /// addition chain, or by repeated squaring.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, and $m$ is `exp` times the largest number of significant bits of any of its
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+1").unwrap())
    ///         .pow(3)
    ///         .to_string(),
    ///     "x^3+3*x^2+3*x+1"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("2*x+1").unwrap())
    ///         .pow(4)
    ///         .to_string(),
    ///     "16*x^4+32*x^3+24*x^2+8*x+1"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+x").unwrap())
    ///         .pow(0)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pow` from `fmpz_poly/pow.c`, FLINT 3.6.0, except that a
    /// factor of $x^k$ is removed before powering, and that the algorithm is chosen by measured
    /// criteria, which include addition chains.
    #[inline]
    fn pow(self, exp: u64) -> NaturalPolynomial {
        NaturalPolynomial {
            coefficients: pow_ref(&self.coefficients, exp),
        }
    }
}

impl PowAssign<u64> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power in place.
    ///
    /// $$
    /// p \gets p^e.
    /// $$
    ///
    /// The zeroth power of every polynomial, including 0, is 1. Depending on the length of the
    /// polynomial, the size of its coefficients, and the exponent, the power is computed by the
    /// binomial theorem, by J. C. P. Miller's recurrence for the coefficients of a power, by an
    /// addition chain, or by repeated squaring.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `exp` times the length of the
    /// polynomial, and $m$ is `exp` times the largest number of significant bits of any of its
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    /// p.pow_assign(3);
    /// assert_eq!(p.to_string(), "x^3+3*x^2+3*x+1");
    ///
    /// let mut p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// p.pow_assign(4);
    /// assert_eq!(p.to_string(), "16*x^4+32*x^3+24*x^2+8*x+1");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+x").unwrap();
    /// p.pow_assign(0);
    /// assert_eq!(p.to_string(), "1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pow` from `fmpz_poly/pow.c`, FLINT 3.6.0, except that a
    /// factor of $x^k$ is removed before powering, and that the algorithm is chosen by measured
    /// criteria, which include addition chains.
    #[inline]
    fn pow_assign(&mut self, exp: u64) {
        pow_assign_vec(&mut self.coefficients, exp);
    }
}
