// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::polynomial::{DeflatePowerOfX, DeflatePowerOfXAssign};

impl DeflatePowerOfX for RationalPolynomial {
    type Output = Self;

    /// Deflates a [`RationalPolynomial`] by $n$, taking it by value, giving the polynomial $q$ with
    /// $q(x^n) = p(x)$. The coefficient of $x^{in}$ moves to $x^i$.
    ///
    /// $$
    /// f(p, n) = q, \quad \text{where} \quad q(x^n) = p(x).
    /// $$
    ///
    /// Only the numerator changes, so the result stays in lowest terms. A constant polynomial
    /// deflates to itself, and deflating by 1 changes nothing.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m)$
    ///
    /// $M(m) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `n` is 0, or if the polynomial has a nonzero coefficient at an exponent that is
    /// not a multiple of `n`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DeflatePowerOfX;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^4-1/3").unwrap();
    /// assert_eq!(p.deflate_power_of_x(2).to_string(), "1/2*x^2-1/3");
    /// ```
    ///
    /// FLINT has no `fmpq_poly_deflate`; this corresponds to `fmpz_poly_deflate` from
    /// `fmpz_poly/deflate.c`, FLINT 3.6.0, applied to the numerator, except that it panics rather
    /// than dropping the coefficients at other exponents.
    #[inline]
    fn deflate_power_of_x(mut self, n: u64) -> Self {
        self.deflate_power_of_x_assign(n);
        self
    }
}

impl DeflatePowerOfX for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Deflates a [`RationalPolynomial`] by $n$, taking it by reference, giving the polynomial $q$
    /// with $q(x^n) = p(x)$. The coefficient of $x^{in}$ moves to $x^i$.
    ///
    /// $$
    /// f(p, n) = q, \quad \text{where} \quad q(x^n) = p(x).
    /// $$
    ///
    /// Only the numerator changes, so the result stays in lowest terms. A constant polynomial
    /// deflates to itself, and deflating by 1 changes nothing.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the total number of bits of the
    /// numerator's coefficients and the denominator.
    ///
    /// # Panics
    /// Panics if `n` is 0, or if the polynomial has a nonzero coefficient at an exponent that is
    /// not a multiple of `n`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DeflatePowerOfX;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^4-1/3").unwrap();
    /// assert_eq!((&p).deflate_power_of_x(2).to_string(), "1/2*x^2-1/3");
    /// ```
    ///
    /// FLINT has no `fmpq_poly_deflate`; this corresponds to `fmpz_poly_deflate` from
    /// `fmpz_poly/deflate.c`, FLINT 3.6.0, applied to the numerator, except that it panics rather
    /// than dropping the coefficients at other exponents.
    fn deflate_power_of_x(self, n: u64) -> RationalPolynomial {
        RationalPolynomial {
            numerator: (&self.numerator).deflate_power_of_x(n),
            denominator: self.denominator.clone(),
        }
    }
}

impl DeflatePowerOfXAssign for RationalPolynomial {
    /// Deflates a [`RationalPolynomial`] by $n$ in place, replacing $p$ with the polynomial $q$
    /// such that $q(x^n) = p(x)$. The coefficient of $x^{in}$ moves to $x^i$.
    ///
    /// $$
    /// p \gets q, \quad \text{where} \quad q(x^n) = p(x).
    /// $$
    ///
    /// Only the numerator changes, so the result stays in lowest terms. A constant polynomial
    /// deflates to itself, and deflating by 1 changes nothing.
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m)$
    ///
    /// $M(m) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `n` is 0, or if the polynomial has a nonzero coefficient at an exponent that is
    /// not a multiple of `n`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::DeflatePowerOfXAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^4-1/3").unwrap();
    /// p.deflate_power_of_x_assign(2);
    /// assert_eq!(p.to_string(), "1/2*x^2-1/3");
    /// ```
    ///
    /// FLINT has no `fmpq_poly_deflate`; this corresponds to `fmpz_poly_deflate` from
    /// `fmpz_poly/deflate.c`, FLINT 3.6.0, applied to the numerator, except that it panics rather
    /// than dropping the coefficients at other exponents.
    #[inline]
    fn deflate_power_of_x_assign(&mut self, n: u64) {
        self.numerator.deflate_power_of_x_assign(n);
    }
}
