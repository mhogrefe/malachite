// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2IsReduced;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModPowerOf2Derivative, ModPowerOf2DerivativeAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, pow: u64) {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// Multiplies the coefficient of x^i, for i at least 1, by i, reduces it modulo 2^pow, and moves it
// to x^(i-1). Arithmetic modulo 2^WIDTH, with a wrapping running counter for i, loses nothing,
// since 2^pow divides 2^WIDTH. The products can be zero, so the result is trimmed.
fn mod_power_of_2_derivative_ref<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    assert_reduced(p, pow);
    let mut i = T::ZERO;
    let mut q = UnsignedPolynomial {
        coefficients: p
            .coefficients
            .iter()
            .skip(1)
            .map(|&c| {
                i.wrapping_add_assign(T::ONE);
                c.wrapping_mul(i).mod_power_of_2(pow)
            })
            .collect(),
    };
    q.trim();
    q
}

fn mod_power_of_2_derivative_in_place<T: PrimitiveUnsigned>(
    p: &mut UnsignedPolynomial<T>,
    pow: u64,
) {
    assert_reduced(p, pow);
    if p.coefficients.is_empty() {
        return;
    }
    p.coefficients.remove(0);
    let mut i = T::ZERO;
    for c in &mut p.coefficients {
        i.wrapping_add_assign(T::ONE);
        *c = c.wrapping_mul(i).mod_power_of_2(pow);
    }
    p.trim();
}

impl<T: PrimitiveUnsigned> ModPowerOf2Derivative for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the derivative of an [`UnsignedPolynomial`] modulo $2^k$, taking the polynomial by
    /// value. The coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = p' \bmod 2^k.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$, reduced, and moved to $x^{i-1}$. Since $ia_i$
    /// can be divisible by the modulus even when $a_i$ is not zero, the derivative can lose any
    /// number of degrees. A constant polynomial, including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Derivative;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(p.mod_power_of_2_derivative(2).to_string(), "3*x^2+2*x+2");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+x").unwrap();
    /// assert_eq!(p.mod_power_of_2_derivative(1).to_string(), "x^4+1");
    ///
    /// // The derivative can lose more than one degree.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+x^3+1").unwrap();
    /// assert_eq!(p.mod_power_of_2_derivative(1).to_string(), "x^2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_derivative` from `nmod_poly/derivative.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_derivative(mut self, pow: u64) -> Self {
        mod_power_of_2_derivative_in_place(&mut self, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Derivative for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the derivative of an [`UnsignedPolynomial`] modulo $2^k$, taking the polynomial by
    /// reference. The coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = p' \bmod 2^k.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$, reduced, and moved to $x^{i-1}$. Since $ia_i$
    /// can be divisible by the modulus even when $a_i$ is not zero, the derivative can lose any
    /// number of degrees. A constant polynomial, including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2Derivative;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!((&p).mod_power_of_2_derivative(2).to_string(), "3*x^2+2*x+2");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+x").unwrap();
    /// assert_eq!((&p).mod_power_of_2_derivative(1).to_string(), "x^4+1");
    ///
    /// // The derivative can lose more than one degree.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+x^3+1").unwrap();
    /// assert_eq!((&p).mod_power_of_2_derivative(1).to_string(), "x^2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_derivative` from `nmod_poly/derivative.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_derivative(self, pow: u64) -> UnsignedPolynomial<T> {
        mod_power_of_2_derivative_ref(self, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DerivativeAssign for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its derivative modulo $2^k$, in place. The
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets p' \bmod 2^k.
    /// $$
    ///
    /// The coefficient of $x^i$ is multiplied by $i$, reduced, and moved to $x^{i-1}$. Since $ia_i$
    /// can be divisible by the modulus even when $a_i$ is not zero, the derivative can lose any
    /// number of degrees. A constant polynomial, including zero, has derivative zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DerivativeAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+2*x+1").unwrap();
    /// p.mod_power_of_2_derivative_assign(2);
    /// assert_eq!(p.to_string(), "3*x^2+2*x+2");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+x").unwrap();
    /// p.mod_power_of_2_derivative_assign(1);
    /// assert_eq!(p.to_string(), "x^4+1");
    ///
    /// // The derivative can lose more than one degree.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^4+x^3+1").unwrap();
    /// p.mod_power_of_2_derivative_assign(1);
    /// assert_eq!(p.to_string(), "x^2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_derivative` from `nmod_poly/derivative.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_derivative_assign(&mut self, pow: u64) {
        mod_power_of_2_derivative_in_place(self, pow);
    }
}
