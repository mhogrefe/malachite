// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2IsReduced;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::polynomial::{ModPowerOf2NthDerivative, ModPowerOf2NthDerivativeAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, pow: u64) {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// Multiplies the coefficient of x^i, for i at least n, by the falling factorial i(i - 1)...(i - n +
// 1) modulo 2^pow and moves it to x^(i - n). Each falling factorial is multiplied out from its n
// factors with wrapping arithmetic, which loses nothing, since 2^pow divides 2^WIDTH. They are all
// multiples of n!, so if 2^pow divides n!, which happens when n - n.count_ones() is at least pow,
// the result is zero. The products can be zero, so the result is trimmed.
fn mod_power_of_2_nth_derivative_in_place<T: PrimitiveUnsigned>(
    p: &mut UnsignedPolynomial<T>,
    n: u64,
    pow: u64,
) {
    assert_reduced(p, pow);
    if n == 0 {
        return;
    }
    if u64::exact_from(p.coefficients.len()) <= n || n - u64::from(n.count_ones()) >= pow {
        p.coefficients.clear();
        return;
    }
    let mut top = T::wrapping_from(n);
    p.coefficients.drain(..usize::exact_from(n));
    for c in &mut p.coefficients {
        let mut falling = T::ONE;
        let mut factor = top;
        for _ in 0..n {
            falling.wrapping_mul_assign(factor);
            factor.wrapping_sub_assign(T::ONE);
        }
        *c = c.wrapping_mul(falling).mod_power_of_2(pow);
        top.wrapping_add_assign(T::ONE);
    }
    p.trim();
}

impl<T: PrimitiveUnsigned> ModPowerOf2NthDerivative for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the $n$th derivative of an [`UnsignedPolynomial`] modulo $2^k$, taking the
    /// polynomial by value. The coefficients must already be reduced modulo $2^k$.
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
    /// $T(k, n) = O(kn)$
    ///
    /// $M(k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `self.len()`, and $n$ is `n`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2NthDerivative;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+3*x^3+2*x+5").unwrap();
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
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+x^4").unwrap();
    /// assert_eq!(p.mod_power_of_2_nth_derivative(4, 3).to_string(), "0");
    /// ```
    ///
    /// FLINT has no `nmod_poly_nth_derivative`; this computes the same multipliers as
    /// `fmpz_poly_nth_derivative` from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, directly modulo
    /// $2^k$.
    #[inline]
    fn mod_power_of_2_nth_derivative(mut self, n: u64, pow: u64) -> Self {
        mod_power_of_2_nth_derivative_in_place(&mut self, n, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2NthDerivative for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the $n$th derivative of an [`UnsignedPolynomial`] modulo $2^k$, taking the
    /// polynomial by reference. The coefficients must already be reduced modulo $2^k$.
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
    /// $T(k, n) = O(kn)$
    ///
    /// $M(k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `self.len()`, and $n$ is `n`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2NthDerivative;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+3*x^3+2*x+5").unwrap();
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
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+x^4").unwrap();
    /// assert_eq!((&p).mod_power_of_2_nth_derivative(4, 3).to_string(), "0");
    /// ```
    ///
    /// FLINT has no `nmod_poly_nth_derivative`; this computes the same multipliers as
    /// `fmpz_poly_nth_derivative` from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, directly modulo
    /// $2^k$.
    #[inline]
    fn mod_power_of_2_nth_derivative(self, n: u64, pow: u64) -> UnsignedPolynomial<T> {
        let mut p = self.clone();
        mod_power_of_2_nth_derivative_in_place(&mut p, n, pow);
        p
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2NthDerivativeAssign for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its $n$th derivative modulo $2^k$, in place. The
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
    /// $T(k, n) = O(kn)$
    ///
    /// $M(k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `self.len()`, and $n$ is `n`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2NthDerivativeAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^4+3*x^3+2*x+5").unwrap();
    /// p.mod_power_of_2_nth_derivative_assign(2, 3);
    /// assert_eq!(p.to_string(), "4*x^2+2*x");
    ///
    /// // 2^3 divides 4!.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+x^4").unwrap();
    /// p.mod_power_of_2_nth_derivative_assign(4, 3);
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// FLINT has no `nmod_poly_nth_derivative`; this computes the same multipliers as
    /// `fmpz_poly_nth_derivative` from `fmpz_poly/nth_derivative.c`, FLINT 3.6.0, directly modulo
    /// $2^k$.
    #[inline]
    fn mod_power_of_2_nth_derivative_assign(&mut self, n: u64, pow: u64) {
        mod_power_of_2_nth_derivative_in_place(self, n, pow);
    }
}
