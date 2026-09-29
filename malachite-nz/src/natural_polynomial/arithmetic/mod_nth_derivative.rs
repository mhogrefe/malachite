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
    DivExactAssign, DivModPrecomputed, DivisibleBy, Factorial, ModIsReduced, ModMulAssign,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{ModNthDerivative, ModNthDerivativeAssign};

fn assert_reduced(p: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// Multiplies the coefficient of x^i, for i at least n, by the falling factorial i(i - 1)...(i - n +
// 1) modulo m and moves it to x^(i - n). The falling factorials are carried exactly, as in the
// non-modular derivative, since the division that carries one to the next need not be possible
// modulo m; each is reduced before it is used. They are all multiples of n!, so if m divides n!,
// the result is zero. The products can be zero, so the result is trimmed.
fn mod_nth_derivative_in_place(p: &mut NaturalPolynomial, n: u64, m: &Natural) {
    assert_reduced(p, m);
    if n == 0 {
        return;
    }
    if u64::exact_from(p.coefficients.len()) <= n {
        p.coefficients.clear();
        return;
    }
    let mut f = Natural::factorial(n);
    if (&f).divisible_by(m) {
        p.coefficients.clear();
        return;
    }
    let n = usize::exact_from(n);
    p.coefficients.drain(..n);
    let data = Natural::precompute_div_mod_data(m);
    for (j, c) in p.coefficients.iter_mut().enumerate() {
        if j != 0 {
            f.div_exact_assign(Natural::from(j));
            f *= Natural::from(j + n);
        }
        c.mod_mul_assign((&f).div_mod_precomputed(m, &data).1, m);
    }
    p.trim();
}

impl ModNthDerivative<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the $n$th derivative of a [`NaturalPolynomial`] modulo `m`, taking the polynomial
    /// by value and the modulus by value. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, n, m) = p^{(n)} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by the falling factorial $i^{\underline n} =
    /// i(i-1)\cdots(i-n+1)$, reduced, and moved to $x^{i-n}$. The products can be zero even when
    /// the coefficients are not, so the derivative can lose any number of degrees; if `m` divides
    /// $n!$, every falling factorial is a multiple of the modulus, and the result is zero.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the number of coefficients times
    /// `m.significant_bits()`, and $k$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModNthDerivative;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+3*x^3+2*x+4").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_nth_derivative(2, Natural::from(5u32))
    ///         .to_string(),
    ///     "2*x^2+3*x"
    /// );
    /// assert_eq!(
    ///     p.mod_nth_derivative(0, Natural::from(5u32)).to_string(),
    ///     "x^4+3*x^3+2*x+4"
    /// );
    ///
    /// // 6 divides 3!.
    /// let p = NaturalPolynomial::from_str("x^4+x^3").unwrap();
    /// assert_eq!(
    ///     p.mod_nth_derivative(3, Natural::from(6u32)).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly_nth_derivative`; this corresponds to `fmpz_poly_nth_derivative`
    /// from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, with each multiplier reduced modulo `m`.
    #[inline]
    fn mod_nth_derivative(mut self, n: u64, m: Natural) -> Self {
        mod_nth_derivative_in_place(&mut self, n, &m);
        self
    }
}

impl ModNthDerivative<&Natural> for NaturalPolynomial {
    type Output = Self;

    /// Computes the $n$th derivative of a [`NaturalPolynomial`] modulo `m`, taking the polynomial
    /// by value and the modulus by reference. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, n, m) = p^{(n)} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by the falling factorial $i^{\underline n} =
    /// i(i-1)\cdots(i-n+1)$, reduced, and moved to $x^{i-n}$. The products can be zero even when
    /// the coefficients are not, so the derivative can lose any number of degrees; if `m` divides
    /// $n!$, every falling factorial is a multiple of the modulus, and the result is zero.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the number of coefficients times
    /// `m.significant_bits()`, and $k$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModNthDerivative;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+3*x^3+2*x+4").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_nth_derivative(2, &Natural::from(5u32))
    ///         .to_string(),
    ///     "2*x^2+3*x"
    /// );
    /// assert_eq!(
    ///     p.mod_nth_derivative(0, &Natural::from(5u32)).to_string(),
    ///     "x^4+3*x^3+2*x+4"
    /// );
    ///
    /// // 6 divides 3!.
    /// let p = NaturalPolynomial::from_str("x^4+x^3").unwrap();
    /// assert_eq!(
    ///     p.mod_nth_derivative(3, &Natural::from(6u32)).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly_nth_derivative`; this corresponds to `fmpz_poly_nth_derivative`
    /// from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, with each multiplier reduced modulo `m`.
    #[inline]
    fn mod_nth_derivative(mut self, n: u64, m: &Natural) -> Self {
        mod_nth_derivative_in_place(&mut self, n, m);
        self
    }
}

impl ModNthDerivative<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the $n$th derivative of a [`NaturalPolynomial`] modulo `m`, taking the polynomial
    /// by reference and the modulus by value. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, n, m) = p^{(n)} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by the falling factorial $i^{\underline n} =
    /// i(i-1)\cdots(i-n+1)$, reduced, and moved to $x^{i-n}$. The products can be zero even when
    /// the coefficients are not, so the derivative can lose any number of degrees; if `m` divides
    /// $n!$, every falling factorial is a multiple of the modulus, and the result is zero.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the number of coefficients times
    /// `m.significant_bits()`, and $k$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModNthDerivative;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+3*x^3+2*x+4").unwrap();
    /// assert_eq!(
    ///     (&p).mod_nth_derivative(2, Natural::from(5u32)).to_string(),
    ///     "2*x^2+3*x"
    /// );
    /// assert_eq!(
    ///     (&p).mod_nth_derivative(0, Natural::from(5u32)).to_string(),
    ///     "x^4+3*x^3+2*x+4"
    /// );
    ///
    /// // 6 divides 3!.
    /// let p = NaturalPolynomial::from_str("x^4+x^3").unwrap();
    /// assert_eq!(
    ///     (&p).mod_nth_derivative(3, Natural::from(6u32)).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly_nth_derivative`; this corresponds to `fmpz_poly_nth_derivative`
    /// from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, with each multiplier reduced modulo `m`.
    #[inline]
    fn mod_nth_derivative(self, n: u64, m: Natural) -> NaturalPolynomial {
        let mut p = self.clone();
        mod_nth_derivative_in_place(&mut p, n, &m);
        p
    }
}

impl ModNthDerivative<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the $n$th derivative of a [`NaturalPolynomial`] modulo `m`, taking the polynomial
    /// by reference and the modulus by reference. The coefficients must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, n, m) = p^{(n)} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by the falling factorial $i^{\underline n} =
    /// i(i-1)\cdots(i-n+1)$, reduced, and moved to $x^{i-n}$. The products can be zero even when
    /// the coefficients are not, so the derivative can lose any number of degrees; if `m` divides
    /// $n!$, every falling factorial is a multiple of the modulus, and the result is zero.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the number of coefficients times
    /// `m.significant_bits()`, and $k$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModNthDerivative;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+3*x^3+2*x+4").unwrap();
    /// assert_eq!(
    ///     (&p).mod_nth_derivative(2, &Natural::from(5u32)).to_string(),
    ///     "2*x^2+3*x"
    /// );
    /// assert_eq!(
    ///     (&p).mod_nth_derivative(0, &Natural::from(5u32)).to_string(),
    ///     "x^4+3*x^3+2*x+4"
    /// );
    ///
    /// // 6 divides 3!.
    /// let p = NaturalPolynomial::from_str("x^4+x^3").unwrap();
    /// assert_eq!(
    ///     (&p).mod_nth_derivative(3, &Natural::from(6u32)).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly_nth_derivative`; this corresponds to `fmpz_poly_nth_derivative`
    /// from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, with each multiplier reduced modulo `m`.
    #[inline]
    fn mod_nth_derivative(self, n: u64, m: &Natural) -> NaturalPolynomial {
        let mut p = self.clone();
        mod_nth_derivative_in_place(&mut p, n, m);
        p
    }
}

impl ModNthDerivativeAssign<Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its $n$th derivative modulo `m`, in place, taking the
    /// modulus by value. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p^{(n)} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by the falling factorial $i^{\underline n} =
    /// i(i-1)\cdots(i-n+1)$, reduced, and moved to $x^{i-n}$. The products can be zero even when
    /// the coefficients are not, so the derivative can lose any number of degrees; if `m` divides
    /// $n!$, every falling factorial is a multiple of the modulus, and the result is zero.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the number of coefficients times
    /// `m.significant_bits()`, and $k$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModNthDerivativeAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+3*x^3+2*x+4").unwrap();
    /// p.mod_nth_derivative_assign(2, Natural::from(5u32));
    /// assert_eq!(p.to_string(), "2*x^2+3*x");
    ///
    /// // 6 divides 3!.
    /// let mut p = NaturalPolynomial::from_str("x^4+x^3").unwrap();
    /// p.mod_nth_derivative_assign(3, Natural::from(6u32));
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly_nth_derivative`; this corresponds to `fmpz_poly_nth_derivative`
    /// from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, with each multiplier reduced modulo `m`.
    #[inline]
    fn mod_nth_derivative_assign(&mut self, n: u64, m: Natural) {
        mod_nth_derivative_in_place(self, n, &m);
    }
}

impl ModNthDerivativeAssign<&Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its $n$th derivative modulo `m`, in place, taking the
    /// modulus by reference. The coefficients must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p^{(n)} \bmod m.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by the falling factorial $i^{\underline n} =
    /// i(i-1)\cdots(i-n+1)$, reduced, and moved to $x^{i-n}$. The products can be zero even when
    /// the coefficients are not, so the derivative can lose any number of degrees; if `m` divides
    /// $n!$, every falling factorial is a multiple of the modulus, and the result is zero.
    ///
    /// # Worst-case complexity
    /// $T(b, k) = O(k(b + k \log k))$
    ///
    /// $M(b, k) = O(b + k^2 \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $b$ is the number of coefficients times
    /// `m.significant_bits()`, and $k$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModNthDerivativeAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+3*x^3+2*x+4").unwrap();
    /// p.mod_nth_derivative_assign(2, &Natural::from(5u32));
    /// assert_eq!(p.to_string(), "2*x^2+3*x");
    ///
    /// // 6 divides 3!.
    /// let mut p = NaturalPolynomial::from_str("x^4+x^3").unwrap();
    /// p.mod_nth_derivative_assign(3, &Natural::from(6u32));
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// FLINT has no `fmpz_mod_poly_nth_derivative`; this corresponds to `fmpz_poly_nth_derivative`
    /// from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, with each multiplier reduced modulo `m`.
    #[inline]
    fn mod_nth_derivative_assign(&mut self, n: u64, m: &Natural) {
        mod_nth_derivative_in_place(self, n, m);
    }
}
