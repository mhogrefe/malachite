// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2IsReduced;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::EvaluateModPowerOf2;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Evaluates a polynomial at x modulo 2^pow, after checking that pow fits in `T` and that the
// coefficients and x are reduced.
fn evaluate_mod_power_of_2<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, x: T, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    let mut value = T::ZERO;
    for &c in p.coefficients.iter().rev() {
        value = value.wrapping_mul(x).wrapping_add(c);
    }
    value.mod_power_of_2(pow)
}

impl<T: PrimitiveUnsigned> EvaluateModPowerOf2<T> for &UnsignedPolynomial<T> {
    type Output = T;

    /// Evaluates an [`UnsignedPolynomial`] at a value of its coefficient type, modulo $2^k$, taking
    /// the polynomial by reference. The coefficients and the value must already be reduced modulo
    /// $2^k$, and $k$ may be at most the width of the type.
    ///
    /// $$
    /// f(p, x, k) = \sum_{i=0}^{n-1} c_i x^i \bmod 2^k,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Reducing modulo $2^k$ commutes with wrapping arithmetic, which works modulo $2^w$ for the
    /// type's width $w \geq k$, so Horner's rule is carried out with wrapping multiplications and
    /// additions and the value is reduced once, at the end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `x` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::EvaluateModPowerOf2;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 13 mod 16.
    /// assert_eq!((&p).evaluate_mod_power_of_2(6, 4), 13);
    /// assert_eq!((&p).evaluate_mod_power_of_2(0, 4), 7);
    /// // All 8 bits of a u8: 205 itself.
    /// assert_eq!((&p).evaluate_mod_power_of_2(6, 8), 205);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_nmod` from `nmod_poly/evaluate_nmod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that the value must be reduced.
    #[inline]
    fn evaluate_mod_power_of_2(self, x: T, pow: u64) -> T {
        evaluate_mod_power_of_2(self, x, pow)
    }
}

impl<T: PrimitiveUnsigned> EvaluateModPowerOf2<T> for UnsignedPolynomial<T> {
    type Output = T;

    /// Evaluates an [`UnsignedPolynomial`] at a value of its coefficient type, modulo $2^k$, taking
    /// the polynomial by value. The coefficients and the value must already be reduced modulo
    /// $2^k$, and $k$ may be at most the width of the type.
    ///
    /// $$
    /// f(p, x, k) = \sum_{i=0}^{n-1} c_i x^i \bmod 2^k,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length. The zero polynomial
    /// evaluates to 0 everywhere.
    ///
    /// Reducing modulo $2^k$ commutes with wrapping arithmetic, which works modulo $2^w$ for the
    /// type's width $w \geq k$, so Horner's rule is carried out with wrapping multiplications and
    /// additions and the value is reduced once, at the end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `x` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::EvaluateModPowerOf2;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 5 * 36 + 3 * 6 + 7 = 205, which is 13 mod 16.
    /// assert_eq!(p.clone().evaluate_mod_power_of_2(6, 4), 13);
    /// assert_eq!(p.clone().evaluate_mod_power_of_2(0, 4), 7);
    /// // All 8 bits of a u8: 205 itself.
    /// assert_eq!(p.evaluate_mod_power_of_2(6, 8), 205);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_nmod` from `nmod_poly/evaluate_nmod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that the value must be reduced.
    #[inline]
    fn evaluate_mod_power_of_2(self, x: T, pow: u64) -> T {
        evaluate_mod_power_of_2(&self, x, pow)
    }
}
