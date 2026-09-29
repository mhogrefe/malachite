// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{
    DivExactAssign, Factorial, ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2MulAssign,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{ModPowerOf2NthDerivative, ModPowerOf2NthDerivativeAssign};

fn assert_reduced(p: &NaturalPolynomial, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// Multiplies the coefficient of x^i, for i at least n, by the falling factorial i(i - 1)...(i - n +
// 1) modulo 2^pow and moves it to x^(i - n). The falling factorials are carried exactly, as in the
// non-modular derivative, and each is reduced before it is used. They are all multiples of n!, so
// if 2^pow divides n!, which happens when n - n.count_ones() is at least pow, the result is zero.
// The products can be zero, so the result is trimmed.
fn mod_power_of_2_nth_derivative_in_place(p: &mut NaturalPolynomial, n: u64, pow: u64) {
    assert_reduced(p, pow);
    if n == 0 {
        return;
    }
    if u64::exact_from(p.coefficients.len()) <= n || n - u64::from(n.count_ones()) >= pow {
        p.coefficients.clear();
        return;
    }
    let mut f = Natural::factorial(n);
    let n = usize::exact_from(n);
    p.coefficients.drain(..n);
    for (j, c) in p.coefficients.iter_mut().enumerate() {
        if j != 0 {
            f.div_exact_assign(Natural::from(j));
            f *= Natural::from(j + n);
        }
        c.mod_power_of_2_mul_assign((&f).mod_power_of_2(pow), pow);
    }
    p.trim();
}

impl ModPowerOf2NthDerivative for NaturalPolynomial {
    type Output = Self;

    /// Computes the $n$th derivative of a [`NaturalPolynomial`] modulo $2^k$, taking the polynomial
    /// by value. The coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, n, k) = p^{(n)} \bmod 2^k.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by the falling factorial $i^{\underline n} =
    /// i(i-1)\cdots(i-n+1)$, reduced, and moved to $x^{i-n}$. The products can be zero even when
    /// the coefficients are not, so the derivative can lose any number of degrees; if $2^k$ divides
    /// $n!$, every falling factorial is a multiple of the modulus, and the result is zero.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the number of coefficients times `pow`,
    /// and $k$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2NthDerivative;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+3*x^3+2*x+5").unwrap();
    /// assert_eq!(
    ///     p.clone().mod_power_of_2_nth_derivative(2, 3).to_string(),
    ///     "4*x^2+2*x"
    /// );
    /// assert_eq!(
    ///     p.mod_power_of_2_nth_derivative(0, 3).to_string(),
    ///     "x^4+3*x^3+2*x+5"
    /// );
    ///
    /// // 2^3 divides 4!.
    /// let p = NaturalPolynomial::from_str("x^5+x^4").unwrap();
    /// assert_eq!(p.mod_power_of_2_nth_derivative(4, 3).to_string(), "0");
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly_nth_derivative`; this corresponds to `fmpz_poly_nth_derivative`
    /// from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, with each multiplier reduced modulo $2^k$.
    #[inline]
    fn mod_power_of_2_nth_derivative(mut self, n: u64, pow: u64) -> Self {
        mod_power_of_2_nth_derivative_in_place(&mut self, n, pow);
        self
    }
}

impl ModPowerOf2NthDerivative for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the $n$th derivative of a [`NaturalPolynomial`] modulo $2^k$, taking the polynomial
    /// by reference. The coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, n, k) = p^{(n)} \bmod 2^k.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by the falling factorial $i^{\underline n} =
    /// i(i-1)\cdots(i-n+1)$, reduced, and moved to $x^{i-n}$. The products can be zero even when
    /// the coefficients are not, so the derivative can lose any number of degrees; if $2^k$ divides
    /// $n!$, every falling factorial is a multiple of the modulus, and the result is zero.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the number of coefficients times `pow`,
    /// and $k$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2NthDerivative;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+3*x^3+2*x+5").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_nth_derivative(2, 3).to_string(),
    ///     "4*x^2+2*x"
    /// );
    /// assert_eq!(
    ///     (&p).mod_power_of_2_nth_derivative(0, 3).to_string(),
    ///     "x^4+3*x^3+2*x+5"
    /// );
    ///
    /// // 2^3 divides 4!.
    /// let p = NaturalPolynomial::from_str("x^5+x^4").unwrap();
    /// assert_eq!((&p).mod_power_of_2_nth_derivative(4, 3).to_string(), "0");
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly_nth_derivative`; this corresponds to `fmpz_poly_nth_derivative`
    /// from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, with each multiplier reduced modulo $2^k$.
    #[inline]
    fn mod_power_of_2_nth_derivative(self, n: u64, pow: u64) -> NaturalPolynomial {
        let mut p = self.clone();
        mod_power_of_2_nth_derivative_in_place(&mut p, n, pow);
        p
    }
}

impl ModPowerOf2NthDerivativeAssign for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its $n$th derivative modulo $2^k$, in place. The
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets p^{(n)} \bmod 2^k.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by the falling factorial $i^{\underline n} =
    /// i(i-1)\cdots(i-n+1)$, reduced, and moved to $x^{i-n}$. The products can be zero even when
    /// the coefficients are not, so the derivative can lose any number of degrees; if $2^k$ divides
    /// $n!$, every falling factorial is a multiple of the modulus, and the result is zero.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the number of coefficients times `pow`,
    /// and $k$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2NthDerivativeAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+3*x^3+2*x+5").unwrap();
    /// p.mod_power_of_2_nth_derivative_assign(2, 3);
    /// assert_eq!(p.to_string(), "4*x^2+2*x");
    ///
    /// // 2^3 divides 4!.
    /// let mut p = NaturalPolynomial::from_str("x^5+x^4").unwrap();
    /// p.mod_power_of_2_nth_derivative_assign(4, 3);
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly_nth_derivative`; this corresponds to `fmpz_poly_nth_derivative`
    /// from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, with each multiplier reduced modulo $2^k$.
    #[inline]
    fn mod_power_of_2_nth_derivative_assign(&mut self, n: u64, pow: u64) {
        mod_power_of_2_nth_derivative_in_place(self, n, pow);
    }
}
