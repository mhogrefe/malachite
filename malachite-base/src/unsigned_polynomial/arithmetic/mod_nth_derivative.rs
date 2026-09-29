// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::polynomial::{ModNthDerivative, ModNthDerivativeAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// Adds 1 to a counter modulo m.
fn increment_mod<T: PrimitiveUnsigned>(i: &mut T, m: T) {
    *i += T::ONE;
    if *i == m {
        *i = T::ZERO;
    }
}

// Multiplies the coefficient of x^i, for i at least n, by the falling factorial i(i - 1)...(i - n +
// 1) modulo m and moves it to x^(i - n). The division that carries one falling factorial to the
// next need not be possible modulo m, so each is multiplied out from its n factors, which are
// counted down from i mod m. They are all multiples of n!, so n! mod m is computed first, and if it
// is zero, so is the result. The products can be zero, so the result is trimmed.
fn mod_nth_derivative_in_place<T: PrimitiveUnsigned>(p: &mut UnsignedPolynomial<T>, n: u64, m: T) {
    assert_reduced(p, m);
    if n == 0 {
        return;
    }
    if u64::exact_from(p.coefficients.len()) <= n {
        p.coefficients.clear();
        return;
    }
    // After this loop, `top` is n mod m.
    let mut top = T::ZERO;
    let mut factorial = T::ONE;
    for _ in 0..n {
        increment_mod(&mut top, m);
        factorial.mod_mul_assign(top, m);
    }
    if factorial == T::ZERO {
        p.coefficients.clear();
        return;
    }
    p.coefficients.drain(..usize::exact_from(n));
    for c in &mut p.coefficients {
        let mut falling = T::ONE;
        let mut factor = top;
        for _ in 0..n {
            falling.mod_mul_assign(factor, m);
            factor = if factor == T::ZERO {
                m - T::ONE
            } else {
                factor - T::ONE
            };
        }
        c.mod_mul_assign(falling, m);
        increment_mod(&mut top, m);
    }
    p.trim();
}

impl<T: PrimitiveUnsigned> ModNthDerivative<T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the $n$th derivative of an [`UnsignedPolynomial`] modulo `m`, taking the polynomial
    /// by value. The coefficients must already be reduced modulo `m`.
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
    /// $T(k, n) = O(kn)$
    ///
    /// $M(k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `self.len()`, and $n$ is `n`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModNthDerivative;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+3*x^3+2*x+4").unwrap();
    /// assert_eq!(p.clone().mod_nth_derivative(2, 5).to_string(), "2*x^2+3*x");
    /// assert_eq!(p.mod_nth_derivative(0, 5).to_string(), "x^4+3*x^3+2*x+4");
    ///
    /// // 6 divides 3!.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+x^3").unwrap();
    /// assert_eq!(p.mod_nth_derivative(3, 6).to_string(), "0");
    /// ```
    ///
    /// FLINT has no `nmod_poly_nth_derivative`; this computes the same multipliers as
    /// `fmpz_poly_nth_derivative` from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, directly modulo
    /// `m`.
    #[inline]
    fn mod_nth_derivative(mut self, n: u64, m: T) -> Self {
        mod_nth_derivative_in_place(&mut self, n, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModNthDerivative<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the $n$th derivative of an [`UnsignedPolynomial`] modulo `m`, taking the polynomial
    /// by reference. The coefficients must already be reduced modulo `m`.
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
    /// $T(k, n) = O(kn)$
    ///
    /// $M(k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `self.len()`, and $n$ is `n`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModNthDerivative;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+3*x^3+2*x+4").unwrap();
    /// assert_eq!((&p).mod_nth_derivative(2, 5).to_string(), "2*x^2+3*x");
    /// assert_eq!((&p).mod_nth_derivative(0, 5).to_string(), "x^4+3*x^3+2*x+4");
    ///
    /// // 6 divides 3!.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+x^3").unwrap();
    /// assert_eq!((&p).mod_nth_derivative(3, 6).to_string(), "0");
    /// ```
    ///
    /// FLINT has no `nmod_poly_nth_derivative`; this computes the same multipliers as
    /// `fmpz_poly_nth_derivative` from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, directly modulo
    /// `m`.
    #[inline]
    fn mod_nth_derivative(self, n: u64, m: T) -> UnsignedPolynomial<T> {
        let mut p = self.clone();
        mod_nth_derivative_in_place(&mut p, n, m);
        p
    }
}

impl<T: PrimitiveUnsigned> ModNthDerivativeAssign<T> for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its $n$th derivative modulo `m`, in place. The
    /// coefficients must already be reduced modulo `m`.
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
    /// $T(k, n) = O(kn)$
    ///
    /// $M(k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `self.len()`, and $n$ is `n`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModNthDerivativeAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^4+3*x^3+2*x+4").unwrap();
    /// p.mod_nth_derivative_assign(2, 5);
    /// assert_eq!(p.to_string(), "2*x^2+3*x");
    ///
    /// // 6 divides 3!.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^4+x^3").unwrap();
    /// p.mod_nth_derivative_assign(3, 6);
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// FLINT has no `nmod_poly_nth_derivative`; this computes the same multipliers as
    /// `fmpz_poly_nth_derivative` from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, directly modulo
    /// `m`.
    #[inline]
    fn mod_nth_derivative_assign(&mut self, n: u64, m: T) {
        mod_nth_derivative_in_place(self, n, m);
    }
}
